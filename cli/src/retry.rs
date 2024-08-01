use std::time::Duration;

use anyhow::bail;
use lazy_static::lazy_static;
use reqwest::{IntoUrl, Response};

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn get_retry<U: IntoUrl>(url: U) -> anyhow::Result<Response> {
    let url = url.into_url()?;

    let max_retries = 5;
    let max_interval = Duration::from_secs(15);
    let mut interval = Duration::from_millis(500);

    for retry in 1..=max_retries {
        match CLIENT.get(url.clone()).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    return Ok(resp);
                }
                let err_msg = resp.text().await.unwrap_or_default();
                eprintln!("Failed to GET {url}: {err_msg}. Attempt no. {retry}");
            }
            Err(err) => {
                eprintln!("Network error on GET {url}: {err}. Attempt no. {retry}");
            }
        }

        let jitter = Duration::from_millis(fastrand::u64(0..1500));
        interval = std::cmp::min(interval * 2, max_interval); // Double the interval for exponential backoff

        tokio::time::sleep(interval + jitter).await;
    }

    bail!("Failed to GET {url} after {max_retries} retries.");
}
