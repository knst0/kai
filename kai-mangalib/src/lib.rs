use reqwest::header::HeaderValue;

pub mod client;
pub mod error;
pub mod queries;
pub mod types;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SiteId {
    Manga = 1,
    Slash = 2,
    Ranobe = 3,
    Hentai = 4,
    Anime = 5,
}

impl From<SiteId> for HeaderValue {
    fn from(value: SiteId) -> Self {
        let s = (value as u8).to_string();
        HeaderValue::from_str(&s).expect("valid header value")
    }
}
