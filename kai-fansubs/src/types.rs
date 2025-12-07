#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct CatalogMedia {
    /// Unique identifier for the media
    pub id: u32,
    /// Title of the media
    pub title: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Author {
    /// Unique identifier for the author (may be None for guest contributors)
    pub id: Option<u32>,
    /// Name of the author
    pub name: String,
    /// Role of the author (e.g., "Переводчик", "Редактор", "Оформление", etc.)
    pub role: Option<String>,
    /// Name of the team or group the author belongs to
    pub team: Option<String>,
    /// URL link to the team's page or profile
    pub team_url: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct SubtitleEntry {
    /// Unique identifier for the subtitle
    pub id: u32,
    /// Title or description of the subtitle
    pub title: String,
    /// Format of the subtitle file (e.g., "ASS", "SRT", "VTT")
    pub format: String,
    /// Date when the subtitle was created or uploaded
    pub date: String,
    /// Whether this subtitle has additional notes available
    pub has_note: bool,
    /// List of authors who contributed to this subtitle
    pub authors: Vec<Author>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Link {
    /// Display title for the link
    pub title: String,
    /// The actual URL
    pub url: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct MediaDetails {
    /// Unique identifier for the media
    pub id: u32,
    /// Primary title of the media
    pub main_title: String,
    /// List of alternative names (e.g., original language title, romanized title)
    pub alternative_names: Vec<String>,
    /// General information or description about the media
    pub general_info: Option<String>,
    /// URL to the poster or cover image
    pub poster_url: Option<String>,
    /// External links related to this media
    pub links: Vec<Link>,
    /// List of available subtitles for this media
    pub subtitles: Vec<SubtitleEntry>,
}
