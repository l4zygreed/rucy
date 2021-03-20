use json::{parse, JsonValue};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Configuration {
    pub prefix: &'static str,
    pub token: &'static str,
}

impl From<JsonValue> for Configuration {
    fn from(jv: JsonValue) -> Self {
        Configuration {
            prefix: Box::leak(jv["prefix"].to_string().into_boxed_str()),
            token: Box::leak(jv["token"].to_string().into_boxed_str()),
        }
    }
}

pub fn get_config() -> Configuration {
    let path = Path::new("config/config.json");

    let mut file = File::open(&path).expect("couldn't open config.json");

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("could not read config.json");

    parse(&s).unwrap().into()
}
