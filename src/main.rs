mod download;
mod err;
use download::Downloader;
use err::Result;
use std::io::BufRead;
use std::ops::Deref;

#[tokio::main]
async fn main() -> Result<()> {
    //read -> parse urls -> map to buffers -> save -> update console ui
    // let path = "~/Desktop/links.txt";
    // let files_url = read_file_content(path).map(|content| file_content_to_links(content))?;

    let download = Downloader::new();
    let link = "https://c.neh.tw/thumb/f/720/comhiclipartdatah.jpg".to_string();
    let result = download.download_single_file(&link).await.unwrap();
    std::fs::write("./img.png", result);
    Ok(())
}

fn read_file_content(path: &str) -> Result<String> {
    let file_content = std::fs::read_to_string(path)?;
    Ok(file_content)
}

fn file_content_to_links(content: String) -> Vec<String> {
    let splits = content.split("\r\n");

    splits.map(|str| str.to_string()).collect()
}
