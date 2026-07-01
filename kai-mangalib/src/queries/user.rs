use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::queries::field_macro::media_field_enum;
use crate::types::{User, UserResponse};
use reqwest::{Method, StatusCode};

media_field_enum! {
    UserField,
    common: [
        Background => "background",
        Roles => "roles",
        Points => "points",
        BanInfo => "ban_info",
        Gender => "gender",
        CreatedAt => "created_at",
        About => "about",
        Teams => "teams",
        PremiumBackgroundId => "premium_background_id",
        LoginStreak => "login_streak",
        PreviousUsernames => "previous_usernames",
    ],
    extra: []
}

#[derive(Debug, Clone)]
pub struct UserQuery {
    user_id: i64,
    fields: Vec<UserField>,
    site_id: SiteId,
}

impl UserQuery {
    pub fn new(user_id: i64) -> Self {
        Self { user_id, fields: Vec::new(), site_id: SiteId::Manga }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn with_field(mut self, field: UserField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: impl IntoIterator<Item = UserField>) -> Self {
        self.fields.extend(fields);
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Option<User>, Error> {
        let mut query = Vec::new();
        for field_name in &self.fields {
            query.push(("fields[]", field_name.as_str()));
        }

        let request = client
            .init_request(Method::GET, &format!("/user/{}", self.user_id), self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let result = response.json::<UserResponse>().await.map_err(Error::HttpError)?;

        Ok(Some(result.data))
    }
}
