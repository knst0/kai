use serde::Deserialize;
use serde_json::Value;

// ============================================================================
// Common Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct LabeledId {
    pub id: i64,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct Cover {
    pub filename: Option<String>,
    pub thumbnail: String,
    pub default: String,
    pub md: String,
}

#[derive(Debug, Deserialize)]
pub struct ImageFile {
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Views {
    pub total: i64,
    pub short: String,
    pub formated: String,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub average: String,
    #[serde(rename = "averageFormated")]
    pub average_formated: String,
    pub votes: i64,
    #[serde(rename = "votesFormated")]
    pub votes_formated: String,
    pub user: i64,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
    pub adult: bool,
    pub alert: bool,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    pub branch_id: Option<i64>,
    pub is_active: bool,
    pub subscriptions_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
    pub cover: Cover,
    pub details: Details,
    pub donate_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Subscription {
    pub is_subscribed: bool,
    pub source_type: String,
    pub source_id: i64,
    pub relation: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Publisher {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
    pub rus_name: Option<String>,
    pub cover: Cover,
    pub subscription: Subscription,
}

#[derive(Debug, Deserialize)]
pub struct Franchise {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct People {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
    pub rus_name: Option<String>,
    pub alt_name: Option<String>,
    pub cover: Cover,
    pub subscription: Subscription,
    pub confirmed: bool,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Premium {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub avatar: Avatar,
    pub last_online_at: Option<String>,
    pub premium: Premium,
}

#[derive(Debug, Deserialize)]
pub struct CommentsDisabled {
    pub media: bool,
    pub content: bool,
}

#[derive(Debug, Deserialize)]
pub struct Characters {
    #[serde(rename = "Main")]
    pub main: i64,
    #[serde(rename = "Supporting")]
    pub supporting: i64,
}

#[derive(Debug, Deserialize)]
pub struct Reviews {
    pub neutral: i64,
    pub positive: i64,
    pub negative: i64,
    pub all: i64,
}

#[derive(Debug, Deserialize)]
pub struct Count {
    pub branches: i64,
    pub characters: Characters,
    pub reviews: Reviews,
    pub relations: i64,
    pub people: i64,
    pub covers: i64,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub close_comments: i64,
    pub comments_disabled: CommentsDisabled,
    pub count: Count,
}

#[derive(Debug, Deserialize)]
pub struct ItemsCount {
    pub uploaded: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct Time {
    pub value: u32,
    pub formated: String,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeSchedule {
    pub number: String,
    pub airing_at: String,
}

#[derive(Debug, Deserialize)]
pub struct EpisodeStatus {
    pub id: String,
    pub label: String,
    pub abbr: Option<String>,
}

// ============================================================================
// Manga Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Manga {
    pub id: i64,
    pub name: String,
    pub rus_name: String,
    pub eng_name: String,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    pub is_licensed: bool,
    pub status: LabeledId,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,

    // Extended fields (sparse fieldsets)
    #[serde(default)]
    pub background: Option<ImageFile>,
    #[serde(default, rename = "otherNames")]
    pub other_names: Option<Vec<String>>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub close_view: Option<i64>,
    #[serde(default, rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub views: Option<Views>,
    #[serde(default, rename = "rate")]
    pub rating: Option<Rating>,
    #[serde(default)]
    pub moderated: Option<LabeledId>,
    #[serde(default)]
    pub teams: Option<Vec<Team>>,
    #[serde(default)]
    pub genres: Option<Vec<Genre>>,
    #[serde(default)]
    pub tags: Option<Vec<Genre>>,
    #[serde(default)]
    pub publisher: Option<Vec<Publisher>>,
    #[serde(default)]
    pub franchise: Option<Vec<Franchise>>,
    #[serde(default)]
    pub authors: Option<Vec<People>>,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
    #[serde(default, rename = "items_count")]
    pub items_count: Option<ItemsCount>,
    #[serde(default, rename = "scanlateStatus")]
    pub scanlate_status: Option<LabeledId>,
    #[serde(default)]
    pub artists: Option<Vec<People>>,
    #[serde(default)]
    pub format: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct MangaResponse {
    pub data: Manga,
}

// ============================================================================
// Anime Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub id: i64,
    pub name: String,
    pub rus_name: String,
    pub eng_name: String,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    pub is_licensed: bool,
    pub status: LabeledId,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,

    // Extended fields (sparse fieldsets)
    #[serde(default)]
    pub background: Option<ImageFile>,
    #[serde(default, rename = "otherNames")]
    pub other_names: Option<Vec<String>>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub close_view: Option<i64>,
    #[serde(default, rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub views: Option<Views>,
    #[serde(default, rename = "rate")]
    pub rating: Option<Rating>,
    #[serde(default)]
    pub moderated: Option<LabeledId>,
    #[serde(default)]
    pub teams: Option<Vec<Team>>,
    #[serde(default)]
    pub genres: Option<Vec<Genre>>,
    #[serde(default)]
    pub tags: Option<Vec<Genre>>,
    #[serde(default)]
    pub publisher: Option<Vec<Publisher>>,
    #[serde(default)]
    pub franchise: Option<Vec<Franchise>>,
    #[serde(default)]
    pub authors: Option<Vec<People>>,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
    #[serde(default)]
    pub time: Option<Time>,
    #[serde(default, rename = "items_count")]
    pub items_count: Option<ItemsCount>,
    #[serde(default, rename = "episodesSchedule")]
    pub episodes_schedule: Option<Vec<EpisodeSchedule>>,
    #[serde(default)]
    pub shikimori_href: Option<String>,
    #[serde(default)]
    pub shiki_rate: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeResponse {
    pub data: Anime,
}

// ============================================================================
// Episode Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub id: i64,
    pub model: String,
    pub name: String,
    pub number: String,
    pub number_secondary: String,
    pub season: String,
    pub status: EpisodeStatus,
    pub anime_id: i64,
    pub created_at: String,
    pub item_number: i64,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct EpisodesResponse {
    pub data: Vec<Episode>,
}
