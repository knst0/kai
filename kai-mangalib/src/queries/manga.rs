use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Manga, MangaResponse};
use reqwest::{Method, StatusCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MangaField {
    Background,
    EngName,
    OtherNames,
    Summary,
    ReleaseDate,
    TypeId,
    Caution,
    Views,
    CloseView,
    RateAvg,
    Rate,
    Genres,
    Tags,
    Teams,
    User,
    Franchise,
    Authors,
    Publisher,
    UserRating,
    Moderated,
    Metadata,
    MetadataCount,
    MetadataCloseComments,
    MangaStatusId,
    ChapCount,
    StatusId,
    Artists,
    Format,
}

impl MangaField {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::EngName => "eng_name",
            Self::OtherNames => "otherNames",
            Self::Summary => "summary",
            Self::ReleaseDate => "releaseDate",
            Self::TypeId => "type_id",
            Self::Caution => "caution",
            Self::Views => "views",
            Self::CloseView => "close_view",
            Self::RateAvg => "rate_avg",
            Self::Rate => "rate",
            Self::Genres => "genres",
            Self::Tags => "tags",
            Self::Teams => "teams",
            Self::User => "user",
            Self::Franchise => "franchise",
            Self::Authors => "authors",
            Self::Publisher => "publisher",
            Self::UserRating => "userRating",
            Self::Moderated => "moderated",
            Self::Metadata => "metadata",
            Self::MetadataCount => "metadata.count",
            Self::MetadataCloseComments => "metadata.close_comments",
            Self::MangaStatusId => "manga_status_id",
            Self::ChapCount => "chap_count",
            Self::StatusId => "status_id",
            Self::Artists => "artists",
            Self::Format => "format",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MangaQuery {
    slug_url: String,
    fields: Vec<MangaField>,
    site_id: SiteId,
}

impl MangaQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), fields: Vec::new(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
        self
    }

    pub fn with_field(mut self, field: MangaField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: impl IntoIterator<Item = MangaField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Option<Manga>, Error> {
        let mut query = Vec::new();
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str()));
        }

        let request = client
            .init_request(Method::GET, &format!("/manga/{}", self.slug_url), self.site_id)
            .query(&query)
            .build()
            .expect("failed to build request");

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<MangaResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}
