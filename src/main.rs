#[macro_use]
extern crate serde;

extern crate pretty_env_logger;

mod decks;
mod utils;

use reqwest::Client;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = Client::new();
    crate::decks::get_decks(&client, None, Some(20000), None).await;
}
