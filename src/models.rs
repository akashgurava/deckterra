use std::{cmp::Ordering, collections::HashMap};

use chrono::{serde::ts_milliseconds, DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::deserialize::Queryable;
use lordeckcodes::encoder::deck_from_code;

use super::schema::decks;
use crate::utils::chain_ordering;

type DB = diesel::sqlite::Sqlite;

lazy_static! {
    static ref INT_TO_FACTION: HashMap<u32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, "DE");
        map.insert(1, "FR");
        map.insert(2, "IO");
        map.insert(3, "NX");
        map.insert(4, "PZ");
        map.insert(5, "SI");
        map.insert(6, "BW");
        map.insert(9, "MT");
        map
    };
}

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
    // TODO: Remove clones
    pub fn get_deck_cards(&self) -> Vec<Card> {
        let deck_code = self.deck_code.clone();
        let cards = deck_from_code(&deck_code).unwrap();
        let cards = cards.cards();

        cards
            .into_iter()
            .map(|card| {
                let set = card.card().set();
                let faction = card.card().faction();
                let number = card.card().number();

                let str_faction = INT_TO_FACTION.get(&faction).unwrap();
                let code = format!("{:0>2}{}{:0>3}", set, str_faction, number);
                Card {
                    deck_code: deck_code.clone(),
                    code,
                    set,
                    faction,
                    number,
                    count: card.count(),
                }
            })
            .collect::<Vec<_>>()

        // Card::multiple_from_card_and_count(Some(self.export.clone()), cards.cards())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    deck_code: String,
    code: String,
    set: u32,
    faction: u32,
    number: u32,
    count: i32,
}

// #[derive(Insertable)]
// #[table_name = "decks"]
// pub struct NewDeck<'a> {
//     pub uid: &'a str,
//     pub deck_code: &'a str,
//     pub title: &'a str,
//     pub description: &'a str,
//     pub rating: i32,
//     pub mode: &'a str,
//     pub playstyle: &'a str,
//     pub created_at: NaiveDateTime,
//     pub changed_at: NaiveDateTime,
//     pub is_private: bool,
//     pub is_draft: bool,
//     pub is_riot: bool,
// }

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeckData {
    has_next: bool,
    pub decks: Vec<Deck>,
}
