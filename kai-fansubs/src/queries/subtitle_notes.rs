use crate::client::Client;
use crate::error::Error;
use crate::parsing::selectors::NOTE_DIV_SELECTOR;
use encoding_rs::WINDOWS_1251;
use reqwest::Method;
use scraper::Html;

/// Query for fetching additional notes associated with a subtitle.
///
/// This query retrieves optional additional information, comments, or notes
/// that may be associated with a specific subtitle entry. Not all subtitles
/// have notes, so this query may return `None` for subtitles without additional information.
///
/// # Examples
///
/// ```rust
/// use kai_fansubs::client::Client;
/// use kai_fansubs::queries::SubtitleNoteQuery;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let query = SubtitleNoteQuery::new(3645);
///
/// match query.execute(&client).await? {
///     Some(notes) => {
///         println!("Subtitle notes:");
///         println!("{}", notes);
///     }
///     None => {
///         println!("No notes available for this subtitle");
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SubtitleNoteQuery {
    subtitle_id: u32,
}

impl SubtitleNoteQuery {
    /// Creates a new subtitle note query for the specified subtitle ID.
    pub fn new(subtitle_id: u32) -> Self {
        Self { subtitle_id }
    }

    /// Executes the query and returns the subtitle notes if available.
    pub async fn execute(&self, client: &Client) -> Result<Option<String>, Error> {
        let request = client
            .init_request(Method::GET, &format!("/base.php?note={}", self.subtitle_id))
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response =
            client.send(request).await.map_err(|e| Error::ServiceError(e.to_string()))?;
        let bytes = response.bytes().await?;
        let (html, _, _) = WINDOWS_1251.decode(&bytes);

        let document = Html::parse_document(&html);

        if let Some(div) = document.select(&NOTE_DIV_SELECTOR).next() {
            let content = div.inner_html();
            let cleaned = content
                .replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<br />", "\n")
                .replace("&nbsp;", " ")
                .replace("&quot;", "\"")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&amp;", "&");

            let result = cleaned
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            if !result.is_empty() {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }
}
