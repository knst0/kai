use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;
use serde_json::Value;

/// Requires an authenticated user; only observed as a 401 error without
/// credentials, so the success response shape is unconfirmed and returned as
/// raw JSON.
#[derive(Debug, Clone)]
pub struct AuthMeQuery {
    site_id: SiteId,
}

impl AuthMeQuery {
    pub fn new() -> Self {
        Self { site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Value, Error> {
        let request = client
            .init_request(Method::GET, "/auth/me", self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<Value>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

impl Default for AuthMeQuery {
    fn default() -> Self {
        Self::new()
    }
}
