// #![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod client;
mod request;

use client::{Client, Method};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = Client::new();
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Struct {}

    client
        .fetch::<Struct, Struct>(Method::Get, "https://httpbin.org/status/200".into(), None)
        .await;

    client
        .fetch_multiple::<Struct, Struct>(
            Method::Get,
            "https://httpbin.org/status/200".into(),
            vec![None, None],
        )
        .await;
}
