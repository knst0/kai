use kai_mangalib::client::ClientBuilder;
use kai_mangalib::queries::{MangaField, MangaQuery};

#[tokio::main]
async fn main() {
    let mut builder = ClientBuilder::new();

    if let Ok(token) = std::env::var("API_TOKEN") {
        builder = builder.api_token(token);
    }

    let client = builder.build();

    let response = MangaQuery::new("247--shingeki-no-kyojin")
        .with_field(MangaField::Background)
        .execute(&client)
        .await;

    if let Ok(Some(manga)) = response {
        dbg!(&manga);
    }
}
