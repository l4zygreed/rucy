use crate::bot::modules::Module;
use crate::bot::modules::User;
use mongodb::Database;
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;

pub fn init(db: Database) -> Module<impl Future<Output = Option<Box<dyn User>>>> {
    Module {
        name: "picarto",
        db: db.collection("picarto"),
        url: "https://picarto.tv/",
        fetch_user: fetch,
        map: HashMap::new(),
    }
}

async fn fetch(name: String) -> Option<Box<dyn User>> {
    let resp = get(format!("https://api.picarto.tv/v1/channel/name/{}", name))
        .await
        .ok()?
        .json::<Picarto>()
        .await
        .ok()?;

    Some(Box::new(resp))
}

#[derive(Deserialize)]
struct Picarto {
    name: String,
    online: bool,
    avatar: String,
    // thumbnails:
}

impl User for Picarto {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn avatar(&self) -> String {
        self.avatar.to_owned()
    }

    fn online(&self) -> bool {
        self.online
    }
}
