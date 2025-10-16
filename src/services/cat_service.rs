use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const CATFACT_URL: &str = "https://catfact.ninja/fact";
const TIMEOUT_SECS: u64 = 5;

#[derive(Debug, Deserialize)]
struct CatFactDto {
    fact: String,
}

/// Errors only for the Cat Facts upstream.
/// We keep upstream status as `u16` to avoid mixing different `http::StatusCode` types
/// between `reqwest` (http 0.2) and `axum` (http 1.x).
#[derive(Debug)]
pub enum CatApiError {
    Timeout,
    Network,
    Status(u16),
    InvalidJson,
    Unknown,
}

impl From<reqwest::Error> for CatApiError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            CatApiError::Timeout
        } else if e.is_connect() {
            CatApiError::Network
        } else if let Some(s) = e.status() {
            CatApiError::Status(s.as_u16())
        } else {
            CatApiError::Unknown
        }
    }
}

/// Fetch a cat fact from the upstream service with proper timeout & error classification.
pub async fn get_cat_fact_strict(client: &Client) -> Result<String, CatApiError> {
    let res = client
        .get(CATFACT_URL)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(CatApiError::from)?;

    if !res.status().is_success() {
        return Err(CatApiError::Status(res.status().as_u16()));
    }

    let dto = res
        .json::<CatFactDto>()
        .await
        .map_err(|_| CatApiError::InvalidJson)?;

    Ok(dto.fact)
}
