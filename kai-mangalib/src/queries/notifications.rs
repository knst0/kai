use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;
use serde_json::Value;

/// Requires an authenticated user; only observed as a 401 error without
/// credentials, so the success response shape is unconfirmed and returned as
/// raw JSON.
#[derive(Debug, Clone)]
pub struct NotificationsQuery {
    notification_type: String,
    read_type: String,
    sort_type: String,
    page: u32,
    site_id: SiteId,
}

impl NotificationsQuery {
    pub fn new() -> Self {
        Self {
            notification_type: "all".to_owned(),
            read_type: "unread".to_owned(),
            sort_type: "desc".to_owned(),
            page: 1,
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn notification_type(mut self, notification_type: impl Into<String>) -> Self {
        self.notification_type = notification_type.into();
        self
    }

    pub fn read_type(mut self, read_type: impl Into<String>) -> Self {
        self.read_type = read_type.into();
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

    pub async fn execute(&self, client: &Client) -> Result<Value, Error> {
        let request = client
            .init_request(Method::GET, "/notifications", self.site_id)
            .query(&[
                ("notification_type", self.notification_type.clone()),
                ("read_type", self.read_type.clone()),
                ("sort_type", self.sort_type.clone()),
                ("page", self.page.to_string()),
            ])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<Value>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

impl Default for NotificationsQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires an authenticated user; only observed as a 401 error without
/// credentials, so the success response shape is unconfirmed and returned as
/// raw JSON.
#[derive(Debug, Clone)]
pub struct NotificationsCountQuery {
    site_id: SiteId,
}

impl NotificationsCountQuery {
    pub fn new() -> Self {
        Self { site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Value, Error> {
        let request = client
            .init_request(Method::GET, "/notifications/count", self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<Value>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

impl Default for NotificationsCountQuery {
    fn default() -> Self {
        Self::new()
    }
}
