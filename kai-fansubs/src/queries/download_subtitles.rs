use crate::client::Client;
use crate::error::Error;
use encoding_rs::WINDOWS_1251;
use reqwest::Method;

/// Query for downloading subtitle files.
///
/// This query allows you to download subtitle files in various formats
/// (ZIP, ASS, SRT, etc.) by their unique subtitle ID. The downloaded content
/// can be retrieved as raw bytes, decoded string, or saved directly to a file.
///
/// # Examples
///
/// ```rust
/// use kai_fansubs::client::Client;
/// use kai_fansubs::queries::DownloadSubtitleQuery;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let query = DownloadSubtitleQuery::new(3645);
///
/// // Download as string
/// let subtitle_content = query.execute_as_string(&client).await?;
/// println!("Subtitle content:\n{}", subtitle_content);
///
/// // Download as bytes for binary processing
/// let subtitle_bytes = query.execute(&client).await?;
/// println!("Downloaded {} bytes", subtitle_bytes.len());
///
/// // Save directly to file
/// query.download_to_file(&client, "subtitle.ass").await?;
/// println!("Subtitle saved to file");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct DownloadSubtitleQuery {
    subtitle_id: u32,
}

impl DownloadSubtitleQuery {
    /// Creates a new download subtitle query for the specified subtitle ID.
    pub fn new(subtitle_id: u32) -> Self {
        Self { subtitle_id }
    }

    /// Downloads the subtitle file as raw bytes.
    pub async fn execute(&self, client: &Client) -> Result<Vec<u8>, Error> {
        let body = format!("srt={}&x=0&y=0", self.subtitle_id);

        let request = client
            .init_request(Method::POST, "/base.php")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client
            .send(request)
            .await
            .map_err(|e| Error::ServiceError(e.to_string()))?;
        let bytes = response.bytes().await?;

        Ok(bytes.to_vec())
    }

    /// Downloads the subtitle file and returns it as a decoded string.
    ///
    /// This method automatically detects the encoding:
    /// - If the file is a ZIP archive, returns an error (use `execute()` for binary data)
    /// - If the file starts with UTF-8 BOM, decodes as UTF-8
    /// - Otherwise, decodes as Windows-1251 (default encoding for the site)
    pub async fn execute_as_string(&self, client: &Client) -> Result<String, Error> {
        let bytes = self.execute(client).await?;

        // Check for ZIP magic bytes (PK\x03\x04)
        if bytes.len() >= 4 && &bytes[0..4] == b"PK\x03\x04" {
            return Err(Error::EncodingError(
                "File is a ZIP archive. Use execute() to get raw bytes instead.".to_string(),
            ));
        }

        // Check for UTF-8 BOM (EF BB BF)
        if bytes.len() >= 3 && &bytes[0..3] == b"\xEF\xBB\xBF" {
            // Remove BOM and decode as UTF-8
            let text = std::str::from_utf8(&bytes[3..])?;
            return Ok(text.to_string());
        }

        // Check if it's valid UTF-8 without BOM
        if let Ok(text) = std::str::from_utf8(&bytes) {
            return Ok(text.to_string());
        }

        // Fall back to Windows-1251 decoding
        let (text, _, _) = WINDOWS_1251.decode(&bytes);
        Ok(text.to_string())
    }

    /// Downloads the subtitle file and saves it directly to the filesystem.
    pub async fn download_to_file(&self, client: &Client, path: &str) -> Result<(), Error> {
        let bytes = self.execute(client).await?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
}
