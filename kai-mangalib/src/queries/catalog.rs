use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::field_macro::media_field_enum;
use crate::types::MangaListResponse;
use reqwest::Method;

media_field_enum! {
    MangaListField,
    common: [
        Rate => "rate",
        RateAvg => "rate_avg",
        UserBookmark => "userBookmark",
    ],
    extra: []
}

#[derive(Debug, Clone, Default)]
pub struct MangaListQuery {
    q: Option<String>,
    fields: Vec<MangaListField>,
    moderated: Vec<i64>,
    sort_by: Option<String>,
    sort_type: Option<String>,
    site_ids: Vec<i64>,
    types: Vec<i64>,
    genres: Vec<i64>,
    bookmarks: Vec<i64>,
    seed: Option<String>,
    rate_max: Option<f64>,
    ignored_teams: Option<bool>,
    licensed: Option<bool>,
    page: Option<u32>,
    site_id: Option<SiteId>,
}

impl MangaListQuery {
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

    pub fn moderated(mut self, moderated: impl IntoIterator<Item = i64>) -> Self {
        self.moderated.extend(moderated);
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

    pub fn with_field(mut self, field: MangaListField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: impl IntoIterator<Item = MangaListField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn site_ids(mut self, site_ids: impl IntoIterator<Item = i64>) -> Self {
        self.site_ids.extend(site_ids);
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

    pub fn bookmarks(mut self, bookmarks: impl IntoIterator<Item = i64>) -> Self {
        self.bookmarks.extend(bookmarks);
        self
    }

    pub fn seed(mut self, seed: impl Into<String>) -> Self {
        self.seed = Some(seed.into());
        self
    }

    pub fn rate_max(mut self, rate_max: f64) -> Self {
        self.rate_max = Some(rate_max);
        self
    }

    pub fn ignored_teams(mut self, ignored_teams: bool) -> Self {
        self.ignored_teams = Some(ignored_teams);
        self
    }

    pub fn licensed(mut self, licensed: bool) -> Self {
        self.licensed = Some(licensed);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<MangaListResponse, Error> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(q) = &self.q {
            query.push(("q", q.clone()));
        }
        for moderated in &self.moderated {
            query.push(("moderated[]", moderated.to_string()));
        }
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str().to_owned()));
        }
        for site_id in &self.site_ids {
            query.push(("site_id[]", site_id.to_string()));
        }
        for type_id in &self.types {
            query.push(("types[]", type_id.to_string()));
        }
        for genre_id in &self.genres {
            query.push(("genres[]", genre_id.to_string()));
        }
        for bookmark_status in &self.bookmarks {
            query.push(("bookmarks[]", bookmark_status.to_string()));
        }
        if let Some(seed) = &self.seed {
            query.push(("seed", seed.clone()));
        }
        if let Some(rate_max) = self.rate_max {
            query.push(("rate_max", rate_max.to_string()));
        }
        if let Some(ignored_teams) = self.ignored_teams {
            query.push(("ignored_teams", ignored_teams.to_string()));
        }
        if let Some(licensed) = self.licensed {
            query.push(("licensed", licensed.to_string()));
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
            .init_request(Method::GET, "/manga", self.site_id.unwrap_or(SiteId::Manga))
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<MangaListResponse>().await.map_err(Error::HttpError)?;

        Ok(result)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{GENRE_ID, client};

    #[tokio::test]
    async fn lists_the_catalog() {
        let page = MangaListQuery::new().execute(&client()).await.expect("request failed");

        assert!(!page.data.is_empty(), "catalog page was empty");
        assert_eq!(page.meta.current_page, 1);
        for manga in &page.data {
            assert!(!manga.slug_url.is_empty());
        }
    }

    #[tokio::test]
    async fn paginates() {
        let first = MangaListQuery::new().page(1).execute(&client()).await.expect("request failed");
        let second =
            MangaListQuery::new().page(2).execute(&client()).await.expect("request failed");

        assert_eq!(second.meta.current_page, 2);

        let first_ids: Vec<i64> = first.data.iter().map(|m| m.id).collect();
        let overlap = second.data.iter().filter(|m| first_ids.contains(&m.id)).count();
        assert!(overlap < second.data.len(), "page 2 duplicated page 1 entirely");
    }

    #[tokio::test]
    async fn search_matches_the_query() {
        let page =
            MangaListQuery::new().q("naruto").execute(&client()).await.expect("request failed");

        assert!(!page.data.is_empty(), "search returned no results");
        assert!(
            page.data.iter().any(|m| {
                m.name.to_lowercase().contains("naruto")
                    || m.rus_name.to_lowercase().contains("naruto")
                    || m.eng_name.as_deref().unwrap_or_default().to_lowercase().contains("naruto")
            }),
            "no result mentioned the search term"
        );
    }

    #[tokio::test]
    async fn requested_fields_are_populated() {
        let page = MangaListQuery::new()
            .with_fields([MangaListField::Rate, MangaListField::RateAvg])
            .execute(&client())
            .await
            .expect("request failed");

        assert!(page.data.iter().any(|m| m.rating.is_some()), "rate field was not returned");
    }

    #[tokio::test]
    async fn genre_filter_is_applied() {
        let page = MangaListQuery::new()
            .genres([GENRE_ID])
            .execute(&client())
            .await
            .expect("request failed");

        assert!(!page.data.is_empty(), "genre-filtered page was empty");
    }
}
