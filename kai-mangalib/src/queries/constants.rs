use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::field_macro::media_field_enum;
use crate::types::{Constants, ConstantsResponse};
use reqwest::Method;

media_field_enum! {
    ConstantsField,
    common: [
        Types => "types",
        ScanlateStatus => "scanlateStatus",
        ImageServers => "imageServers",
        Genres => "genres",
        Tags => "tags",
    ],
    extra: []
}

#[derive(Debug, Clone, Default)]
pub struct ConstantsQuery {
    fields: Vec<ConstantsField>,
    site_id: Option<SiteId>,
}

impl ConstantsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = Some(site_id);
        self
    }

    pub fn with_field(mut self, field: ConstantsField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: impl IntoIterator<Item = ConstantsField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Constants, Error> {
        let mut query = Vec::new();
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str()));
        }

        let request = client
            .init_request(Method::GET, "/constants", self.site_id.unwrap_or(SiteId::Manga))
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<ConstantsResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::client;

    #[tokio::test]
    async fn fetches_catalog_constants() {
        let constants = ConstantsQuery::new()
            .with_fields([ConstantsField::Genres, ConstantsField::Tags])
            .execute(&client())
            .await
            .expect("request failed");

        assert!(
            constants.genres.as_ref().is_some_and(|g| !g.is_empty()),
            "genres constant was not returned"
        );
    }
}
