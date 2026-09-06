use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Franchise, SearchResponse};
use reqwest::Method;

#[derive(Debug, Clone, Default)]
pub struct FranchiseSearchQuery {
    q: Option<String>,
    page: Option<u32>,
    site_id: Option<SiteId>,
}

impl FranchiseSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = Some(site_id);
        self
    }

    pub fn q(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<SearchResponse<Franchise>, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(q) = &self.q {
            query.push(("q", q.clone()));
        }
        if let Some(page) = self.page {
            query.push(("page", page.to_string()));
        }

        let request = client
            .init_request(Method::GET, "/franchise", self.site_id.unwrap_or(SiteId::Manga))
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result =
            response.json::<SearchResponse<Franchise>>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn lists_franchises() {
        let page = FranchiseSearchQuery::new().execute(&client()).await.expect("request failed");

        for franchise in &page.data {
            assert!(!franchise.slug_url.is_empty());
        }
    }

    #[tokio::test]
    async fn nonsense_query_returns_an_empty_page() {
        let page = FranchiseSearchQuery::new()
            .q("zzzqqqxxxnotarealfranchise")
            .execute(&client())
            .await
            .expect("request failed");

        assert!(page.data.is_empty());
    }
}
