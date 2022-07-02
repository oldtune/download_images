use std::future::Future;

pub use crate::err::Result;
use bytes::Bytes;
// use bytes::Bytes
use reqwest::Request;
//https://i.imgur.com/tvMbFww.jpeg
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
        // let futures: Vec<dyn Future<Output = Result<Bytes>>> = links
        //     .iter()
        //     .map(|link| self.download_single_file(link))
        //     .collect();
        todo!()
    }

    pub async fn download_single_file(&self, link: &String) -> Result<Bytes> {
        // let http_result = self.http_client.get(link).send().await?;
        // let bytes = http_result.bytes().await?;
        // Ok(bytes)
        todo!()
    }
}
