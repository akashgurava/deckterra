use std::{cmp::Ordering, time::Duration, time::Instant};

use futures::stream::StreamExt;
use log::{debug, trace};
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::delay_for;

const ENDPOINT_HOME: &str = "https://lor.mobalytics.gg/api/v2/";

/// chain two orderings: the first one gets more priority
pub fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}

pub fn build_request<Q: Serialize>(
    client: &Client,
    method: &str,
    route: &str,
    query: Option<Q>,
) -> RequestBuilder {
    let url = Url::parse(ENDPOINT_HOME).unwrap().join(route).unwrap();
    let mut request = client.request(Method::from_bytes(method.as_bytes()).unwrap(), url);
    if query.is_some() {
        request = request.query(&query.unwrap());
    }
    let request = request;
    request
}

pub async fn fetch<T: DeserializeOwned>(request: RequestBuilder, identifier: usize) -> Option<T> {
    trace!("{} - Request Start", identifier);
    let start = Instant::now();

    let mut num_try: u8 = 1;
    // Max retries
    while num_try < 4 {
        // Clone Request before
        let request = request.try_clone().unwrap();
        let response = request.send().await;
        if response.is_ok() {
            let response = response.unwrap().json::<T>().await;
            if response.is_ok() {
                trace!(
                    "{}th - Request Complete in {}th try in {} secs",
                    identifier,
                    num_try,
                    start.elapsed().as_secs()
                );
                return Some(response.unwrap());
            } else {
                // Some error during decode
                trace!(
                    "{}th - Request - {}th Try - Unable to convert response data - {}",
                    identifier,
                    num_try,
                    response.err().unwrap()
                );
            }
        } else {
            trace!(
                "{}th - Request - {}th Try - Request Failed - {}",
                identifier,
                num_try,
                response.err().unwrap()
            );
            trace!("Request failed.");
        }
        delay_for(Duration::from_millis(1500)).await;
        num_try += 1;
    }
    debug!(
        "{}th - Request Failed after {} tries. Time spent - {} secs",
        identifier,
        num_try,
        start.elapsed().as_secs()
    );
    None

    // debug!("request with query {:?} failed", query.unwrap());
}

pub async fn fetch_multiple<T: DeserializeOwned>(requests: Vec<RequestBuilder>) -> Vec<Option<T>> {
    let queries = tokio::stream::iter(requests);
    let dur = Duration::from_millis(250); // 1 request per 250ms
    let max_concurrent_req = 8usize;

    let data = tokio::time::throttle(dur, queries)
        .enumerate()
        .map(|(mut i, request)| async move {
            i += 1;
            if i > max_concurrent_req {
                delay_for(Duration::from_secs(8)).await;
            }
            crate::utils::fetch::<T>(request, i).await
        })
        // Send max of 5 requests at a time
        .buffered(max_concurrent_req)
        .collect::<Vec<_>>()
        .await;
    data
}
