use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::field_macro::media_field_enum;
use crate::types::{Anime, AnimeResponse};
use reqwest::{Method, StatusCode};

media_field_enum! {
    AnimeField,
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
        AnimeStatusId => "anime_status_id",
        Time => "time",
        Episodes => "episodes",
        EpisodesCount => "episodes_count",
        EpisodesSchedule => "episodesSchedule",
        ShikiRate => "shiki_rate",
    ]
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
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<AnimeResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{ANIME_SLUG, MISSING_SLUG, client};

    #[tokio::test]
    async fn fetches_a_known_anime() {
        let anime = AnimeQuery::new(ANIME_SLUG)
            .site_id(SiteId::Anime)
            .execute(&client())
            .await
            .expect("request failed")
            .expect("anime should exist");

        assert_eq!(anime.slug_url, ANIME_SLUG);
        assert_eq!(anime.model, "anime");
        assert!(!anime.name.is_empty());
    }

    #[tokio::test]
    async fn requested_fields_are_populated() {
        let anime = AnimeQuery::new(ANIME_SLUG)
            .site_id(SiteId::Anime)
            .with_fields([AnimeField::Summary, AnimeField::Genres, AnimeField::EpisodesCount])
            .execute(&client())
            .await
            .expect("request failed")
            .expect("anime should exist");

        assert!(anime.summary.is_some(), "summary field was not returned");
        assert!(
            anime.genres.as_ref().is_some_and(|g| !g.is_empty()),
            "genres field was not returned"
        );
    }

    #[tokio::test]
    async fn missing_anime_resolves_to_none() {
        let anime = AnimeQuery::new(MISSING_SLUG)
            .site_id(SiteId::Anime)
            .execute(&client())
            .await
            .expect("request failed");

        assert!(anime.is_none());
    }
}
