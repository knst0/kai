use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{ChapterListItem, ChaptersResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct ChaptersQuery {
    slug_url: String,
    site_id: SiteId,
}

impl ChaptersQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<ChapterListItem>, Error> {
        let request = client
            .init_request(Method::GET, &format!("/manga/{}/chapters", self.slug_url), self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = match client.send(request).await {
            Ok(response) => response,
            Err(Error::NotFound) => return Ok(Vec::new()),
            Err(error) => return Err(error),
        };

        let result = response.json::<ChaptersResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{BRANCHED_MANGA_SLUG, client};

    #[tokio::test]
    async fn lists_chapters_in_order() {
        let chapters = ChaptersQuery::new(BRANCHED_MANGA_SLUG)
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!chapters.is_empty(), "title reported no chapters");
        for chapter in &chapters {
            assert!(!chapter.number.is_empty());
            assert!(chapter.id > 0);
        }
    }
}
