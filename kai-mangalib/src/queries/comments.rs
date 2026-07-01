use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Comment, CommentsData, CommentsResponse, CommentsStickyResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct CommentsQuery {
    post_id: i64,
    post_type: String,
    post_page: u32,
    page: u32,
    sort_by: String,
    sort_type: String,
    site_id: SiteId,
}

impl CommentsQuery {
    pub fn new(post_id: i64, post_type: impl Into<String>) -> Self {
        Self {
            post_id,
            post_type: post_type.into(),
            post_page: 1,
            page: 1,
            sort_by: "id".to_owned(),
            sort_type: "desc".to_owned(),
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn post_page(mut self, post_page: u32) -> Self {
        self.post_page = post_page;
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
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

    pub async fn execute(&self, client: &Client) -> Result<CommentsData, Error> {
        let request = client
            .init_request(Method::GET, "/comments", self.site_id)
            .query(&[
                ("post_id", self.post_id.to_string()),
                ("post_type", self.post_type.clone()),
                ("post_page", self.post_page.to_string()),
                ("page", self.page.to_string()),
                ("sort_by", self.sort_by.clone()),
                ("sort_type", self.sort_type.clone()),
            ])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<CommentsResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[derive(Debug, Clone)]
pub struct CommentsStickyQuery {
    post_id: i64,
    post_type: String,
    site_id: SiteId,
}

impl CommentsStickyQuery {
    pub fn new(post_id: i64, post_type: impl Into<String>) -> Self {
        Self { post_id, post_type: post_type.into(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<Comment>, Error> {
        let request = client
            .init_request(Method::GET, "/comments/sticky", self.site_id)
            .query(&[("post_id", self.post_id.to_string()), ("post_type", self.post_type.clone())])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<CommentsStickyResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}
