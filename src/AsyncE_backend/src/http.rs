use ic_cdk::api::{
    call::RejectionCode,
    management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod},
};

pub async fn send_http_request(
    url: &str,
    body: Vec<u8>,
    method: HttpMethod,
) -> Result<Vec<u8>, (RejectionCode, String)> {
    let request = CanisterHttpRequestArgument {
        url: String::from(url),
        body: Some(body),
        method,
        headers: Vec::new(),
        transform: None,
        max_response_bytes: None,
    };

    Ok(http_request(request, 0).await?.0.body)
}

pub async fn send_get_request(url: &str) -> Result<Vec<u8>, (RejectionCode, String)> {
    send_http_request(url, Vec::new(), HttpMethod::GET).await
}

pub async fn send_post_request(
    url: &str,
    body: Vec<u8>,
) -> Result<Vec<u8>, (RejectionCode, String)> {
    send_http_request(url, body, HttpMethod::POST).await
}

pub async fn send_process_subtitles_request(body: Vec<u8>) -> Result<Vec<u8>, String> {
    send_post_request("http://localhost:5555/ subtitles", body)
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing subtitle"))
}

pub async fn send_thumbnail_request(body: Vec<u8>) -> Result<Vec<u8>, String> {
    send_post_request("http://localhost:5555/thumbnail", body)
        .await
        .map_err(|_| String::from("Failed to send HTTP request for processing thumbnail"))
}

// pub async fn send_concat_video_request(
//     video1: Vec<u8>,
//     video2: Vec<u8>,
// ) -> Result<Vec<u8>, String> {
//     send_post_request("http://localhost:5555/subtitles", video1, video2)
//         .await
//         .map_err(|_| String::from("Failed to send HTTP request for processing concat"))
// }
