use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Chapter, ChapterResponse};
use reqwest::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct ChapterQuery {
    slug_url: String,
    branch_id: Option<i64>,
    number: String,
    volume: String,
    site_id: SiteId,
}

impl ChapterQuery {
    pub fn new(
        slug_url: impl Into<String>,
        volume: impl Into<String>,
        number: impl Into<String>,
    ) -> Self {
        Self {
            slug_url: slug_url.into(),
            branch_id: None,
            number: number.into(),
            volume: volume.into(),
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub fn branch_id(mut self, branch_id: i64) -> Self {
        self.branch_id = Some(branch_id);
        self
    }

    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = number.into();
        self
    }

    pub fn volume(mut self, volume: impl Into<String>) -> Self {
        self.volume = volume.into();
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Option<Chapter>, Error> {
        let mut query = vec![("number", self.number.clone()), ("volume", self.volume.clone())];
        if let Some(branch_id) = self.branch_id {
            query.push(("branch_id", branch_id.to_string()));
        }

        let request = client
            .init_request(Method::GET, &format!("/manga/{}/chapter", self.slug_url), self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<ChapterResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}
