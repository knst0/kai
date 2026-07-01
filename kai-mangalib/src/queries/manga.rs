use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::field_macro::media_field_enum;
use crate::types::{Manga, MangaResponse};
use reqwest::{Method, StatusCode};

media_field_enum! {
    MangaField,
    common: [
        Background => "background",
        EngName => "eng_name",
        OtherNames => "otherNames",
        Summary => "summary",
        ReleaseDate => "releaseDate",
        TypeId => "type_id",
        Caution => "caution",
        Views => "views",
        CloseView => "close_view",
        RateAvg => "rate_avg",
        Rate => "rate",
        Genres => "genres",
        Tags => "tags",
        Teams => "teams",
        User => "user",
        Franchise => "franchise",
        Authors => "authors",
        Publisher => "publisher",
        UserRating => "userRating",
        Moderated => "moderated",
        Metadata => "metadata",
        MetadataCount => "metadata.count",
        MetadataCloseComments => "metadata.close_comments",
    ],
    extra: [
        MangaStatusId => "manga_status_id",
        ChapCount => "chap_count",
        StatusId => "status_id",
        Artists => "artists",
        Format => "format",
    ]
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
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<MangaResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}
