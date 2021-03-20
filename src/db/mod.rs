use mongodb::{options::ClientOptions, Client, Database};

// struct Streamer {
//     id: mongodb::bson::oid::ObjectId,
//     nick: String,
//     channel: String,
//     message: String,
// }

// struct Poster {
//     id: mongodb::bson::oid::ObjectId,
//     nick: String,
//     channel: String,
//     last: u64,
// }

pub async fn init() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    client.database("lucy")
}
