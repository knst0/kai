use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;

/// Marks a chapter as viewed.
///
/// The response body shape is unconfirmed (only observed with an unauthenticated
/// request, which does not register a view), so this only reports whether the
/// request succeeded rather than parsing a response body.
#[derive(Debug, Clone)]
pub struct ChapterViewMutation {
    manga_id: i64,
    chapter_id: i64,
    site_id: SiteId,
}

impl ChapterViewMutation {
    pub fn new(manga_id: i64, chapter_id: i64) -> Self {
        Self { manga_id, chapter_id, site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<(), Error> {
        let request = client
            .init_request(
                Method::POST,
                &format!("/manga/{}/chapters/{}/view", self.manga_id, self.chapter_id),
                self.site_id,
            )
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        response.error_for_status().map_err(Error::HttpError)?;

        Ok(())
    }
}
