use log::info;

use crate::data::{
    cards::Card,
    decks::{Deck, DeckData},
};
use crate::utils::{build_request, fetch_multiple, write_file};

// Constants
const MAX_DECKS: u32 = 100_000;

const ENDPOINT_HOME: &str = "https://lor.mobalytics.gg/api/v2/";
const ENDPOINT_DECKS_LIBRARY: &str = "decks/library";

const DEFAULT_DECK_FETCH_COUNT: u32 = 5000;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeckCategory {
    Community,
    Budget,
    Featured,
}

impl Default for DeckCategory {
    fn default() -> Self {
        DeckCategory::Community
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum DeckSort {
    RecentlyUpdated,
    Hot,
    Popularity,
}

impl Default for DeckSort {
    fn default() -> Self {
        DeckSort::RecentlyUpdated
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct DeckUri {
    sort_by: Option<DeckSort>,
    from: Option<u32>,
    count: u32,
    category: Option<DeckCategory>,
}

impl Default for DeckUri {
    fn default() -> Self {
        DeckUri {
            sort_by: Some(DeckSort::default()),
            from: Some(0),
            count: DEFAULT_DECK_FETCH_COUNT,
            category: Some(DeckCategory::default()),
        }
    }
}

impl DeckUri {
    fn new(
        sort_by: Option<DeckSort>,
        from: Option<u32>,
        count: Option<u32>,
        category: Option<DeckCategory>,
    ) -> Self {
        DeckUri {
            sort_by,
            from,
            count: count.unwrap_or_else(|| DEFAULT_DECK_FETCH_COUNT),
            category,
        }
    }
}

#[allow(unused_variables)]
pub async fn get_decks(
    total_decks: Option<u32>,
    sort_by: Option<DeckSort>,
    category: Option<DeckCategory>,
) -> Vec<Deck> {
    let total_decks = total_decks.unwrap_or_else(|| MAX_DECKS);

    // We request ma20% more decks or 5000
    let extra_decks = (total_decks as f64 * 0.2).ceil() as u32;
    let total_decks = if extra_decks > DEFAULT_DECK_FETCH_COUNT {
        total_decks + DEFAULT_DECK_FETCH_COUNT
    } else {
        total_decks + extra_decks
    };
    let num_requests = (total_decks as f64 / DEFAULT_DECK_FETCH_COUNT as f64).ceil() as u32;

    let category = Some(category.unwrap_or_default());
    let sort_by = Some(sort_by.unwrap_or_default());

    let requests = (0..num_requests)
        .map(|i| {
            let count = if i < num_requests - 1 {
                DEFAULT_DECK_FETCH_COUNT
            } else {
                let rem = total_decks % DEFAULT_DECK_FETCH_COUNT;
                if rem == 0 {
                    DEFAULT_DECK_FETCH_COUNT
                } else {
                    rem
                }
            };
            let query = DeckUri::new(
                sort_by,
                Some(i * DEFAULT_DECK_FETCH_COUNT),
                Some(count),
                category,
            );
            build_request(
                "GET",
                &format!("{}{}", ENDPOINT_HOME, ENDPOINT_DECKS_LIBRARY),
                Some(query),
            )
        })
        .collect::<Vec<_>>();

    info!(
        "Sending {} requests for {} decks.",
        num_requests, total_decks
    );
    let data = {
        let mut data = fetch_multiple::<DeckData>(requests)
            .await
            .into_iter()
            .map(|x| x.unwrap_or_default())
            .map(|x| x.decks)
            .flatten()
            .collect::<Vec<_>>();

        data.sort();
        data.dedup();
        data
    };
    info!("Recieved {} decks.", data.len());
    data
}

pub async fn save_decks(
    total_decks: Option<u32>,
    sort_by: Option<DeckSort>,
    category: Option<DeckCategory>,
) {
    let decks = get_decks(total_decks, sort_by, category).await;
    write_file("decks.json", &decks);
    info!("Saved deck data at {}.", "decks.json");

    let cards = decks
        .iter()
        .map(|deck| Card::from_deck(deck))
        .flatten()
        .collect::<Vec<_>>();
    write_file("cards.json", &cards);
    info!("Saved card data at {}.", "cards.json");
}
