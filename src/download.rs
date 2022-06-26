pub use crate::err::Result;
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

    pub async fn download_multiple_file(&self, links: &Vec<String>) -> Result<()> {
        // let result: Vec<Box<dyn Future<Output = Result<()>>>> = links
        //     .iter()
        //     .map(|link| Box::new(self.download_single_file(link)) as Box<dyn Future<Output = _>>)
        //     .collect();
        Ok(())
    }

    pub async fn download_single_file(&self, link: &String) -> Result<Vec<u8>> {
        let httpResult = self.http_client.get(link).send().await?;
        let bytes = httpResult.bytes().await?;
        let vec_bytes = bytes.to_vec();
        Ok(vec_bytes)
    }
}
