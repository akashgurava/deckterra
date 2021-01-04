use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use crate::models::Deck;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn read() {
    use crate::schema::decks::dsl::*;

    let connection = establish_connection();
    let results: Vec<Deck> = decks
        .limit(5)
        .load(&connection)
        .expect("Error loading decks");
    dbg!(results);
}
