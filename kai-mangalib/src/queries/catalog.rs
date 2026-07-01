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
    fields: Vec<MangaListField>,
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
