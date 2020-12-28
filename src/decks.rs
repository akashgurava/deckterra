use log::trace;
use reqwest::{Client, Method};

use crate::models::*;

// Constants
const MAX_DECKS: u32 = 125_000;

const ENDPOINT_DECKS_LIBRARY: &str = "decks/library";

const DECK_FETCH_COUNT: u32 = 5000;
const DECK_DIV_COUNT: u32 = 4000;

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
    let num_requests = (total_decks as f64 / DECK_DIV_COUNT as f64).ceil() as u32;

    let count = (total_decks as f64 / num_requests as f64).ceil() as u32;
    // dbg!(num_requests);
    // dbg!(count);

    let requests = (0..num_requests)
        .map(|i| DeckQuery {
            sort_by,
            from: i * count,
            count: DECK_FETCH_COUNT,
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

        data.sort();
        data.dedup();
        data
    };
    // dbg!(data);
    println!("{}", data.len());
}
