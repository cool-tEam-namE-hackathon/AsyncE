use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse,
    },
};
use serde::Deserialize;

use crate::chunk;

lazy_static::lazy_static! {
    pub static ref HTTP_OK: candid::Nat = candid::Nat::from(200u128);
}

#[derive(Deserialize)]
struct ChunkInfoResponse {
    chunk_count: usize,
    file_size: usize,
}

pub fn map_response_body_to_err<T>(response: HttpResponse) -> Result<T, String> {
    let unk = format!("Unknown error: {:?}", response.body);
    let body_str = String::from_utf8(response.body).unwrap_or(unk);
    let body_str = format!("HTTP calls error: {}", body_str);

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
    let uuid_response = send_post_request("http://localhost:5555/subtitles/start", Vec::new())
        .await
        .map_err(|(code, body)| {
            format!(
                "Failed to send HTTP request for processing subtitle.start ({:?}: {})",
                code, body
            )
        })?;

    if uuid_response.status != *HTTP_OK {
        return map_response_body_to_err(uuid_response);
    }

    let uuid = String::from_utf8(uuid_response.body).map_err(|_| {
        String::from("Cannot convert bytes to uuid while processing subtitle.start")
    })?;

    let chunks = body.chunks(chunk::MB).collect::<Vec<_>>();
    let chunk_len = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let url = if i == chunk_len - 1 {
            format!("http://localhost:5555/subtitles/{}/end", uuid)
        } else {
            format!("http://localhost:5555/subtitles/{}/add", uuid)
        };

        let response = send_post_request(url, chunk.to_vec())
            .await
            .map_err(|_| String::from("Failed to send HTTP request for processing subtitle.add"))?;
        if response.status != *HTTP_OK {
            return map_response_body_to_err(response);
        }
    }

    Ok(uuid)
}

pub async fn send_thumbnail_request(body: Vec<u8>) -> Result<Vec<u8>, String> {
    let response = send_post_request("http://localhost:5555/thumbnail", body)
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing thumbnail"))?;
    if response.status != *HTTP_OK {
        return map_response_body_to_err(response);
    }

    return Ok(response.body);
}

pub async fn send_concat_video_request(video1: Vec<u8>, video2: Vec<u8>) -> Result<(), String> {
    let uuid_response1 = send_post_request("http://localhost:5555/concat/start", Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing concat"))?;
    if uuid_response1.status != *HTTP_OK {
        return map_response_body_to_err(uuid_response1);
    }

    let uuid_response2 = send_post_request("http://localhost:5555/concat/start", Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing concat"))?;
    if uuid_response2.status != *HTTP_OK {
        return map_response_body_to_err(uuid_response2);
    }

    let uuid1 = String::from_utf8(uuid_response1.body)
        .map_err(|_| String::from("Cannot convert bytes to uuid"))?;

    let uuid2 = String::from_utf8(uuid_response2.body)
        .map_err(|_| String::from("Cannot convert bytes to uuid"))?;

    for (video, uuid) in [(video1, &uuid1), (video2, &uuid2)] {
        let chunks = video.chunks(chunk::MB).collect::<Vec<_>>();
        let chunk_len = chunks.len();
        for (i, chunk) in chunks.into_iter().enumerate() {
            let url = if i == chunk_len - 1 {
                format!("http://localhost:5555/concat/{}/end", uuid)
            } else {
                format!("http://localhost:5555/concat/{}/add", uuid)
            };

            let response = send_post_request(url, chunk.to_vec()).await.map_err(|_| {
                String::from("Failed to send HTTP request for processing concat.add")
            })?;
            if response.status != *HTTP_OK {
                return map_response_body_to_err(response);
            }
        }
    }

    let url = format!("http://localhost:5555/concat/{}/{}", uuid1, uuid2);
    let response = send_post_request(url, Vec::new())
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing concat.add"))?;
    if response.status != *HTTP_OK {
        return map_response_body_to_err(response);
    }

    Ok(())
}

pub async fn get_processed_video_subtitles(uuid: &str) -> Result<Option<Vec<u8>>, String> {
    let url = format!("http://localhost:5555/subtitles/{}", uuid);
    let uuid_response = send_get_request(url).await.map_err(|_| {
        String::from("Failed to send HTTP request for getting processed video subtitles")
    })?;
    if uuid_response.status != *HTTP_OK {
        return Ok(None);
    }

    let response = serde_json::from_slice::<ChunkInfoResponse>(&uuid_response.body)
        .map_err(|err| format!("Deserialize to chunk info from json error: {}", err))?;

    let mut data = Vec::with_capacity(response.file_size);
    for i in 0..response.chunk_count {
        let url = format!("http://localhost:5555/subtitles/{}/{}", uuid, i);
        let response = send_get_request(url).await.map_err(|_| {
            String::from("Failed to send HTTP request for getting processed video subtitles chunk")
        })?;

        if response.status != *HTTP_OK {
            return Ok(None);
        }

        data.extend(response.body);
    }

    Ok(Some(data))
}
