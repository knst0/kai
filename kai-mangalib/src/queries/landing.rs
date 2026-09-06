use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Landing, LandingResponse};
use reqwest::Method;

#[derive(Debug, Clone, Default)]
pub struct LandingQuery {
    site_id: Option<SiteId>,
}

impl LandingQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = Some(site_id);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Landing, Error> {
        let request = client
            .init_request(Method::GET, "/", self.site_id.unwrap_or(SiteId::Manga))
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<LandingResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn fetches_the_manga_landing_page() {
        let landing = LandingQuery::new().execute(&client()).await.expect("request failed");

        assert!(!landing.popular.is_empty(), "popular section was empty");
        assert!(!landing.latest_updates.is_empty(), "latest updates section was empty");
        for manga in &landing.popular {
            assert!(!manga.slug_url.is_empty());
        }
    }

    #[tokio::test]
    async fn fetches_the_anime_landing_page() {
        let landing = LandingQuery::new()
            .site_id(SiteId::Anime)
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!landing.popular.is_empty(), "popular section was empty");
    }
}
