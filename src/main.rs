#[macro_use]
extern crate serde;

extern crate pretty_env_logger;

mod decks;
mod models;
mod utils;

use decks::get_decks;
use hyper::{client::HttpConnector, Body, Client};
use hyper_rustls::HttpsConnector;
use log::info;
use tokio::time::Instant;

pub type HyperClient = Client<HttpsConnector<HttpConnector>, Body>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let app_start = Instant::now();

    info!("App Started!");
    // Build Hyper Client with HTTPS support
    let _app_start = Instant::now();
    let https = HttpsConnector::with_native_roots();
    let client: HyperClient = Client::builder().build(https);

    get_decks(&client, Some(20000), None, None).await;

    info!("App completed in {} secs", app_start.elapsed().as_secs());
}
