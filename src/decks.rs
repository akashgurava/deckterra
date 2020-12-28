use log::trace;
use reqwest::{Client, Method};

// Constants
const MAX_DECKS: u32 = 125_000;

const ENDPOINT_DECKS_LIBRARY: &str = "decks/library";

const DECK_DEFAULT_SORT_BY: DeckSort = DeckSort::RecentlyUpdated;
const DECK_DEFAULT_START: u32 = 0;
const DECK_DEFAULT_COUNT: u32 = 5000;
const DECK_REQ_DIV: u32 = 4000;
const DECK_DEFAULT_CATEGORY: DeckCategory = DeckCategory::Community;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeckCategory {
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "BUDGET")]
    Budget,
    #[serde(rename = "FEATURED")]
    Featured,
}

impl Default for DeckCategory {
    fn default() -> Self {
        DeckCategory::Community
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeckSort {
    #[serde(rename = "recently_updated")]
    RecentlyUpdated,
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "popularity")]
    Popularity,
}

impl Default for DeckSort {
    fn default() -> Self {
        DeckSort::RecentlyUpdated
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckQuery {
    #[serde(rename(serialize = "sortBy"))]
    pub sort_by: DeckSort,
    pub from: u32,
    pub count: u32,
    pub category: DeckCategory,
}

impl Default for DeckQuery {
    fn default() -> Self {
        DeckQuery {
            sort_by: DECK_DEFAULT_SORT_BY,
            from: DECK_DEFAULT_START,
            count: DECK_DEFAULT_COUNT,
            category: DECK_DEFAULT_CATEGORY,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Deck {
    uid: String,
    title: String,
    description: String,
    #[serde(rename = "exportUID")]
    export: String,
    mode: String,
    #[serde(rename = "playStyle")]
    playstyle: String,
    created_at: u64,
    changed_at: u64,
    rating: i32,
    is_private: bool,
    is_draft: bool,
    is_riot: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct DeckData {
    has_next: bool,
    decks: Vec<Deck>,
}

pub async fn get_decks(
    client: &Client,
    category: Option<DeckCategory>,
    total_decks: Option<u32>,
    sort_by: Option<DeckSort>,
) {
    let category = category.unwrap_or_default();
    // We request 20% more decks
    let total_decks = ((total_decks.unwrap_or_else(|| MAX_DECKS)) as f64 * 1.2).ceil() as u64;
    let sort_by = sort_by.unwrap_or_default();
    let num_requests = (total_decks as f64 / DECK_REQ_DIV as f64).ceil() as u32;

    let count = (total_decks as f64 / num_requests as f64).ceil() as u32;
    // dbg!(num_requests);
    // dbg!(count);

    let requests = (0..num_requests)
        .map(|i| DeckQuery {
            sort_by,
            from: i * count,
            count,
            category,
        })
        .map(|query| {
            crate::utils::build_request(
                client,
                Method::GET.as_str(),
                ENDPOINT_DECKS_LIBRARY,
                Some(query),
            )
        })
        .collect::<Vec<_>>();

    trace!("Sending {} requests.", num_requests);

    let data = {
        let mut data = crate::utils::fetch_multiple::<DeckData>(requests)
            .await
            .into_iter()
            .map(|x: Option<DeckData>| x.unwrap_or_default())
            .map(|x| x.decks)
            .flatten()
            .collect::<Vec<_>>();

        data.sort_by(|a, b| {
            crate::utils::chain_ordering(a.uid.cmp(&b.uid), a.export.cmp(&b.export))
        });
        data.dedup_by(|a, b| (a.uid == b.uid) & (a.export == b.export));
        data
    };
    // dbg!(data);
    println!("{}", data.len());
}
