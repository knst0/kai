use crate::SiteId;
use crate::client::Client;
use crate::error::Error;
use crate::types::{Bookmark, BookmarksResponse};
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct BookmarksQuery {
    user_id: i64,
    status: i64,
    sort_by: String,
    sort_type: String,
    page: u32,
    site_id: SiteId,
}

impl BookmarksQuery {
    /// Creates a query for one bookmark shelf.
    ///
    /// `status` selects the shelf and is required by the API; a request without
    /// it is rejected with a validation error rather than an empty list.
    pub fn new(user_id: i64, status: i64) -> Self {
        Self {
            user_id,
            status,
            sort_by: "name".to_owned(),
            sort_type: "desc".to_owned(),
            page: 1,
            site_id: SiteId::Manga,
        }
    }

    pub fn site_id(mut self, site_id: SiteId) -> Self {
        self.site_id = site_id;
        self
    }

    pub fn status(mut self, status: i64) -> Self {
        self.status = status;
        self
    }

    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = sort_by.into();
        self
    }

    pub fn sort_type(mut self, sort_type: impl Into<String>) -> Self {
        self.sort_type = sort_type.into();
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub async fn execute(&self, client: &Client) -> Result<Vec<Bookmark>, Error> {
        let query = vec![
            ("status", self.status.to_string()),
            ("user_id", self.user_id.to_string()),
            ("sort_by", self.sort_by.clone()),
            ("sort_type", self.sort_type.clone()),
            ("page", self.page.to_string()),
        ];

        let request = client
            .init_request(Method::GET, "/bookmarks", self.site_id)
            .query(&query)
            .build()
            .map_err(|e| Error::RequestBuildError(e.to_string()))?;

        let response = client.send(request).await?;
        let result = response.json::<BookmarksResponse>().await.map_err(Error::HttpError)?;

        Ok(result.data)
    }
}

#[cfg(all(test, feature = "live-tests"))]
mod tests {
    use super::*;
    use crate::queries::test_util::{BOOKMARK_STATUS, USER_ID, client};

    #[tokio::test]
    async fn lists_a_users_bookmarks() {
        skip_without_token!();

        let bookmarks = BookmarksQuery::new(USER_ID, BOOKMARK_STATUS)
            .execute(&client())
            .await
            .expect("request failed");

        for bookmark in &bookmarks {
            assert!(bookmark.id > 0);
            assert_eq!(bookmark.status, BOOKMARK_STATUS);
        }
    }

    /// Reading another account's shelf is allowed but yields nothing, so this
    /// case works without a token and still exercises the success decode path.
    #[tokio::test]
    async fn a_shelf_decodes_even_when_empty() {
        BookmarksQuery::new(USER_ID, BOOKMARK_STATUS)
            .sort_by("created_at")
            .execute(&client())
            .await
            .expect("request failed");
    }

    /// A rejected request must surface as [`Error::ApiError`] carrying the
    /// server's explanation, not as an opaque serde decode failure from trying
    /// to read the validation object as a list.
    #[tokio::test]
    async fn a_rejected_request_reports_the_api_message() {
        let error = BookmarksQuery::new(USER_ID, BOOKMARK_STATUS)
            .sort_by("definitely-not-a-sort-column")
            .execute(&client())
            .await
            .expect_err("the API should reject an unknown sort column");

        match error {
            Error::ApiError { status, message } => {
                assert_eq!(status, 422);
                assert!(
                    message.contains("sort_by"),
                    "message should name the offending field, got: {message}"
                );
            }
            other => panic!("expected Error::ApiError, got {other:?}"),
        }
    }
}
