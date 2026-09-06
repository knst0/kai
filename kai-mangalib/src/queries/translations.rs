use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct TranslationsQuery {
    site_id: Option<SiteId>,
}

impl TranslationsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = Some(site_id);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<HashMap<String, Value>, Error> {
        let request = client
            .init_request(Method::GET, "/translations", self.site_id.unwrap_or(SiteId::Manga))
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<HashMap<String, Value>>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn fetches_the_translation_catalogue() {
        let translations =
            TranslationsQuery::new().execute(&client()).await.expect("request failed");

        assert!(!translations.is_empty(), "translation catalogue was empty");
        assert!(translations.contains_key("auth"), "expected an auth translation group");
    }
}
