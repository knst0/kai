use kai_fansubs::client::Client;
use kai_fansubs::queries::{AlphabetCatalogQuery, CatalogSymbol};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    for symbol in CatalogSymbol::iter() {
        let result = AlphabetCatalogQuery::new(symbol).execute(&client).await?;

        for media in result {
            println!("{:5}: {}", media.id, media.title);
        }
    }

    Ok(())
}
