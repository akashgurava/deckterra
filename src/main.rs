// #![allow(dead_code)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod client;
mod request;

use client::{Client, Method};
use request::Request;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = Client::new();
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Struct {}

    let req1 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);
    let req2 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);
    let req3 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);

    client.fetch::<Struct, Struct>(req1).await;

    client
        .fetch_multiple::<Struct, Struct>(vec![req3, req2])
        .await;
}
