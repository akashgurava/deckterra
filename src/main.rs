#[macro_use]
extern crate serde;

mod decks;
mod models;
mod utils;

use std::time::Instant;

use decks::get_decks;
use log::info;
use reqwest::Client;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let app_start = Instant::now();

    info!("App Started!");
    let client = Client::new();
    get_decks(&client, None, Some(20000), None).await;
    info!("App completed in {} secs", app_start.elapsed().as_secs());
}
