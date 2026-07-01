use reqwest::{Method, Request, RequestBuilder, Response};
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
/// use kai_fansubs::client::Client;
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
    site_url: String,
    reqwest_client_builder: reqwest::ClientBuilder,
    rate_limit_per_minute: u64,
    rate_limit_per_second: u64,
}

static CRATE_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/sekiju/kai)",
);

impl ClientBuilder {
    /// Creates a new [`ClientBuilder`] with default settings.
    ///
    /// Default rate limits:
    /// - 90 requests per minute
    /// - 5 requests per second
    pub fn new() -> ClientBuilder {
        Self {
            site_url: "http://fansubs.ru".to_owned(),
            reqwest_client_builder: reqwest::ClientBuilder::new().user_agent(CRATE_USER_AGENT),
            rate_limit_per_minute: 90,
            rate_limit_per_second: 5,
        }
    }

    /// Sets the base Site URL for the client.
    pub fn site_url(mut self, url: impl Into<String>) -> ClientBuilder {
        self.site_url = url.into();
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
            .expect("kai-fansubs: failed to build reqwest client");

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

        Client { site_url: self.site_url, http_client, http_service }
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
    pub(crate) site_url: String,
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

    pub(crate) fn init_request(&self, method: Method, path: &str) -> RequestBuilder {
        self.http_client.request(method, self.site_url.clone() + path)
    }

    pub(crate) async fn send(&self, request: Request) -> Result<Response, BoxError> {
        let mut svc = self.http_service.clone();
        svc.ready().await?;
        svc.call(request).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
