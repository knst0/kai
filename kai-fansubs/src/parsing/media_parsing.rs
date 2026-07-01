use crate::error::Error;
use crate::parsing::selectors::*;
use crate::types::{Author, Link, SubtitleEntry};
use scraper::{Element, ElementRef, Html};

pub fn parse_subtitles(html: &str) -> Result<Vec<SubtitleEntry>, Error> {
    let document = Html::parse_fragment(html);

    let table_headers: Vec<_> = document.select(&SUBTITLE_TABLE_SELECTOR).collect();

    let mut subtitles = Vec::new();

    for table in table_headers {
        let id = table
            .select(&SUBTITLE_INPUT_SELECTOR)
            .next()
            .ok_or_else(|| Error::ParseError("Failed to parse subtitle entry".to_string()))?
            .value()
            .attr("value")
            .ok_or_else(|| Error::ParseError("Missing srt value".to_string()))?;

        let columns = table.select(&TD_ROW3_SELECTOR).collect::<Vec<_>>();

        let title = columns
            .get(2)
            .map(|td| td.text().collect::<String>().trim().to_string())
            .ok_or_else(|| Error::ParseError("Missing title column".to_string()))?;

        let format = columns
            .get(3)
            .map(|td| td.text().collect::<String>().trim().to_string())
            .ok_or_else(|| Error::ParseError("Missing format column".to_string()))?;

        let date = columns
            .get(4)
            .map(|td| td.text().collect::<String>().trim().to_string())
            .ok_or_else(|| Error::ParseError("Missing date column".to_string()))?;

        let has_note = columns
            .get(5)
            .map(|td| td.inner_html().contains(r#"href="base.php?note="#))
            .unwrap_or(false);

        let mut authors = Vec::new();
        let mut current = table.next_sibling_element();

        while let Some(next_element) = current {
            if next_element.value().name() == "table"
                && next_element.value().attr("width") == Some("100%")
                && !next_element.value().classes().any(|c| c == "row1")
            {
                current = next_element.next_sibling_element();
                continue;
            }

            if next_element.value().name() == "table"
                && next_element.value().classes().any(|c| c == "row1")
            {
                if let Some(author) = parse_author_from_table(&next_element)? {
                    authors.push(author);
                }
                current = next_element.next_sibling_element();
            } else {
                break;
            }
        }

        let subtitle_id = id
            .parse::<u32>()
            .map_err(|e| Error::ParseError(format!("Invalid subtitle ID: {}", e)))?;

        subtitles.push(SubtitleEntry { id: subtitle_id, title, format, date, has_note, authors });
    }

    Ok(subtitles)
}

pub fn parse_author_from_table(table: &ElementRef) -> Result<Option<Author>, Error> {
    let td = match table.select(&AUTHOR_TD_SELECTOR).next() {
        Some(td) => td,
        None => return Ok(None),
    };

    let content = td.inner_html();
    let text = td.text().collect::<String>();

    if text.trim().is_empty() {
        return Ok(None);
    }

    let role = extract_role(&text);
    let (id, name) = extract_author_info(&content);
    let (team_url, team) = extract_team_info(&content);

    if name.is_empty() {
        return Ok(None);
    }

    Ok(Some(Author { id, name, role, team, team_url }))
}

pub fn extract_role(text: &str) -> Option<String> {
    const ROLES: [&str; 11] = [
        "Переводчик",
        "Редактор",
        "Оформление",
        "Тайм-код",
        "Перевод песен",
        "Редактор/Оформление",
        "Редактор/Тайм-код",
        "Редактор/Перевод песен",
        "Переводчик/Оформление",
        "Редактор/Тайм-код/Оформление",
        "Редактор/Таймкод",
    ];

    ROLES.iter().find(|&&role| text.contains(&format!("{}:", role))).map(|&role| role.to_string())
}

pub fn extract_author_info(html: &str) -> (Option<u32>, String) {
    let document = Html::parse_fragment(html);

    if let Some(b) = document.select(&AUTHOR_LINK_SELECTOR).next() {
        let name = b.text().collect::<String>().trim().to_string();
        if let Some(link) = b.parent().and_then(|p| p.value().as_element()) {
            let id = link
                .attr("href")
                .and_then(|href| href.strip_prefix("base.php?au="))
                .and_then(|id_str| id_str.parse().ok());
            return (id, name);
        }
    }

    let name = document
        .select(&B_TAG_SELECTOR)
        .find_map(|b| {
            let text = b.text().collect::<String>().trim().to_string();
            const SKIP: [&str; 4] = ["Переводчик", "Редактор", "Оформление", "Тайм-код"];
            (!text.is_empty() && !SKIP.iter().any(|&s| text.contains(s))).then_some(text)
        })
        .unwrap_or_default();

    (None, name)
}

pub fn extract_team_info(html: &str) -> (Option<String>, Option<String>) {
    let document = Html::parse_fragment(html);

    document
        .select(&TEAM_LINK_SELECTOR)
        .next()
        .and_then(|link| {
            link.value().attr("href").map(|href| {
                let name = link.text().collect::<String>().trim().to_string();
                (Some(href.to_string()), (!name.is_empty()).then_some(name))
            })
        })
        .unwrap_or((None, None))
}

pub struct InformationBlock {
    pub main_title: String,
    pub alternative_names: Vec<String>,
    pub general_info: Option<String>,
    pub poster_url: Option<String>,
    pub links: Vec<Link>,
}

pub fn parse_information_block(
    document: &Html,
    site_url_prefix: &str,
) -> Result<InformationBlock, Error> {
    let table = document
        .select(&INFO_TABLE_SELECTOR)
        .next()
        .ok_or_else(|| Error::ParseError("Missing information table".to_string()))?;

    let trs = table.select(&TR_SELECTOR).collect::<Vec<_>>();

    let main_title = trs
        .first()
        .ok_or_else(|| Error::ParseError("Missing title row".to_string()))?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let poster_url = trs
        .get(2)
        .and_then(|tr| tr.select(&POSTER_IMG_SELECTOR).next())
        .and_then(|img| img.attr("src"))
        .map(|src| format!("{site_url_prefix}/{src}"));

    let elements = document.select(&BLOCKQUOTE_TITLE_SELECTOR).collect::<Vec<_>>();

    let mut alternative_names: Vec<String> = Vec::new();
    let mut general_info: Option<String> = None;
    let mut links: Vec<Link> = Vec::new();

    for el in elements {
        let blockquote = el.next_sibling().and_then(|node| {
            if node.value().is_element()
                && node.value().as_element().unwrap().name() == "blockquote"
            {
                Some(ElementRef::wrap(node).unwrap())
            } else {
                let mut current = node;
                loop {
                    current = current.next_sibling()?;
                    if let Some(elem) = ElementRef::wrap(current)
                        && elem.value().name() == "blockquote"
                    {
                        return Some(elem);
                    }
                }
            }
        });

        if let Some(blockquote) = blockquote {
            match el.text().collect::<String>().trim() {
                "Альтернативные названия:" => {
                    alternative_names = blockquote
                        .text()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
                "Общая информация:" => {
                    general_info = Some(blockquote.text().collect::<String>().trim().to_string());
                }
                "Ссылки:" => {
                    links = blockquote
                        .select(&ANCHOR_WEB_SELECTOR)
                        .filter_map(|link| {
                            link.value()
                                .attr("href")
                                .filter(|href| href.starts_with("http"))
                                .map(|href| Link {
                                    title: link.text().collect::<String>().trim().to_string(),
                                    url: href.to_string(),
                                })
                                .filter(|l| !l.title.is_empty())
                        })
                        .collect();
                }
                _ => {}
            }
        }
    }

    Ok(InformationBlock { main_title, alternative_names, general_info, poster_url, links })
}
