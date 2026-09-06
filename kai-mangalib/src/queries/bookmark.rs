use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::MangaBookmarkResponse;
use reqwest::Method;
use serde_json::Value;

/// Returns the authenticated user's bookmark status for a manga. Only observed
/// as `null` (not bookmarked / unauthenticated), so the populated shape is
/// unconfirmed.
#[derive(Debug, Clone)]
pub struct MangaBookmarkQuery {
    slug_url: String,
    site_id: SiteId,
}

impl MangaBookmarkQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Option<Value>, Error> {
        let request = client
            .init_request(Method::GET, &format!("/manga/{}/bookmark", self.slug_url), self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<MangaBookmarkResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{MANGA_SLUG, client};

    #[tokio::test]
    async fn fetches_the_bookmark_state_of_a_title() {
        skip_without_token!();

        MangaBookmarkQuery::new(MANGA_SLUG).execute(&client()).await.expect("request failed");
    }
}
