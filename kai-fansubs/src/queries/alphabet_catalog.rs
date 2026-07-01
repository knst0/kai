use crate::client::Client;
use crate::error::Error;
use crate::parsing::selectors::CATALOG_LINK_SELECTOR;
use crate::types::CatalogMedia;
use encoding_rs::WINDOWS_1251;
use reqwest::Method;
use scraper::Html;

/// Represents a catalog symbol for browsing media alphabetically.
///
/// This enum contains all available letters and symbols that can be used
/// to browse the media catalog. It supports both English (A-Z) and
/// Cyrillic (А-Я) letters, as well as a special hash symbol (#) for
/// media starting with numbers or special characters.
///
/// # Examples
///
/// ```rust
/// use kai_fansubs::queries::CatalogSymbol;
///
/// let symbol = CatalogSymbol::A;
/// println!("Symbol URL encoded: {}", symbol.as_url_encoded());
///
/// // Navigate through symbols
/// if let Some(next) = symbol.next() {
///     println!("Next symbol: {:?}", next);
/// }
///
/// // Iterate through all symbols
/// for sym in CatalogSymbol::iter() {
///     println!("Available symbol: {:?}", sym);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogSymbol {
    /// Special symbol for media starting with numbers or special characters
    Hash,
    /// English letter A
    A,
    /// English letter B
    B,
    /// English letter C
    C,
    /// English letter D
    D,
    /// English letter E
    E,
    /// English letter F
    F,
    /// English letter G
    G,
    /// English letter H
    H,
    /// English letter I
    I,
    /// English letter J
    J,
    /// English letter K
    K,
    /// English letter L
    L,
    /// English letter M
    M,
    /// English letter N
    N,
    /// English letter O
    O,
    /// English letter P
    P,
    /// English letter Q
    Q,
    /// English letter R
    R,
    /// English letter S
    S,
    /// English letter T
    T,
    /// English letter U
    U,
    /// English letter V
    V,
    /// English letter W
    W,
    /// English letter X
    X,
    /// English letter Y
    Y,
    /// English letter Z
    Z,
    /// Cyrillic letter А
    CyrillicA,
    /// Cyrillic letter Б
    CyrillicB,
    /// Cyrillic letter В
    CyrillicV,
    /// Cyrillic letter Г
    CyrillicG,
    /// Cyrillic letter Д
    CyrillicD,
    /// Cyrillic letter Е
    CyrillicE,
    /// Cyrillic letter Ж
    CyrillicZh,
    /// Cyrillic letter З
    CyrillicZ,
    /// Cyrillic letter И
    CyrillicI,
    /// Cyrillic letter К
    CyrillicK,
    /// Cyrillic letter Л
    CyrillicL,
    /// Cyrillic letter М
    CyrillicM,
    /// Cyrillic letter Н
    CyrillicN,
    /// Cyrillic letter О
    CyrillicO,
    /// Cyrillic letter П
    CyrillicP,
    /// Cyrillic letter Р
    CyrillicR,
    /// Cyrillic letter С
    CyrillicS,
    /// Cyrillic letter Т
    CyrillicT,
    /// Cyrillic letter У
    CyrillicU,
    /// Cyrillic letter Ф
    CyrillicF,
    /// Cyrillic letter Х
    CyrillicH,
    /// Cyrillic letter Ц
    CyrillicTs,
    /// Cyrillic letter Ч
    CyrillicCh,
    /// Cyrillic letter Ш
    CyrillicSh,
    /// Cyrillic letter Щ
    CyrillicShch,
    /// Cyrillic letter Э
    CyrillicE2,
    /// Cyrillic letter Ю
    CyrillicYu,
    /// Cyrillic letter Я
    CyrillicYa,
}

impl CatalogSymbol {
    const ALL: &'static [Self] = &[
        Self::Hash,
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        Self::CyrillicA,
        Self::CyrillicB,
        Self::CyrillicV,
        Self::CyrillicG,
        Self::CyrillicD,
        Self::CyrillicE,
        Self::CyrillicZh,
        Self::CyrillicZ,
        Self::CyrillicI,
        Self::CyrillicK,
        Self::CyrillicL,
        Self::CyrillicM,
        Self::CyrillicN,
        Self::CyrillicO,
        Self::CyrillicP,
        Self::CyrillicR,
        Self::CyrillicS,
        Self::CyrillicT,
        Self::CyrillicU,
        Self::CyrillicF,
        Self::CyrillicH,
        Self::CyrillicTs,
        Self::CyrillicCh,
        Self::CyrillicSh,
        Self::CyrillicShch,
        Self::CyrillicE2,
        Self::CyrillicYu,
        Self::CyrillicYa,
    ];

    /// Converts the catalog symbol to its URL-encoded string representation for API requests.
    pub fn as_url_encoded(&self) -> &'static str {
        match self {
            Self::Hash => ".",
            Self::A => "a",
            Self::B => "b",
            Self::C => "c",
            Self::D => "d",
            Self::E => "e",
            Self::F => "f",
            Self::G => "g",
            Self::H => "h",
            Self::I => "i",
            Self::J => "j",
            Self::K => "k",
            Self::L => "l",
            Self::M => "m",
            Self::N => "n",
            Self::O => "o",
            Self::P => "p",
            Self::Q => "q",
            Self::R => "r",
            Self::S => "s",
            Self::T => "t",
            Self::U => "u",
            Self::V => "v",
            Self::W => "w",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::CyrillicA => "%E0",
            Self::CyrillicB => "%E1",
            Self::CyrillicV => "%E2",
            Self::CyrillicG => "%E3",
            Self::CyrillicD => "%E4",
            Self::CyrillicE => "%E5",
            Self::CyrillicZh => "%E6",
            Self::CyrillicZ => "%E7",
            Self::CyrillicI => "%E8",
            Self::CyrillicK => "%EA",
            Self::CyrillicL => "%EB",
            Self::CyrillicM => "%EC",
            Self::CyrillicN => "%ED",
            Self::CyrillicO => "%EE",
            Self::CyrillicP => "%EF",
            Self::CyrillicR => "%F0",
            Self::CyrillicS => "%F1",
            Self::CyrillicT => "%F2",
            Self::CyrillicU => "%F3",
            Self::CyrillicF => "%F4",
            Self::CyrillicH => "%F5",
            Self::CyrillicTs => "%F6",
            Self::CyrillicCh => "%F7",
            Self::CyrillicSh => "%F8",
            Self::CyrillicShch => "%F9",
            Self::CyrillicE2 => "%FD",
            Self::CyrillicYu => "%FE",
            Self::CyrillicYa => "%FF",
        }
    }

    /// Returns the next catalog symbol in alphabetical order.
    ///
    /// This method follows the order: Hash → A-Z → А-Я
    pub fn next(&self) -> Option<Self> {
        let current_pos = Self::ALL.iter().position(|s| s == self)?;
        Self::ALL.get(current_pos + 1).copied()
    }

    /// Returns the previous catalog symbol in alphabetical order.
    ///
    /// This method follows the order: Hash ← A-Z ← А-Я
    pub fn prev(&self) -> Option<Self> {
        let current_pos = Self::ALL.iter().position(|s| s == self)?;
        if current_pos > 0 { Self::ALL.get(current_pos - 1).copied() } else { None }
    }

    /// Returns an iterator over all catalog symbols in alphabetical order.
    ///
    /// The iterator yields symbols in the order: Hash → A-Z → А-Я
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.iter().copied()
    }
}

/// Query for browsing the media catalog by alphabetical symbol.
///
/// This query allows you to browse media organized alphabetically by their
/// first letter. It supports both English (A-Z) and Cyrillic (А-Я) letters,
/// as well as a special symbol for media starting with numbers or special characters.
///
/// # Examples
///
/// ```rust
/// use kai_fansubs::client::Client;
/// use kai_fansubs::queries::{AlphabetCatalogQuery, CatalogSymbol};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new();
///
/// // Browse media starting with "A"
/// let catalog = AlphabetCatalogQuery::new(CatalogSymbol::A)
///     .execute(&client)
///     .await?;
///
/// for media in &catalog {
///     println!("ID: {}, Title: {}", media.id, media.title);
/// }
///
/// if catalog.is_empty() {
///     println!("No media found for this symbol");
/// }
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct AlphabetCatalogQuery {
    symbol: CatalogSymbol,
}

impl AlphabetCatalogQuery {
    /// Creates a new alphabet catalog query for the specified symbol.
    pub fn new(symbol: CatalogSymbol) -> Self {
        Self { symbol }
    }

    /// Executes the alphabet catalog query and returns media for the specified symbol.
    pub async fn execute(&self, client: &Client) -> Result<Vec<CatalogMedia>, Error> {
        let builder = client
            .init_request(Method::GET, &format!("/base.php?l={}", self.symbol.as_url_encoded()));
        let response = client.execute_built(builder).await?;
        let bytes = response.bytes().await?;
        let (html, _, _) = WINDOWS_1251.decode(&bytes);

        let document = Html::parse_document(&html);

        let mut media_list = Vec::new();

        for element in document.select(&CATALOG_LINK_SELECTOR) {
            if let Some(href) = element.value().attr("href")
                && let Some(id_str) = href.strip_prefix("base.php?id=")
                && let Ok(id) = id_str.parse::<u32>()
            {
                let title = element.text().collect::<String>().trim().to_string();
                if !title.is_empty() {
                    media_list.push(CatalogMedia { id, title });
                }
            }
        }

        Ok(media_list)
    }
}
