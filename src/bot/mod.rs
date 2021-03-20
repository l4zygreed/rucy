pub mod commands;
pub mod config;
mod modules;

use config::Configuration;
use mongodb::Database;
use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub async fn init(config: Configuration, db: Database) {
    println!("Starting Bot");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.prefix))
        .group(&commands::GENERAL_GROUP);

    let mut client = Client::builder(config.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    modules::init(db).await;
    panic!("END");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
