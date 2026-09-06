use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Branch, BranchesResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct BranchesQuery {
    manga_id: i64,
    team_defaults: Option<bool>,
    site_id: SiteId,
}

impl BranchesQuery {
    pub fn new(manga_id: i64) -> Self {
        Self { manga_id, team_defaults: None, site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn team_defaults(mut self, team_defaults: bool) -> Self {
        self.team_defaults = Some(team_defaults);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<Branch>, Error> {
        let mut query = Vec::new();
        if let Some(team_defaults) = self.team_defaults {
            query.push(("team_defaults", i32::from(team_defaults).to_string()));
        }

        let request = client
            .init_request(Method::GET, &format!("/branches/{}", self.manga_id), self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<BranchesResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{BRANCHED_MANGA_ID, client};

    #[tokio::test]
    async fn lists_translation_branches() {
        let branches =
            BranchesQuery::new(BRANCHED_MANGA_ID).execute(&client()).await.expect("request failed");

        for branch in &branches {
            assert!(branch.id > 0);
        }
    }

    #[tokio::test]
    async fn accepts_the_team_defaults_flag() {
        BranchesQuery::new(BRANCHED_MANGA_ID)
            .team_defaults(true)
            .execute(&client())
            .await
            .expect("request failed");
    }
}
