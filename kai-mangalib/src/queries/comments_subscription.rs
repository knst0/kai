use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use reqwest::Method;
use serde_json::Value;

/// Requires an authenticated user with permission on the target content; only
/// observed as a 403 permission error without credentials, so the success
/// response shape is unconfirmed and returned as raw JSON.
#[derive(Debug, Clone)]
pub struct CommentsSubscriptionQuery {
    content_id: i64,
    source_id: i64,
    source_type: String,
    site_id: SiteId,
}

impl CommentsSubscriptionQuery {
    pub fn new(content_id: i64, source_id: i64, source_type: impl Into<String>) -> Self {
        Self { content_id, source_id, source_type: source_type.into(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Value, Error> {
        let request = client
            .init_request(Method::GET, "/comments/subscription", self.site_id)
            .query(&[
                ("content_id", self.content_id.to_string()),
                ("source_id", self.source_id.to_string()),
                ("source_type", self.source_type.clone()),
            ])
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<Value>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{COMMENT_POST_TYPE, MANGA_ID, client};

    #[tokio::test]
    async fn fetches_the_subscription_state() {
        skip_without_token!();

        CommentsSubscriptionQuery::new(MANGA_ID, MANGA_ID, COMMENT_POST_TYPE)
            .execute(&client())
            .await
            .expect("request failed");
    }
}
