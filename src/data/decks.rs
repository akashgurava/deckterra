use std::cmp::Ordering;

use chrono::{serde::ts_milliseconds, DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::deserialize::Queryable;

use super::schema::decks;
use crate::utils::chain_ordering;

type DB = diesel::sqlite::Sqlite;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    uid: String,
    #[serde(rename(deserialize = "exportUID"))]
    deck_code: String,
    title: String,
    description: String,
    rating: i32,
    mode: String,
    #[serde(rename(deserialize = "playStyle"))]
    playstyle: String,
    #[serde(with = "ts_milliseconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    changed_at: DateTime<Utc>,
    is_private: bool,
    is_draft: bool,
    is_riot: bool,
}

impl Eq for Deck {}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        (self.uid == other.uid) & (self.deck_code == other.deck_code)
    }
}

impl Ord for Deck {
    fn cmp(&self, other: &Self) -> Ordering {
        chain_ordering(
            self.uid.cmp(&other.uid),
            self.deck_code.cmp(&other.deck_code),
        )
    }
}

impl PartialOrd for Deck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Queryable<decks::SqlType, DB> for Deck {
    type Row = (
        i32,
        String,
        String,
        String,
        String,
        i32,
        String,
        String,
        NaiveDateTime,
        NaiveDateTime,
        bool,
        bool,
        bool,
    );

    fn build(row: Self::Row) -> Self {
        fn change(dt: NaiveDateTime) -> DateTime<Utc> {
            Utc.from_utc_datetime(&dt)
        }

        Deck {
            uid: row.1,
            deck_code: row.2,
            title: row.3,
            description: row.4,
            rating: row.5,
            mode: row.6,
            playstyle: row.7,
            created_at: change(row.8),
            changed_at: change(row.9),
            is_private: row.10,
            is_draft: row.11,
            is_riot: row.12,
        }
    }
}

impl Deck {
    pub fn deck_code(&self) -> String {
        self.deck_code.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeckData {
    has_next: bool,
    pub(crate) decks: Vec<Deck>,
}
