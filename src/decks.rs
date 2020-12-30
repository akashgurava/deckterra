use log::info;
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
    let total_decks = ((total_decks.unwrap_or_else(|| MAX_DECKS)) as f64 * 1.2).ceil() as u32;
    let sort_by = sort_by.unwrap_or_default();
    let num_requests = (total_decks as f64 / DECK_DIV_COUNT as f64).ceil() as u32;

    // dbg!(num_requests);
    // dbg!(count);

    let requests = (0..num_requests)
        .map(|i| {
            let count = if i < num_requests - 1 {
                DECK_FETCH_COUNT
            } else {
                let rem = total_decks % DECK_DIV_COUNT;
                if rem == 0 {
                    DECK_FETCH_COUNT
                } else {
                    rem
                }
            };
            DeckQuery {
                sort_by,
                from: i * DECK_DIV_COUNT,
                count,
                category,
            }
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

    info!(
        "Sending {} requests for {} decks.",
        num_requests, total_decks
    );
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
    info!("Recieved {} decks.", data.len());
}
