mod download;
mod err;
use download::Downloader;
use err::Result;
use file_reader::FileReader;
use file_writer::FileWriter;
use std::io::BufRead;
use std::ops::Deref;
mod file_reader;
mod file_writer;

#[tokio::main]
async fn main() -> Result<()> {
    let file_reader = FileReader::new("");
    let links = file_reader.read().await?;

    let downloader = Downloader::new();
    let files = downloader.download_multiple_file(&links).await?;

    let file_writer = FileWriter {};

    for file in files {
        file_writer.write_file("", file).await?;
    }

    Ok(())
}
