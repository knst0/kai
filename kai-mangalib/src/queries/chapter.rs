use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Chapter, ChapterResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct ChapterQuery {
    slug_url: String,
    branch_id: Option<i64>,
    number: String,
    volume: String,
    site_id: SiteId,
}

impl ChapterQuery {
    pub fn new(
        slug_url: impl Into<String>,
        volume: impl Into<String>,
        number: impl Into<String>,
    ) -> Self {
        Self {
            slug_url: slug_url.into(),
            branch_id: None,
            number: number.into(),
            volume: volume.into(),
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub fn branch_id(mut self, branch_id: i64) -> Self {
        self.branch_id = Some(branch_id);
        self
    }

    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = number.into();
        self
    }

    pub fn volume(mut self, volume: impl Into<String>) -> Self {
        self.volume = volume.into();
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Option<Chapter>, Error> {
        let mut query = vec![("number", self.number.clone()), ("volume", self.volume.clone())];
        if let Some(branch_id) = self.branch_id {
            query.push(("branch_id", branch_id.to_string()));
        }

        let request = client
            .init_request(Method::GET, &format!("/manga/{}/chapter", self.slug_url), self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = match client.send(request).await {
            Ok(response) => response,
            Err(Error::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };

        let result = response.json::<ChapterResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::ChaptersQuery;
    use crate::queries::test_util::{BRANCHED_MANGA_SLUG, MISSING_SLUG, client};

    #[tokio::test]
    async fn fetches_the_first_chapter_of_a_title() {
        let chapters = ChaptersQuery::new(BRANCHED_MANGA_SLUG)
            .execute(&client())
            .await
            .expect("chapter list request failed");
        let first = chapters.first().expect("title reported no chapters");

        let chapter = ChapterQuery::new(BRANCHED_MANGA_SLUG, &first.volume, &first.number)
            .execute(&client())
            .await
            .expect("request failed")
            .expect("chapter should exist");

        assert_eq!(chapter.number, first.number);
        assert_eq!(chapter.volume, first.volume);
    }

    #[tokio::test]
    async fn missing_chapter_resolves_to_none() {
        let chapter = ChapterQuery::new(MISSING_SLUG, "1", "1")
            .execute(&client())
            .await
            .expect("request failed");

        assert!(chapter.is_none());
    }
}
