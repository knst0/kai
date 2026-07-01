use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::ChapterPlayersResponse;
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct ChapterPlayersQuery {
    slug_url: String,
    site_id: SiteId,
}

impl ChapterPlayersQuery {
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

    pub async fn execute(&self, client: &Client) -> Result<ChapterPlayersResponse, Error> {
        let request = client
            .init_request(
                Method::GET,
                &format!("/manga/{}/chapters/players", self.slug_url),
                self.site_id,
            )
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<ChapterPlayersResponse>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}
