use std::time::Duration;

use crate::config::DEADLINE_SECS;

pub fn client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(DEADLINE_SECS))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
}

pub async fn bounded_body(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, &'static str> {
    if response.content_length().is_some_and(|n| n > limit as u64) {
        return Err("response_size_limit");
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| "response_read_failed")? {
        if chunk.len() > limit.saturating_sub(body.len()) {
            return Err("response_size_limit");
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
