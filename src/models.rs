use std::{cmp::Ordering, collections::HashMap};

use chrono::{serde::ts_milliseconds, DateTime, Utc};
use lordeckcodes::encoder::deck_from_code;

use crate::utils::chain_ordering;

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

// impl Card {
//     pub fn from_card_and_count(deck_code: Option<String>, card: &CardCodeAndCount) -> Self {
//         let set = card.card().set();
//         let faction = card.card().faction();
//         let number = card.card().number();

//         let str_faction = INT_TO_FACTION.get(&faction).unwrap();
//         let code = format!("{:0>2}{}{:0>3}", set, str_faction, number);
//         Card {
//             deck_code,
//             code,
//             set,
//             faction,
//             number,
//             count: card.count(),
//         }
//     }
//     pub fn multiple_from_card_and_count(
//         deck_code: Option<String>,
//         cards: &Vec<CardCodeAndCount>,
//     ) -> Vec<Self> {
//         cards
//             .iter()
//             .map(|card| Card::from_card_and_count(deck_code.clone(), card))
//             .collect()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeckData {
    has_next: bool,
    decks: Vec<Deck>,
}

impl DeckData {
    /// Consumes self and return decks
    pub fn decks(self) -> Vec<Deck> {
        self.decks
    }
}
