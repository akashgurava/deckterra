// #![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod client;

use client::{Client, Method};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = Client::new();
    #[derive(Serialize, Clone, Deserialize, PartialEq, Eq, Debug)]
    struct Struct {}

    client
        .fetch::<Struct, Struct>(Method::Get, "https://httpbin.org/status/200".into(), None)
        .await;
}
