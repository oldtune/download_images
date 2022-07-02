mod download;
mod err;
use download::Downloader;
use err::Result;
use file_reader::FileReader;
use file_writer::FileWriter;
use log::info;
mod file_reader;
mod file_writer;
mod logging;

static LOGGER: logging::Logger = logging::Logger {};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;

    let file_reader = FileReader::new("");
    let links = file_reader.read().await?;

    info!("found {:?}", &links);

    let downloader = Downloader::new();
    let files = downloader.download_multiple_file(&links).await?;

    info!("downloaded {} files", &files.len());

    let file_writer = FileWriter {};

    for file in files {
        file_writer.write_file("", file).await?;
    }

    Ok(())
}

fn init_logger() -> Result<()> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info))?;
    Ok(())
}
