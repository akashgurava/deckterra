use std::{cmp::Ordering, time::Duration, time::Instant};

use futures::stream::StreamExt;
use log::{debug, error, trace};
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
        trace!(
            "{}th request - {}th try - Recieved response: {}",
            identifier,
            num_try,
            start.elapsed().as_secs()
        );
        if response.is_ok() {
            let response = response.unwrap().json::<T>().await;
            if response.is_ok() {
                debug!(
                    "{}th request - {}th try - Completed in {} secs",
                    identifier,
                    num_try,
                    start.elapsed().as_secs()
                );
                return Some(response.unwrap());
            } else {
                // Some error during decode
                let err = response.err().unwrap();
                debug!(
                    "{}th request - {}th try - Convert to struct failed: {}",
                    identifier,
                    num_try,
                    err.to_string()
                );
            }
        } else {
            let err = response.err().unwrap();
            debug!(
                "{}th request - {}th try - Sending Request Failed: {}",
                identifier,
                num_try,
                err.to_string()
            );
        }
        delay_for(Duration::from_millis(1500)).await;
        num_try += 1;
    }
    error!(
        "{}th request - Failed - Uri: {}",
        identifier,
        request
            .try_clone()
            .unwrap()
            .build()
            .unwrap()
            .url()
            .to_string()
    );
    None

    // debug!("request with query {:?} failed", query.unwrap());
}

pub async fn fetch_multiple<T: DeserializeOwned>(requests: Vec<RequestBuilder>) -> Vec<Option<T>> {
    let queries = tokio::stream::iter(requests);
    let dur = Duration::from_millis(250); // 1 request per 250ms
    let max_concurrent_req = 6usize;

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
