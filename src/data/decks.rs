use std::cmp::Ordering;

use chrono::{serde::ts_milliseconds, DateTime, Utc};

use crate::utils::chain_ordering;

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
