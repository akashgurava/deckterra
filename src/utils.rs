use std::error::Error;
use std::time::Duration;

use futures::stream::StreamExt;

use log::trace;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::delay_for;

const ENDPOINT_HOME: &str = "https://lor.mobalytics.gg/api/v2/";

pub async fn fetch<Q: Serialize, R: DeserializeOwned>(
    client: &Client,
    method: &str,
    route: &str,
    query: Option<Q>,
) -> Option<R> {
    let url = &format!("{}{}", ENDPOINT_HOME, route);
    let mut request = client.request(Method::from_bytes(method.as_bytes()).unwrap(), url);
    if query.is_some() {
        request = request.query(&query.unwrap());
    }
    let mut data: Option<R> = None;

    let mut try_count: u8 = 1;
    while try_count < 4 {
        let req_clone = request.try_clone().unwrap();
        let response = req_clone.send().await;
        if response.is_ok() {
            let req_data = response.unwrap().json::<R>().await;
            if req_data.is_ok() {
                data.replace(req_data.unwrap());
                break;
            } else {
                // Some error during decode
                let req_err: reqwest::Error = req_data.err().unwrap();
                let req_err = req_err.source().unwrap();
                trace!(
                    "{}th Try: Unable to convert response data: {}",
                    try_count,
                    req_err
                );
            }
        } else {
            trace!("Request failed.");
        }
        delay_for(Duration::from_millis(1500)).await;
        try_count += 1;
    }
    // debug!("request with query {:?} failed", query.unwrap());
    data
}

pub async fn fetch_multiple<Q: Serialize, R: DeserializeOwned>(
    client: &Client,
    method: &str,
    route: &str,
    queries: Vec<Q>,
) -> Vec<Option<R>> {
    let queries = tokio::stream::iter(queries);
    let dur = Duration::from_millis(250); // 1 request per 250ms

    let data: Vec<Option<R>> = tokio::time::throttle(dur, queries)
        .map(|query| async {
            crate::utils::fetch::<Q, R>(client, method, route, Some(query)).await
        })
        // Send max of 5 requests at a time
        .buffered(5)
        .collect()
        .await;
    data
}
