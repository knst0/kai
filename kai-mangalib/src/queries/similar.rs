use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{SimilarItem, SimilarResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct SimilarQuery {
    slug_url: String,
    site_id: SiteId,
}

impl SimilarQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<SimilarItem>, Error> {
        let request = client
            .init_request(Method::GET, &format!("/manga/{}/similar", self.slug_url), self.site_id)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<SimilarResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{MANGA_SLUG, client};

    #[tokio::test]
    async fn lists_similar_titles() {
        let similar =
            SimilarQuery::new(MANGA_SLUG).execute(&client()).await.expect("request failed");

        for item in &similar {
            assert!(item.id > 0);
        }
    }
}
