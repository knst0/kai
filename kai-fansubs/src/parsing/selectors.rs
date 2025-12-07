use scraper::Selector;
use std::sync::LazyLock;

// Catalog selectors
pub static CATALOG_LINK_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"a[href^="base.php?id="]"#).unwrap());

// Media query selectors
pub static SUBTITLE_TABLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"table[width="750"]:has(> form)"#).unwrap());

pub static SUBTITLE_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"input[name="srt"]"#).unwrap());

pub static TD_ROW3_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td.row3").unwrap());

pub static AUTHOR_TD_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"td[align="center"][valign="middle"]:not([class])"#).unwrap());

pub static AUTHOR_LINK_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"a[href^="base.php?au="] b"#).unwrap());

pub static B_TAG_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("b").unwrap());

pub static TEAM_LINK_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"a[target="web"][href^="http"]"#).unwrap());

pub static INFO_TABLE_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table.row1[width=\"100%\"]").unwrap());

pub static TR_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tr").unwrap());

pub static POSTER_IMG_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td.row2 > img").unwrap());

pub static BLOCKQUOTE_TITLE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("td > b").unwrap());

pub static ANCHOR_WEB_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"a[target="web"]"#).unwrap());

// Subtitle notes selectors
pub static NOTE_DIV_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"div[align="justify"]"#).unwrap());
