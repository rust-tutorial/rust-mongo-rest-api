use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

#[derive(Clone)]
pub struct MongoDB {
    pub db: Database,
}

pub async fn connect(name: String, uri: String, db: String) -> MongoDB {
    let mut client_options = ClientOptions::parse(uri).await.expect("Error parsing URI");
    client_options.app_name = Some(name);
    let client = Client::with_options(client_options).expect("Error client option");
    // Get a handle to a database.
    let db = client.database(&db);
    MongoDB { db }
}
