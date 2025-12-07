use kai_fansubs::client::Client;
use kai_fansubs::queries::{MediaQuery, SubtitleNoteQuery};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let result = MediaQuery::new(4214).execute(&client).await;
    dbg!(&result);

    let result = SubtitleNoteQuery::new(10917).execute(&client).await;
    dbg!(&result);
}
