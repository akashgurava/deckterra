#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

extern crate pretty_env_logger;

mod data;
mod moba;
mod utils;

use log::info;
use moba::decks::save_decks;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let app_start = Instant::now();

    info!("App Started!");

    save_decks(Some(10), None, None).await;

    info!("App completed in {} secs", app_start.elapsed().as_secs());
}
