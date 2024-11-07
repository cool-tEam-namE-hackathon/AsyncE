use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
    },
};
use serde::Deserialize;

use crate::{chunk, globals::MEETINGS, meeting::MeetingProcessType};

#[derive(Debug, Clone)]
pub struct ConcatRequest {
    group_id: u128,
    meeting_id: u128,
    uuid: String,
}

lazy_static::lazy_static! {
    pub static ref HTTP_OK: candid::Nat = candid::Nat::from(200u128);
}

thread_local! {
    pub static CONCAT_REQUESTS: Arc<Mutex<Vec<ConcatRequest>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn poll_concat_requests() {
    ic_cdk::println!("Starting poll concat requests");
    ic_cdk_timers::set_timer_interval(Duration::from_secs(10), || {
        ic_cdk::spawn(async move {
            let concat_requests =
                CONCAT_REQUESTS.with(|concat_requests| concat_requests.lock().unwrap().clone());
            if concat_requests.is_empty() {
                return;
            }

            let mut indexes_to_remove = Vec::new();

            ic_cdk::println!("AH HA! {:?}", concat_requests);

            for req in concat_requests {
                let processed_video_data = match get_processed_video_concat(&req.uuid).await {
                    Ok(processed_video_data) => processed_video_data,
                    Err(err) => {
                        ic_cdk::eprintln!("Error while getting processed video concat: {}", err);
                        continue;
                    }
                };

                let processed_video_data = match processed_video_data {
                    Some(processed_video_data) => processed_video_data,
                    None => continue,
                };

                let mut meetings = MEETINGS.lock().unwrap();
                let meetings = meetings
                    .get_mut(&req.group_id)
                    .ok_or(String::from("No meetings found on this group!"))
                    .unwrap();

                let meeting = meetings
                    .get_mut(&req.meeting_id)
                    .ok_or(String::from("No meeting found on this meeting ID!"))
                    .unwrap();

                meeting.process_type = MeetingProcessType::None;
                meeting.full_video_data = processed_video_data;
                indexes_to_remove.push(req.uuid);
            }

            for uuid in indexes_to_remove {
                let concat_requests =
                    CONCAT_REQUESTS.with(|concat_requests| concat_requests.clone());
                let mut concat_requests = concat_requests.lock().unwrap();
                let index_to_remove = concat_requests.iter().position(|x| x.uuid == uuid).unwrap();
                concat_requests.remove(index_to_remove);
            }
        });
    });
}

#[derive(Deserialize, Debug)]
struct ChunkInfoResponse {
    chunk_count: usize,
    file_size: usize,
}

pub fn map_response_body_to_err<T>(url: &str, response: HttpResponse) -> Result<T, String> {
    let unk = format!("Unknown error: {:?}", response.body);
    let body_str = String::from_utf8(response.body).unwrap_or(unk);
    let body_str = format!("HTTP calls error: {} {} {}", url, response.status, body_str);

    Err(body_str)
}

pub async fn send_http_request(
    url: impl Into<String>,
    body: Vec<u8>,
    method: HttpMethod,
) -> Result<HttpResponse, (RejectionCode, String)> {
    let request = CanisterHttpRequestArgument {
        url: url.into(),
        body: Some(body),
        method,
        headers: Vec::new(),
        transform: None,
        max_response_bytes: None,
    };

    let response = http_request(request, 1_000_000_000_000).await?.0;
    Ok(response)
}

pub async fn send_get_request(
    url: impl Into<String>,
) -> Result<HttpResponse, (RejectionCode, String)> {
    send_http_request(url, Vec::new(), HttpMethod::GET).await
}

pub async fn send_post_request(
    url: impl Into<String>,
    body: Vec<u8>,
) -> Result<HttpResponse, (RejectionCode, String)> {
    send_http_request(url, body, HttpMethod::POST).await
}

pub async fn send_process_subtitles_request(body: Vec<u8>) -> Result<String, String> {
    let uuid_response = send_post_request("http://localhost:17191/subtitles/start", Vec::new())
        .await
        .map_err(|(code, body)| {
            format!(
                "Failed to send HTTP request for processing subtitle.start ({:?}: {})",
                code, body
            )
        })?;

    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err("subtitles.start", uuid_response);
    }

    let uuid = String::from_utf8(uuid_response.body).map_err(|_| {
        String::from("Cannot convert bytes to uuid while processing subtitle.start")
    })?;

    let chunks = body.chunks(chunk::MB).collect::<Vec<_>>();
    let chunk_len = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let url = if i == chunk_len - 1 {
            format!("http://localhost:17191/subtitles/{}/end", uuid)
        } else {
            format!("http://localhost:17191/subtitles/{}/add", uuid)
        };

        let response = send_post_request(&url, chunk.to_vec())
            .await
            .map_err(|_| String::from("Failed to send HTTP request for processing subtitle.add"))?;
        if response.status != *HTTP_OK {
            return map_response_body_to_err(&url, response);
        }
    }

    Ok(uuid)
}

pub async fn send_thumbnail_request(body: Vec<u8>) -> Result<Vec<u8>, String> {
    let uuid_response = send_post_request("http://localhost:17191/thumbnail/start", Vec::new())
        .await
        .map_err(|err| {
            format!(
                "Failed to send HTTP request for processing thumbnail.start {:?} {}",
                err.0, err.1
            )
        })?;
    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err("thumbnail.start", uuid_response);
    }

    let uuid = String::from_utf8(uuid_response.body)
        .map_err(|_| String::from("Cannot convert bytes to uuid"))?;

    let chunks = body.chunks(chunk::MB).collect::<Vec<_>>();
    for chunk in chunks.into_iter() {
        let url = format!("http://localhost:17191/thumbnail/{}/add", uuid);

        let response = send_post_request(url, chunk.to_vec()).await.map_err(|_| {
            String::from("Failed to send HTTP request for processing thumbnail.add")
        })?;
        if response.status != *HTTP_OK {
            return map_response_body_to_err("thumbnail.add", response);
        }
    }

    let url = format!("http://localhost:17191/thumbnail/{}/end", uuid);
    let response = send_post_request(url, Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing thumbnail.end"))?;
    if response.status != *HTTP_OK {
        return map_response_body_to_err("thumbnail.end", response);
    }

    Ok(response.body)
}

pub async fn send_concat_video_request(
    group_id: u128,
    meeting_id: u128,
    video1: Vec<u8>,
    video2: Vec<u8>,
) -> Result<(), String> {
    let uuid_response = send_post_request("http://localhost:17191/concat/start", Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing concat"))?;
    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err("concat.start", uuid_response);
    }

    let uuid = String::from_utf8(uuid_response.body)
        .map_err(|_| String::from("Cannot convert bytes to uuid"))?;

    let chunks = video1.chunks(chunk::MB);
    for chunk in chunks.into_iter() {
        let url = format!("http://localhost:17191/concat/{}/add", uuid);

        let response = send_post_request(url, chunk.to_vec())
            .await
            .map_err(|_| String::from("Failed to send HTTP request for processing concat.add"))?;
        if response.status != *HTTP_OK {
            return map_response_body_to_err("concat.add", response);
        }
    }

    let chunks = video2.chunks(chunk::MB);
    for (i, chunk) in chunks.into_iter().enumerate() {
        let url = if i == 0 {
            format!("http://localhost:17191/concat/{}/new", uuid)
        } else {
            format!("http://localhost:17191/concat/{}/add", uuid)
        };

        let response = send_post_request(&url, chunk.to_vec())
            .await
            .map_err(|_| String::from("Failed to send HTTP request for processing concat.add"))?;
        if response.status != *HTTP_OK {
            return map_response_body_to_err(&url, response);
        }
    }

    let url = format!("http://localhost:17191/concat/{}/end", uuid);
    let response = send_post_request(&url, Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing concat.end"))?;
    if response.status != *HTTP_OK {
        return map_response_body_to_err(&url, response);
    }

    let uuid = String::from_utf8(response.body)
        .map_err(|_| String::from("Cannot convert bytes to uuid"))?;

    CONCAT_REQUESTS.with(|concat_requests| {
        concat_requests.lock().unwrap().push(ConcatRequest {
            group_id,
            meeting_id,
            uuid,
        });
    });

    Ok(())
}

pub async fn get_processed_video_subtitles(uuid: &str) -> Result<Option<Vec<u8>>, String> {
    let url = format!("http://localhost:17191/subtitles/{}", uuid);
    let uuid_response = send_get_request(url).await.map_err(|_| {
        String::from("Failed to send HTTP request for getting processed video subtitles")
    })?;
    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err("subtitles.get", uuid_response);
    }

    let response = serde_json::from_slice::<ChunkInfoResponse>(&uuid_response.body)
        .map_err(|err| format!("Deserialize to chunk info from json error: {}", err))?;

    let mut data = Vec::with_capacity(response.file_size);
    for i in 0..response.chunk_count {
        let url = format!("http://localhost:17191/subtitles/{}/{}", uuid, i);
        let response = send_get_request(url).await.map_err(|_| {
            String::from("Failed to send HTTP request for getting processed video subtitles chunk")
        })?;

        if response.status != *HTTP_OK {
            return map_response_body_to_err("subtitles.get.i", response);
        }

        data.extend(response.body);
    }

    Ok(Some(data))
}

pub async fn get_processed_video_concat(uuid: &str) -> Result<Option<Vec<u8>>, String> {
    let url = format!("http://localhost:17191/concat/{}", uuid);
    let uuid_response = send_get_request(url).await.map_err(|_| {
        String::from("Failed to send HTTP request for getting processed video concat")
    })?;
    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err("concat.get", uuid_response);
    }

    let response = serde_json::from_slice::<ChunkInfoResponse>(&uuid_response.body)
        .map_err(|err| format!("Deserialize to chunk info from json error: {}", err))?;

    ic_cdk::println!("concat/id {:?} {:?}", uuid_response.body, response);

    let mut data = Vec::with_capacity(response.file_size);
    for i in 0..response.chunk_count {
        let url = format!("http://localhost:17191/concat/{}/{}", uuid, i + 1);
        let response = send_get_request(url).await.map_err(|_| {
            String::from("Failed to send HTTP request for getting processed video concat chunk")
        })?;

        if response.status != *HTTP_OK {
            return map_response_body_to_err("concat.get.i", response);
        }

        data.extend(response.body);
    }

    Ok(Some(data))
}
