use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::AnimeField;
use crate::types::{Anime, SearchResponse};
use reqwest::Method;

#[derive(Debug, Clone, Default)]
pub struct AnimeListQuery {
    q: Option<String>,
    fields: Vec<AnimeField>,
    moderated: Vec<i64>,
    types: Vec<i64>,
    genres: Vec<i64>,
    page: Option<u32>,
    site_id: Option<SiteId>,
}

impl AnimeListQuery {
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

    pub fn with_field(mut self, field: AnimeField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: impl IntoIterator<Item = AnimeField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn moderated(mut self, moderated: impl IntoIterator<Item = i64>) -> Self {
        self.moderated.extend(moderated);
        self
    }

    pub fn types(mut self, types: impl IntoIterator<Item = i64>) -> Self {
        self.types.extend(types);
        self
    }

    pub fn genres(mut self, genres: impl IntoIterator<Item = i64>) -> Self {
        self.genres.extend(genres);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<SearchResponse<Anime>, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(q) = &self.q {
            query.push(("q", q.clone()));
        }
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str().to_owned()));
        }
        for moderated in &self.moderated {
            query.push(("moderated[]", moderated.to_string()));
        }
        for type_id in &self.types {
            query.push(("types[]", type_id.to_string()));
        }
        for genre_id in &self.genres {
            query.push(("genres[]", genre_id.to_string()));
        }
        if let Some(page) = self.page {
            query.push(("page", page.to_string()));
        }

        let request = client
            .init_request(Method::GET, "/anime", self.site_id.unwrap_or(SiteId::Anime))
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<SearchResponse<Anime>>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn lists_the_anime_catalog() {
        let page = AnimeListQuery::new().execute(&client()).await.expect("request failed");

        assert!(!page.data.is_empty(), "anime catalog page was empty");
        for anime in &page.data {
            assert_eq!(anime.model, "anime");
        }
    }

    #[tokio::test]
    async fn search_matches_the_query() {
        let page = AnimeListQuery::new()
            .q("hello world")
            .moderated([0, 1, 2, 3])
            .with_fields([AnimeField::RateAvg, AnimeField::ReleaseDate])
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!page.data.is_empty(), "search returned no results");
        assert!(
            page.data.iter().any(|a| a.name.to_lowercase().contains("hello")),
            "no result mentioned the search term"
        );
    }

    #[tokio::test]
    async fn paginates() {
        let page = AnimeListQuery::new().page(2).execute(&client()).await.expect("request failed");

        assert_eq!(page.meta.current_page, 2);
    }
}
