mod bot;
mod db;

use bot::config;

#[tokio::main]
async fn main() {
    let db = db::init().await;
    let conf = config::get_config();

    bot::init(conf, db).await;
}
