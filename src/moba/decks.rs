use hyper::Uri;
use log::info;
use serde_qs;

use crate::{
    models::{Deck, DeckData},
    utils::{fetch_multiple, write_file},
};

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
    #[allow(dead_code)]
    fn new(count: Option<u32>) -> Self {
        DeckUri {
            count: count.unwrap_or_else(|| DEFAULT_DECK_FETCH_COUNT),
            ..DeckUri::default()
        }
    }

    #[allow(dead_code)]
    fn create(
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

impl ToString for DeckUri {
    fn to_string(&self) -> String {
        let query = serde_qs::to_string(&self).unwrap();
        format!("{}{}?{}", ENDPOINT_HOME, ENDPOINT_DECKS_LIBRARY, query)
    }
}

// Todo: Change this to TryFrom
impl From<DeckUri> for Uri {
    fn from(value: DeckUri) -> Self {
        value.to_string().parse().unwrap()
    }
}

#[allow(unused_variables)]
pub async fn get_decks(
    total_decks: Option<u32>,
    sort_by: Option<DeckSort>,
    category: Option<DeckCategory>,
) -> Vec<Deck> {
    let total_decks = total_decks.unwrap_or_else(|| MAX_DECKS);
    let num_requests = (total_decks as f64 / DEFAULT_DECK_FETCH_COUNT as f64).ceil() as u32;

    let category = Some(category.unwrap_or_default());
    let sort_by = Some(sort_by.unwrap_or_default());

    let request_uris = (0..num_requests)
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
            Uri::from(DeckUri::create(
                sort_by,
                Some(i * DEFAULT_DECK_FETCH_COUNT),
                Some(count),
                category,
            ))
        })
        .collect::<Vec<_>>();

    info!(
        "Sending {} requests for {} decks.",
        num_requests, total_decks
    );
    let data = {
        let mut data = fetch_multiple::<DeckData>(request_uris)
            .await
            .into_iter()
            .map(|x| x.unwrap_or_default())
            .map(|x| x.decks())
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
        .map(|deck| deck.get_deck_cards())
        .flatten()
        .collect::<Vec<_>>();
    write_file("cards.json", &cards);
    info!("Saved card data at {}.", "cards.json");
}

#[cfg(test)]
mod tests {

    use super::{DeckCategory, DeckSort, DeckUri, Uri};

    #[test]
    fn test_build_uri_from_new() {
        let deck = DeckUri::new(Some(1000));
        assert_eq!(
            Uri::from(deck),
            "https://lor.mobalytics.gg/api/v2/decks/library?count=1000"
        )
    }

    #[test]
    fn test_build_uri_from_all() {
        let deck = DeckUri::create(
            Some(DeckSort::Hot),
            Some(1000),
            Some(4000),
            Some(DeckCategory::Community),
        );
        assert_eq!(
            Uri::from(deck),
            "https://lor.mobalytics.gg/api/v2/decks/library?sortBy=hot&from=1000&count=4000&category=COMMUNITY"
        )
    }
}
