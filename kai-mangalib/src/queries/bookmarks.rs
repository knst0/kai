use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Bookmark, BookmarksResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct BookmarksQuery {
    user_id: i64,
    status: Option<i64>,
    sort_by: String,
    sort_type: String,
    page: u32,
    site_id: SiteId,
}

impl BookmarksQuery {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            status: None,
            sort_by: "name".to_owned(),
            sort_type: "desc".to_owned(),
            page: 1,
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn status(mut self, status: i64) -> Self {
        self.status = Some(status);
        self
    }

    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = sort_by.into();
        self
    }

    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.sort_type = sort_type.into();
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<Bookmark>, Error> {
        let mut query = vec![
            ("user_id", self.user_id.to_string()),
            ("sort_by", self.sort_by.clone()),
            ("sort_type", self.sort_type.clone()),
            ("page", self.page.to_string()),
        ];
        if let Some(status) = self.status {
            query.push(("status", status.to_string()));
        }

        let request = client
            .init_request(Method::GET, "/bookmarks", self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<BookmarksResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}
