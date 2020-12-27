use reqwest::{Client, Method};

// Constants
const MAX_DECKS: u32 = 125_000;

const ENDPOINT_DECKS_LIBRARY: &str = "decks/library";

// TODO: Change SORT_BY and CATEGORY to enums once
//      we identify all possible values
const DECK_DEFAULT_SORT_BY: DeckSort = DeckSort::RecentlyUpdated;
const DECK_DEFAULT_START: u32 = 0;
const DECK_DEFAULT_COUNT: u32 = 5000;
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DeckSort {
    #[serde(rename = "recently_updated")]
    RecentlyUpdated,
    #[serde(rename = "hot")]
    Hot,
    #[serde(rename = "popularity")]
    Popularity,
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
    rating: i8,
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
    let category = category.unwrap_or_else(|| DeckCategory::Community);
    let total_decks = total_decks.unwrap_or_else(|| MAX_DECKS);
    let sort_by = sort_by.unwrap_or_else(|| DeckSort::RecentlyUpdated);

    let num_requests = (total_decks as f64 / DECK_DEFAULT_COUNT as f64).ceil() as u32;

    let queries: Vec<DeckQuery> = (0..num_requests)
        .map(|i| {
            if i < num_requests - 1 {
                DeckQuery {
                    sort_by,
                    from: i * DECK_DEFAULT_COUNT,
                    count: DECK_DEFAULT_COUNT,
                    category,
                }
            } else {
                let rem = total_decks % DECK_DEFAULT_COUNT;
                DeckQuery {
                    sort_by,
                    from: i * DECK_DEFAULT_COUNT,
                    count: if rem == 0 { DECK_DEFAULT_COUNT } else { rem },
                    category,
                }
            }
        })
        .collect();

    let data: Vec<DeckData> = crate::utils::fetch_multiple(
        client,
        Method::GET.as_str(),
        ENDPOINT_DECKS_LIBRARY,
        queries,
    )
    .await
    .into_iter()
    .map(|x: Option<DeckData>| x.unwrap_or_default())
    .collect();
    let data: Vec<Deck> = data.into_iter().map(|x| x.decks).flatten().collect();

    // dbg!(data);
    println!("{}", data.len());
    // let responses: Vec<&Response> = responses
    //     .iter()
    //     .filter_map(|response| response.as_ref().ok())
    //     .map(async |response| response.json().await)
    //     .collect();
    // responses.iter().map(|&response| match response {
    //     Ok(response) => response,
    //     _ => (),
    // });
    // .map(|&response| match response {
    //     reqwest::Response => response,
    //     reqwest::Error => (),
    // })
    // response
}
