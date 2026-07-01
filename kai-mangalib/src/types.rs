use serde::{Deserialize, Deserializer};
use serde_json::Value;

// Upstream serializes this as either a JSON boolean or a 0/1 integer depending
// on the endpoint (observed on `/manga/{slug}` vs `/anime/{slug}`).
fn deserialize_lenient_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrInt {
        Bool(bool),
        Int(i64),
    }

    Ok(match BoolOrInt::deserialize(deserializer)? {
        BoolOrInt::Bool(b) => b,
        BoolOrInt::Int(i) => i != 0,
    })
}

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
    #[serde(default)]
    pub is_media_spoiler: Option<bool>,
    #[serde(default)]
    pub is_general_spoiler: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ContentMarking {
    pub id: i64,
    pub label: String,
    pub formulation: String,
}

#[derive(Debug, Deserialize)]
pub struct FormatPivot {
    pub manga_id: i64,
    pub format_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub id: i64,
    pub name: String,
    pub pivot: FormatPivot,
}

#[derive(Debug, Deserialize)]
pub struct LoginStreak {
    pub last_login_at: String,
    pub login_streak: i64,
    pub max_login_streak: i64,
}

#[derive(Debug, Deserialize)]
pub struct LoginStreakPreferences {
    pub login_streak_light_icon_style: Option<String>,
    pub login_streak_dark_icon_style: Option<String>,
    pub login_streak_theme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PointsInfo {
    pub top: i64,
    pub total_points: i64,
    pub level: i64,
    pub max_level_points: i64,
    pub current_level_points: i64,
    pub point_percent_progress: f64,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub rus_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserTeam {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
    pub cover: Cover,
    #[serde(default)]
    pub stats: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMeta {
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Votes {
    pub up: i64,
    pub down: i64,
    pub user: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Details {
    pub branch_id: Option<i64>,
    #[serde(deserialize_with = "deserialize_lenient_bool")]
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
    #[serde(default)]
    pub details: Option<Details>,
    #[serde(default)]
    pub donate_enabled: Option<bool>,
    #[serde(default)]
    pub open_chapter_with_content_key_enabled: Option<bool>,
    #[serde(default)]
    pub vk: Option<String>,
    #[serde(default)]
    pub discord: Option<String>,
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

    // Extended fields (sparse fieldsets, e.g. via the standalone user endpoint)
    #[serde(default)]
    pub background: Option<ImageFile>,
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub gender: Option<LabeledId>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub ban_info: Option<Value>,
    #[serde(default)]
    pub points_info: Option<PointsInfo>,
    #[serde(default)]
    pub teams: Option<Vec<UserTeam>>,
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub previous_usernames: Option<Value>,
    #[serde(default)]
    pub premium_background_id: Option<Value>,
    #[serde(default)]
    pub can_view_profile: Option<bool>,
    #[serde(default)]
    pub can_view_statistics: Option<bool>,
    #[serde(default)]
    pub viewable_statistics_site_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub can_view_previous_usernames: Option<bool>,
    #[serde(default)]
    pub login_streak: Option<LoginStreak>,
    #[serde(default)]
    pub login_streak_preferences: Option<LoginStreakPreferences>,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub data: User,
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
    #[serde(default)]
    pub close_comments: Option<i64>,
    #[serde(default)]
    pub comments_disabled: Option<CommentsDisabled>,
    #[serde(default)]
    pub count: Option<Count>,
    #[serde(default)]
    pub last_shiki_updated_at: Option<String>,
    #[serde(default)]
    pub last_anilist_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_characters_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_related_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_status_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_tags_updated_at: Option<String>,
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
    pub eng_name: Option<String>,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    #[serde(default)]
    pub is_licensed: Option<bool>,
    pub status: LabeledId,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,
    #[serde(default)]
    pub content_marking: Vec<ContentMarking>,

    // Extended fields (sparse fieldsets)
    #[serde(default)]
    pub background: Option<ImageFile>,
    #[serde(default, rename = "otherNames")]
    pub other_names: Option<Vec<String>>,
    #[serde(default)]
    pub summary: Option<Value>,
    #[serde(default)]
    pub close_view: Option<i64>,
    #[serde(default, rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub views: Option<Views>,
    #[serde(default)]
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
    pub format: Option<Vec<Format>>,
    #[serde(default)]
    pub translation_quality_rating: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct MangaResponse {
    pub data: Manga,
    #[serde(default)]
    pub meta: Option<ResponseMeta>,
}

// ============================================================================
// Anime Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub id: i64,
    pub name: String,
    pub rus_name: String,
    pub eng_name: Option<String>,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    #[serde(default)]
    pub is_licensed: Option<bool>,
    pub status: LabeledId,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,
    #[serde(default)]
    pub content_marking: Vec<ContentMarking>,

    // Extended fields (sparse fieldsets)
    #[serde(default)]
    pub background: Option<ImageFile>,
    #[serde(default, rename = "otherNames")]
    pub other_names: Option<Vec<String>>,
    #[serde(default)]
    pub summary: Option<Value>,
    #[serde(default)]
    pub close_view: Option<i64>,
    #[serde(default, rename = "releaseDate")]
    pub release_date: Option<String>,
    #[serde(default)]
    pub views: Option<Views>,
    #[serde(default)]
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
    // NOTE: upstream sometimes serializes this as a numeric string instead of a number.
    #[serde(default)]
    pub anilist_id: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeResponse {
    pub data: Anime,
    #[serde(default)]
    pub meta: Option<ResponseMeta>,
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

// ============================================================================
// Media Preview Types (similar / relations)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct MediaPreview {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub rus_name: Option<String>,
    #[serde(default)]
    pub eng_name: Option<String>,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    #[serde(default)]
    pub is_licensed: Option<bool>,
    #[serde(default)]
    pub content_marking: Vec<ContentMarking>,
    pub status: LabeledId,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,
    // NOTE: upstream sometimes serializes this as a numeric string instead of a number.
    #[serde(default)]
    pub anilist_id: Option<Value>,
    #[serde(default)]
    pub shiki_rate: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SimilarItem {
    pub id: i64,
    pub similar: String,
    pub user_id: i64,
    pub media: MediaPreview,
    pub votes: Votes,
}

#[derive(Debug, Deserialize)]
pub struct SimilarResponse {
    pub data: Vec<SimilarItem>,
}

#[derive(Debug, Deserialize)]
pub struct RelationItem {
    pub order: i64,
    pub related_type: LabeledId,
    pub media: MediaPreview,
}

#[derive(Debug, Deserialize)]
pub struct RelationsResponse {
    pub data: Vec<RelationItem>,
}

// ============================================================================
// Stats Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct BookmarkStatMeta {
    pub id: Value,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkStat {
    pub percent: f64,
    pub value: i64,
    pub label: String,
    pub meta: BookmarkStatMeta,
}

#[derive(Debug, Deserialize)]
pub struct BookmarksStats {
    pub count: i64,
    pub stats: Vec<BookmarkStat>,
}

#[derive(Debug, Deserialize)]
pub struct RatingStatMeta {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RatingStat {
    pub label: i64,
    pub value: i64,
    pub percent: f64,
    pub meta: RatingStatMeta,
}

#[derive(Debug, Deserialize)]
pub struct RatingStats {
    pub count: i64,
    pub stats: Vec<RatingStat>,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub bookmarks: BookmarksStats,
    pub rating: RatingStats,
}

#[derive(Debug, Deserialize)]
pub struct StatsResponse {
    pub data: Stats,
}

// ============================================================================
// Branches Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub id: i64,
    pub name: String,
    pub notify: bool,
    pub teams: Vec<Team>,
    #[serde(default)]
    pub translation_quality_rating: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct BranchesResponse {
    pub data: Vec<Branch>,
}

// ============================================================================
// Chapters Types (list)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct TeamMini {
    pub id: i64,
    pub slug: String,
    pub slug_url: String,
    pub model: String,
    pub name: String,
    pub cover: Cover,
}

#[derive(Debug, Deserialize)]
pub struct SimpleUser {
    pub username: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ChapterBranch {
    pub id: i64,
    pub branch_id: i64,
    pub created_at: String,
    pub teams: Vec<TeamMini>,
    pub expired_type: i64,
    pub user: SimpleUser,
}

#[derive(Debug, Deserialize)]
pub struct ChapterListItem {
    pub id: i64,
    pub index: i64,
    pub item_number: i64,
    pub volume: String,
    pub number: String,
    pub number_secondary: String,
    pub name: Option<String>,
    pub branches_count: i64,
    pub branches: Vec<ChapterBranch>,
    pub bundle_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ChaptersResponse {
    pub data: Vec<ChapterListItem>,
}

// ============================================================================
// Chapter Types (single, reader)
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ChapterPage {
    pub id: i64,
    pub image: String,
    pub slug: i64,
    pub external: i64,
    pub chunks: i64,
    pub chapter_id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub height: i64,
    pub width: i64,
    pub url: String,
    pub ratio: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslationQualityCategories {
    pub translation_accuracy: String,
    pub readability_adaptation: String,
    pub editing_formatting: String,
}

#[derive(Debug, Deserialize)]
pub struct TranslationQualityUserRating {
    pub average: String,
    #[serde(rename = "averageFormated")]
    pub average_formated: String,
    pub translation_accuracy: i64,
    pub readability_adaptation: i64,
    pub editing_formatting: i64,
}

#[derive(Debug, Deserialize)]
pub struct TranslationQualityRating {
    pub average: String,
    #[serde(rename = "averageFormated")]
    pub average_formated: String,
    pub votes: i64,
    pub rated_chapters: i64,
    pub can_rate: bool,
    pub categories: TranslationQualityCategories,
    pub user: TranslationQualityUserRating,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub model: String,
    pub volume: String,
    pub number: String,
    pub number_secondary: String,
    pub name: String,
    pub slug: String,
    pub branch_id: i64,
    pub manga_id: i64,
    pub created_at: String,
    pub expired_at: Option<String>,
    pub moderated: LabeledId,
    pub likes_count: i64,
    pub is_liked: bool,
    pub is_viewed: bool,
    pub expired_type: i64,
    pub teams: Vec<Team>,
    pub bundle_id: Option<i64>,
    pub bundle: Option<Value>,
    pub publish_at: Option<String>,
    #[serde(default)]
    pub translation_quality_rating: Option<TranslationQualityRating>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub pages: Vec<ChapterPage>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterResponse {
    pub data: Chapter,
}

// ============================================================================
// Chapter Players Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct PageLinks {
    pub first: Option<String>,
    pub last: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterPlayersMeta {
    pub current_page: i64,
    pub from: Option<i64>,
    pub last_page: i64,
    #[serde(default)]
    pub links: Vec<Value>,
    pub path: String,
    pub per_page: i64,
    pub to: Option<i64>,
    pub total: i64,
}

/// Item shape is unconfirmed (no sample data with populated external players was
/// available while reverse-engineering this endpoint), so entries are left untyped.
#[derive(Debug, Deserialize)]
pub struct ChapterPlayersResponse {
    pub data: Vec<Value>,
    pub links: PageLinks,
    pub meta: ChapterPlayersMeta,
}

// ============================================================================
// Manga Catalog Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct MangaListMeta {
    pub current_page: i64,
    pub from: Option<i64>,
    pub path: String,
    pub per_page: i64,
    pub to: Option<i64>,
    pub page: i64,
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(default)]
    pub seed: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MangaListResponse {
    pub data: Vec<Manga>,
    pub links: PageLinks,
    pub meta: MangaListMeta,
}

// ============================================================================
// Constants Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct ConstantType {
    pub id: i64,
    pub label: String,
    #[serde(default)]
    pub site_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ImageServer {
    pub id: String,
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub site_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ConstantGenre {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub alt_name: Vec<String>,
    #[serde(default)]
    pub dsc: String,
    pub adult: bool,
    pub alert: bool,
    #[serde(default)]
    pub site_ids: Vec<i64>,
    #[serde(default)]
    pub blocked_for_country: Vec<Value>,
    #[serde(default)]
    pub allowed_for_country: Vec<Value>,
    #[serde(default)]
    pub allowed_for_domain: Vec<Value>,
    // NOTE: upstream sometimes serializes these as numeric strings instead of numbers.
    #[serde(default)]
    pub anilist_id: Option<Value>,
    #[serde(default)]
    pub shiki_id: Option<Value>,
    #[serde(default)]
    pub mal_id: Option<Value>,
    #[serde(default)]
    pub is_media_spoiler: Option<bool>,
    #[serde(default)]
    pub is_general_spoiler: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Constants {
    #[serde(default)]
    pub types: Option<Vec<ConstantType>>,
    #[serde(default, rename = "scanlateStatus")]
    pub scanlate_status: Option<Vec<ConstantType>>,
    #[serde(default, rename = "imageServers")]
    pub image_servers: Option<Vec<ImageServer>>,
    #[serde(default)]
    pub genres: Option<Vec<ConstantGenre>>,
    #[serde(default)]
    pub tags: Option<Vec<ConstantGenre>>,
}

#[derive(Debug, Deserialize)]
pub struct ConstantsResponse {
    pub data: Constants,
}

// ============================================================================
// Comments Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CommentRelationMedia {
    pub id: i64,
    pub model: String,
    pub name: String,
    #[serde(default)]
    pub rus_name: Option<String>,
    #[serde(default)]
    pub eng_name: Option<String>,
    pub slug: String,
    pub slug_url: String,
    pub site: i64,
}

#[derive(Debug, Deserialize)]
pub struct CommentRelation {
    pub id: i64,
    pub name: String,
    pub volume: String,
    pub number_secondary: String,
    pub number: String,
    pub branch_id: i64,
    pub manga_id: i64,
    pub media: CommentRelationMedia,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub root_id: Option<i64>,
    pub parent_comment: Option<i64>,
    pub comment_level: i64,
    pub post_page: i64,
    pub comment: String,
    pub created_at: String,
    pub created_at_ts: i64,
    pub relation_type: String,
    pub relation_id: i64,
    pub user: User,
    pub votes: Votes,
    pub relation: CommentRelation,
}

#[derive(Debug, Deserialize)]
pub struct CommentsData {
    #[serde(default)]
    pub root: Vec<Comment>,
    #[serde(default)]
    pub replies: Vec<Comment>,
}

#[derive(Debug, Deserialize)]
pub struct CommentsMeta {
    pub has_next_page: bool,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct CommentsResponse {
    pub data: CommentsData,
    pub meta: CommentsMeta,
}

#[derive(Debug, Deserialize)]
pub struct CommentsStickyResponse {
    pub data: Vec<Comment>,
}

// ============================================================================
// Bookmarks Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct LastItem {
    pub id: i64,
    #[serde(default)]
    pub volume: Option<Value>,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub branch_id: Option<i64>,
    #[serde(default)]
    pub status: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkMediaMetadata {
    #[serde(default)]
    pub last_item: Option<LastItem>,
    #[serde(default)]
    pub last_shiki_updated_at: Option<String>,
    #[serde(default)]
    pub last_anilist_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_characters_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_related_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_status_updated_at: Option<String>,
    #[serde(default)]
    pub anilist_tags_updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkMedia {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub rus_name: Option<String>,
    #[serde(default)]
    pub eng_name: Option<String>,
    pub model: String,
    pub slug: String,
    pub slug_url: String,
    pub cover: Cover,
    #[serde(rename = "ageRestriction")]
    pub age_restriction: LabeledId,
    pub site: i64,
    #[serde(rename = "type")]
    pub r#type: LabeledId,
    #[serde(default)]
    pub close_view: Option<i64>,
    #[serde(default)]
    pub is_licensed: Option<bool>,
    #[serde(default)]
    pub last_item_at: Option<String>,
    #[serde(default)]
    pub content_marking: Vec<ContentMarking>,
    #[serde(default)]
    pub metadata: Option<BookmarkMediaMetadata>,
    pub status: LabeledId,
    #[serde(default)]
    pub items_count: Option<ItemsCount>,
    #[serde(rename = "releaseDateString")]
    pub release_date_string: String,
    #[serde(default)]
    pub is_authorship: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub media_id: Option<i64>,
    pub item_id: i64,
    pub progress: String,
    pub status: i64,
    pub created_at: String,
    pub updated_at: String,
    pub meta: Option<Value>,
    pub media: BookmarkMedia,
    #[serde(default)]
    pub rating: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BookmarksMeta {
    pub current_page: i64,
    pub from: Option<i64>,
    pub path: String,
    #[serde(default)]
    pub per_page: Vec<i64>,
    pub to: Option<i64>,
    pub page: i64,
    #[serde(default)]
    pub next_page_url: Value,
}

#[derive(Debug, Deserialize)]
pub struct BookmarksResponse {
    pub data: Vec<Bookmark>,
    pub links: PageLinks,
    pub meta: BookmarksMeta,
}

/// The exact populated shape of a single manga's bookmark status is unconfirmed
/// (only observed as `null` for an anonymous/unauthenticated request), so it is
/// left untyped.
#[derive(Debug, Deserialize)]
pub struct MangaBookmarkResponse {
    pub data: Option<Value>,
}

// ============================================================================
// User Stats Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UserStatItem {
    pub value: i64,
    pub formated: String,
    pub short: String,
    pub label: String,
    pub percent: f64,
}

#[derive(Debug, Deserialize)]
pub struct UserStatSingle {
    pub value: i64,
    pub formated: String,
    pub short: String,
    pub label: String,
    #[serde(default)]
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserReviewsStat {
    #[serde(rename = "type", default)]
    pub kind: Vec<Value>,
    pub evaluation: Reviews,
}

#[derive(Debug, Deserialize)]
pub struct UserStats {
    #[serde(default)]
    pub genres: Vec<UserStatItem>,
    #[serde(default)]
    pub tags: Vec<UserStatItem>,
    pub manga_added: UserStatSingle,
    pub chapters_added: UserStatSingle,
    pub comments: UserStatSingle,
    pub reviews: UserReviewsStat,
}

#[derive(Debug, Deserialize)]
pub struct UserStatsResponse {
    pub data: UserStats,
}
