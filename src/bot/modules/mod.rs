pub mod picarto;

use mongodb::{bson::Bson, Collection, Database};
use std::collections::HashMap;
use std::future::Future;
// use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Module<F>
where
    F: Future<Output = Option<Box<dyn User>>>,
{
    map: HashMap<String, Sender<bool>>,
    name: &'static str,
    db: Collection,
    url: &'static str,
    fetch_user: fn(String) -> F,
}

impl<F> Module<F>
where
    F: Future<Output = Option<Box<dyn User>>>,
{
    fn new(db: Database, name: &'static str, url: &'static str, f: fn(String) -> F) -> Self {
        Self {
            name,
            url,
            db: db.collection(name),
            map: HashMap::new(),
            fetch_user: f,
        }
    }

    fn add(&mut self, name: String) {
        match self.map.get(&name) {
            Some(thread) => {} // TODO
            None => {
                let (tx, rx) = mpsc::channel(1);
                self.map.insert(name.clone(), tx);

                // let f = self.fetch_user.clone();
                // tokio::spawn(async move { gorutine(f, name, rx) });

                // let s = self.clone();
                // tokio::spawn(async move { s.gorutine(name, rx) });
                let n = name.clone().to_owned();
                tokio::spawn(async move {
                    loop {
                        if let Some(user) = ((self.fetch_user)(name.clone())).await {
                            println!("{} {}", user.name(), user.online());
                        }
                    }
                    // (self.fetch_user)(n);
                });
            }
        }

        // println!("{}", self.name);
    }

    fn kill(&self, name: String) {}

    // fn gorutine(&self, name: String, rx: Receiver<bool>) {
    //     loop {
    //         if let Some(user) = (self.fetch_user)(name.clone()).await {
    //             println!("{} {}", user.name(), user.online());
    //         }
    //     }
    // }
}

// impl IModule for Module {
//     fn new(
//         db: Database,
//         name: &'static str,
//         url: &'static str,
//         f: fn(String) -> Pin<Box<dyn Future<Output = Option<Box<dyn User>>>>>,
//     ) -> Self {
//         Self {
//             name,
//             url,
//             db: db.collection(name),
//             fetch_user: f,
//             map: HashMap::new(),
//         }
//     }

//     fn add(&mut self, name: String) {
//         match self.map.get(&name) {
//             Some(thread) => {} // TODO
//             None => {
//                 let (tx, rx) = mpsc::channel(1);
//                 self.map.insert(name.clone(), tx);

//                 // let f = self.fetch_user.clone();
//                 // tokio::spawn(async move { gorutine(f, name, rx) });

//                 let s = self.clone();
//                 tokio::spawn(async move { s.gorutine(name, rx) });
//             }
//         }

//         // println!("{}", self.name);
//     }

//     fn kill(&self, name: String) {}

//     fn gorutine(&self, name: String, rx: Receiver<bool>) {
//         loop {
//             if let Some(user) = (self.fetch_user)(name.clone()) {
//                 println!("{} {}", user.name(), user.online());
//             }
//         }
//     }
// }

pub async fn init(db: Database) {
    let mut modules: Vec<Module<_>> = Vec::new();

    modules.push(picarto::init(db));

    for module in modules.iter_mut() {
        let names: Vec<Bson> = module.db.distinct("nick", None, None).await.unwrap();

        for name in names.into_iter() {
            let name = name.as_str().unwrap().to_string();

            module.add(name);
        }
    }
}

pub trait User {
    fn name(&self) -> String;
    fn avatar(&self) -> String;
    fn online(&self) -> bool;
}

async fn gorutine(s: fn(String) -> Option<Box<dyn User>>, name: String, rx: Receiver<bool>) {}
