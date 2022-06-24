use reqwest::Request;

pub use crate::err::Result;

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
        let result: Vec<Result<()>> = links.iter().map(|f| self.download_single_file(f)).collect();
        Ok(())
    }

    async fn download_single_file(&self, link: &String) -> Result<()> {
        self.http_client.get(link).send().await?;
        Ok(())
    }
}
