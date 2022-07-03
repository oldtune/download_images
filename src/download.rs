use std::future::Future;

pub use crate::err::Result;
use bytes::Bytes;
// use bytes::Bytes
use reqwest::Request;
use tokio::task::futures;
pub struct Downloader {
    http_client: reqwest::Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn download_multiple_file(&self, links: &Vec<String>) -> Result<Vec<Bytes>> {
        let mut handles = Vec::with_capacity(links.len());

        for link in links {
            let future = self.download_single_file(link);
            handles.push(tokio::spawn(future));
        }

        let mut results = Vec::with_capacity(links.len());
        for handle in handles {
            results.push(handle.await.unwrap()?);
        }

        Ok(results)
    }

    pub async fn download_single_file(&self, link: &String) -> Result<Bytes> {
        let http_result = self.http_client.get(link).send().await?;
        let bytes = http_result.bytes().await?;
        Ok(bytes)
    }
}
