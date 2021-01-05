use std::collections::HashMap;

use lordeckcodes::encoder::deck_from_code;

use super::decks::Deck;

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
pub struct Card {
    deck_code: String,
    code: String,
    set: u32,
    faction: u32,
    number: u32,
    count: i32,
}

impl Card {
    pub fn from_deck(deck: &Deck) -> Vec<Card> {
        let deck_code = &deck.deck_code();
        let cards = deck_from_code(deck_code).unwrap();
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
                    deck_code: deck_code.into(),
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
