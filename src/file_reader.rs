use crate::err::Result;

pub struct FileReader<'a> {
    file_path: &'a str,
}

impl<'a> FileReader<'a> {
    pub fn new(link: &str) -> FileReader {
        FileReader { file_path: link }
    }

    pub async fn read(&self) -> Result<Vec<String>> {
        let file_content = self.read_file_content().await?;
        Ok(self.content_to_links(&file_content))
    }

    async fn read_file_content(&self) -> Result<String> {
        let content = tokio::fs::read_to_string(&self.file_path).await?;
        Ok(content)
    }

    fn content_to_links(&self, content: &String) -> Vec<String> {
        let splits = content.split("\r\n");
        splits.map(|str| str.to_string()).collect()
    }
}
