use crate::err::Result;

use bytes::Bytes;
pub struct FileWriter {}

impl FileWriter {
    pub async fn write_file(&self, path: &str, bytes: Bytes) -> Result<()> {
        tokio::fs::write(path, bytes).await?;
        Ok(())
    }
}
