use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Stats, StatsResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct StatsQuery {
    slug_url: String,
    bookmarks: bool,
    rating: bool,
    site_id: SiteId,
}

impl StatsQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), bookmarks: true, rating: true, site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub fn bookmarks(mut self, bookmarks: bool) -> Self {
        self.bookmarks = bookmarks;
        self
    }

    pub fn rating(mut self, rating: bool) -> Self {
        self.rating = rating;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Stats, Error> {
        let request = client
            .init_request(Method::GET, &format!("/manga/{}/stats", self.slug_url), self.site_id)
            .query(&[("bookmarks", self.bookmarks), ("rating", self.rating)])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<StatsResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{MANGA_SLUG, client};

    #[tokio::test]
    async fn fetches_bookmark_and_rating_stats() {
        let stats = StatsQuery::new(MANGA_SLUG)
            .bookmarks(true)
            .rating(true)
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!stats.bookmarks.stats.is_empty(), "bookmark stats were empty");
        assert!(!stats.rating.stats.is_empty(), "rating stats were empty");
        assert!(stats.bookmarks.count >= 0);
    }
}
