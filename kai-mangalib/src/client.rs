use crate::SiteId;
use crate::error::Error;
use reqwest::{Method, Request, RequestBuilder, Response, StatusCode};
use std::time::Duration;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::util::BoxCloneService;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};

/// A builder for configuring and creating a [`Client`].
///
/// This builder allows customization of the HTTP client, API URL, user agent,
/// proxy settings, and other configuration options before creating a [`Client`].
///
/// # Examples
///
/// ```rust,no_run
/// use kai_mangalib::client::Client;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a client with default settings
///     let client = Client::builder().build();
///
///     // Create a client with custom settings
///     let client = Client::builder()
///         .user_agent("MyApp/1.0")
///         .rate_limit_per_minute(60)
///         .rate_limit_per_second(3)
///         .build();
/// }
/// ```
#[derive(Debug)]
pub struct ClientBuilder {
    api_url: String,
    api_token: Option<String>,
    reqwest_client_builder: reqwest::ClientBuilder,
    rate_limit_per_minute: u64,
    rate_limit_per_second: u64,
}

static CRATE_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/knst0/kai)",
);

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`] with default settings.
    ///
    /// Default rate limits:
    /// - 90 requests per minute
    /// - 5 requests per second
    pub fn new() -> ClientBuilder {
        Self {
            api_url: "https://api.cdnlibs.org/api".to_owned(),
            api_token: None,
            // INFO: rustls used to fix TLS renegotiation issues.
            reqwest_client_builder: reqwest::ClientBuilder::new()
                .user_agent(CRATE_USER_AGENT)
                .use_rustls_tls(),
            rate_limit_per_minute: 90,
            rate_limit_per_second: 5,
        }
    }

    /// Sets the base Site URL for the client.
    pub fn api_url(mut self, url: impl Into<String>) -> ClientBuilder {
        self.api_url = url.into();
        self
    }

    /// Sets the API token for the client.
    pub fn api_token(mut self, token: impl Into<String>) -> ClientBuilder {
        self.api_token = Some(token.into());
        self
    }

    /// Sets a custom user agent for HTTP requests.
    pub fn user_agent(mut self, builder: impl Into<String>) -> ClientBuilder {
        self.reqwest_client_builder = self.reqwest_client_builder.user_agent(builder.into());
        self
    }

    /// Sets a proxy for HTTP requests.
    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.reqwest_client_builder = self.reqwest_client_builder.proxy(proxy);
        self
    }

    /// Replaces the internal reqwest ClientBuilder with a custom one.
    pub fn custom_reqwest_builder(mut self, builder: reqwest::ClientBuilder) -> ClientBuilder {
        self.reqwest_client_builder = builder;
        self
    }

    /// Sets the rate limit for requests per minute.
    pub fn rate_limit_per_minute(mut self, limit: u64) -> Self {
        self.rate_limit_per_minute = limit;
        self
    }

    /// Sets the rate limit for requests per second.
    pub fn rate_limit_per_second(mut self, limit: u64) -> Self {
        self.rate_limit_per_second = limit;
        self
    }

    /// Builds and returns a configured [`Client`].
    ///
    /// Panics if the underlying reqwest client cannot be built.
    pub fn build(self) -> Client {
        let http_client = self
            .reqwest_client_builder
            .build()
            .expect("kai-mangalib: failed to build reqwest client");

        let rpm_duration = Duration::from_millis(
            60_000_u64.checked_div(self.rate_limit_per_minute).unwrap_or(60_000),
        );

        let rps_duration = Duration::from_millis(
            1_000_u64.checked_div(self.rate_limit_per_second).unwrap_or(1_000),
        );

        let http_service: BoxCloneService<Request, Response, BoxError> = ServiceBuilder::new()
            .layer(BufferLayer::new(10))
            .layer(RateLimitLayer::new(self.rate_limit_per_minute, rpm_duration))
            .layer(RateLimitLayer::new(self.rate_limit_per_second, rps_duration))
            .service(http_client.clone())
            .boxed_clone();

        Client { api_url: self.api_url, api_token: self.api_token, http_client, http_service }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A HTTP client for interacting with the fansubs.ru API.
///
/// The client provides automatic rate limiting (90 requests per minute, 5 requests per second)
/// and is designed to be used with the query modules to perform API operations.
#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
    api_token: Option<String>,
    http_client: reqwest::Client,
    http_service: BoxCloneService<Request, Response, BoxError>,
}

impl Client {
    /// Creates a new [`Client`] with default settings.
    pub fn new() -> Client {
        ClientBuilder::new().build()
    }

    /// Returns a new [`ClientBuilder`] for configuring a client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) fn init_request(
        &self,
        method: Method,
        path: &str,
        site_id: SiteId,
    ) -> RequestBuilder {
        let mut request = self
            .http_client
            .request(method, self.api_url.clone() + path)
            .header("Site-Id", site_id);

        if let Some(token) = &self.api_token {
            request = request.bearer_auth(token);
        }

        request
    }

    pub(crate) async fn send(&self, request: Request) -> Result<Response, Error> {
        let mut svc = self.http_service.clone();
        svc.ready().await?;
        let response = svc.call(request).await.map_err(Error::from)?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        match status {
            StatusCode::NOT_FOUND => return Err(Error::NotFound),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => return Err(Error::Unauthorized),
            StatusCode::TOO_MANY_REQUESTS => return Err(Error::RateLimited),
            _ => {}
        }

        let body = response.text().await.unwrap_or_default();
        Err(Error::ApiError { status: status.as_u16(), message: summarize_api_error(&body) })
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Extracts a readable message from an API error body.
fn summarize_api_error(body: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
        return body.chars().take(200).collect();
    };

    let data = json.get("data").unwrap_or(&json);

    if let Some(message) = data.pointer("/toast/message").and_then(|m| m.as_str()) {
        return message.to_owned();
    }

    if let Some(fields) = data.as_object() {
        let mut parts: Vec<String> = fields
            .iter()
            .filter_map(|(field, reasons)| {
                let reasons = reasons.as_array()?;
                let joined =
                    reasons.iter().filter_map(|r| r.as_str()).collect::<Vec<_>>().join("; ");
                (!joined.is_empty()).then(|| format!("{field}: {joined}"))
            })
            .collect();
        parts.sort();
        if !parts.is_empty() {
            return parts.join(", ");
        }
    }

    body.chars().take(200).collect()
}
