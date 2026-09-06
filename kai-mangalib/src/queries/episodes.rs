use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Episode, EpisodesResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct EpisodesQuery {
    anime_id: String,
    site_id: SiteId,
}

impl EpisodesQuery {
    pub fn new(anime_id: impl Into<String>) -> Self {
        Self { anime_id: anime_id.into(), site_id: SiteId::Anime }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn anime_id(mut self, anime_id: impl Into<String>) -> Self {
        self.anime_id = anime_id.into();
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<Episode>, Error> {
        let request = client
            .init_request(Method::GET, "/episodes", self.site_id)
            .query(&[("anime_id", &self.anime_id)])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = match client.send(request).await {
            Ok(response) => response,
            Err(Error::NotFound) => return Ok(Vec::new()),
            Err(error) => return Err(error),
        };

        let result = response.json::<EpisodesResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{ANIME_SLUG, client};

    #[tokio::test]
    async fn lists_episodes_of_an_anime() {
        let episodes = EpisodesQuery::new(ANIME_SLUG)
            .site_id(SiteId::Anime)
            .execute(&client())
            .await
            .expect("request failed");

        for episode in &episodes {
            assert!(episode.id > 0);
        }
    }
}
