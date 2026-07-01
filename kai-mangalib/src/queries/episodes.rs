use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Episode, EpisodesResponse};
use reqwest::{Method, StatusCode};

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
            .expect("failed to build request");

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let result = response.json::<EpisodesResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}
