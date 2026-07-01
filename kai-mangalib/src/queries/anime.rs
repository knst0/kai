use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Anime, AnimeResponse};
use reqwest::{Method, StatusCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnimeField {
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
    AnimeStatusId,
    Time,
    Episodes,
    EpisodesCount,
    EpisodesSchedule,
    ShikiRate,
}

impl AnimeField {
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
            Self::AnimeStatusId => "anime_status_id",
            Self::Time => "time",
            Self::Episodes => "episodes",
            Self::EpisodesCount => "episodes_count",
            Self::EpisodesSchedule => "episodesSchedule",
            Self::ShikiRate => "shiki_rate",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimeQuery {
    slug_url: String,
    fields: Vec<AnimeField>,
    site_id: SiteId,
}

impl AnimeQuery {
    pub fn new(slug_url: impl Into<String>) -> Self {
        Self { slug_url: slug_url.into(), fields: Vec::new(), site_id: SiteId::Anime }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn slug_url(mut self, slug_url: impl Into<String>) -> Self {
        self.slug_url = slug_url.into();
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

    pub async fn execute(&self, client: &Client) -> Result<Option<Anime>, Error> {
        let mut query = Vec::new();
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str()));
        }

        let request = client
            .init_request(Method::GET, &format!("/anime/{}", self.slug_url), self.site_id)
            .query(&query)
            .build()
            .expect("failed to build request");

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<AnimeResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}
