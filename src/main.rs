#[macro_use]
extern crate serde;

extern crate pretty_env_logger;

mod decks;
mod utils;

use std::time::Instant;

use log::debug;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let app_start = Instant::now();
    pretty_env_logger::init();
    let client = Client::new();
    crate::decks::get_decks(&client, None, Some(50000), None).await;
    debug!("App completed in {} secs", app_start.elapsed().as_secs());
}
