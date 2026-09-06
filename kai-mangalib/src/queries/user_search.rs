use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{SearchResponse, User};
use reqwest::Method;

#[derive(Debug, Clone, Default)]
pub struct UserSearchQuery {
    q: Option<String>,
    sort_by: Option<String>,
    sort_type: Option<String>,
    page: Option<u32>,
    site_id: Option<SiteId>,
}

impl UserSearchQuery {
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

    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.sort_type = Some(sort_type.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<SearchResponse<User>, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(q) = &self.q {
            query.push(("q", q.clone()));
        }
        if let Some(sort_by) = &self.sort_by {
            query.push(("sort_by", sort_by.clone()));
        }
        if let Some(sort_type) = &self.sort_type {
            query.push(("sort_type", sort_type.clone()));
        }
        if let Some(page) = self.page {
            query.push(("page", page.to_string()));
        }

        let request = client
            .init_request(Method::GET, "/user", self.site_id.unwrap_or(SiteId::Manga))
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<SearchResponse<User>>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn searches_users_by_name() {
        let page = UserSearchQuery::new()
            .q("hello")
            .sort_by("id")
            .sort_type("asc")
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!page.data.is_empty(), "search returned no users");
        assert!(
            page.data.iter().any(|u| u.username.to_lowercase().contains("hello")),
            "no result mentioned the search term"
        );
    }

    #[tokio::test]
    async fn ascending_sort_is_respected() {
        let page = UserSearchQuery::new()
            .q("hello")
            .sort_by("id")
            .sort_type("asc")
            .execute(&client())
            .await
            .expect("request failed");

        let ids: Vec<i64> = page.data.iter().map(|u| u.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted, "results were not sorted by ascending id");
    }
}
