use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{UserStats, UserStatsResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct UserStatsQuery {
    user_id: i64,
    site_id: SiteId,
}

impl UserStatsQuery {
    pub fn new(user_id: i64) -> Self {
        Self { user_id, site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<UserStats, Error> {
        let request = client
            .init_request(Method::GET, &format!("/user/{}/stats", self.user_id), self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<UserStatsResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{USER_ID, client};

    #[tokio::test]
    async fn fetches_public_user_statistics() {
        UserStatsQuery::new(USER_ID).execute(&client()).await.expect("request failed");
    }
}
