use crate::client::Client;
use crate::error::Error;
use crate::parsing::media_parsing::{parse_information_block, parse_subtitles};
use crate::types::MediaDetails;
use encoding_rs::WINDOWS_1251;
use reqwest::Method;
use scraper::Html;

/// Query for fetching detailed information about a specific media item.
///
/// This query retrieves comprehensive media information including title,
/// alternative names, general information, poster image, external links,
/// and all available subtitles with their metadata.
///
/// # Examples
///
/// ```rust
/// use kai_fansubs::client::Client;
/// use kai_fansubs::queries::MediaQuery;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
/// let media_details = MediaQuery::new(4214).execute(&client).await?;
///
/// println!("Title: {}", media_details.main_title);
/// println!("Alternative names: {:?}", media_details.alternative_names);
/// println!("Available subtitles: {}", media_details.subtitles.len());
///
/// // Access subtitle information
/// for subtitle in &media_details.subtitles {
///     println!("Subtitle: {} ({})", subtitle.title, subtitle.format);
///     for author in &subtitle.authors {
///         println!(
///             "  Author: {} - {}",
///             author.name,
///             author.role.as_deref().unwrap_or("Unknown role")
///         );
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MediaQuery {
    id: u32,
}

impl MediaQuery {
    /// Creates a new media query for the specified media ID.
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    /// Executes the query and returns detailed media information.
    pub async fn execute(&self, client: &Client) -> Result<MediaDetails, Error> {
        let request = client
            .init_request(Method::GET, &format!("/base.php?id={}", self.id))
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client
            .send(request)
            .await
            .map_err(|e| Error::ServiceError(e.to_string()))?;
        let bytes = response.bytes().await?;

        let (html, _, _) = WINDOWS_1251.decode(&bytes);
        let html_str = html.to_string();
        let document = Html::parse_document(&html);

        let information = parse_information_block(&document, &client.site_url)?;

        Ok(MediaDetails {
            id: self.id,
            main_title: information.main_title,
            alternative_names: information.alternative_names,
            general_info: information.general_info,
            poster_url: information.poster_url,
            links: information.links,
            subtitles: parse_subtitles(&html_str)?,
        })
    }
}
