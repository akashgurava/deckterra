use std::{cmp::Ordering, fs::OpenOptions, io::BufWriter, time::Duration};

use futures_util::StreamExt;
use hyper::{client::HttpConnector, Body, Client, Request, Uri};
use hyper_rustls::HttpsConnector;
use log::{error, info, trace, warn};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::{sleep, Instant};
use tokio_stream as stream;

type HyperClient = Client<HttpsConnector<HttpConnector>, Body>;

lazy_static! {
    // Build Hyper Client with HTTPS support
    static ref CLIENT: HyperClient = {
        let _app_start = Instant::now();
        let https = HttpsConnector::with_native_roots();
        Client::builder().build(https)
    };
}

/// chain two orderings: the first one gets more priority
pub fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}

pub async fn fetch<T: DeserializeOwned>(uri: Uri, identifier: usize) -> Option<T> {
    trace!("{}th request - Start.", identifier);
    let start = Instant::now();

    let mut num_try: u8 = 1;
    while num_try < 4 {
        // Building a request should not be any issue
        let request = Request::builder()
            .uri(uri.clone())
            .body(Body::empty())
            .unwrap();
        let response = CLIENT.request(request).await;
        trace!(
            "{}th request - {}th try - Recieved response: {}",
            identifier,
            num_try,
            start.elapsed().as_secs()
        );
        // TODO: do more error handling
        if response.is_ok() {
            let body = hyper::body::to_bytes(response.unwrap()).await;
            if body.is_ok() {
                let data: Result<T, _> = serde_json::from_slice(&body.unwrap());
                if data.is_ok() {
                    info!(
                        "{}th request - {}th try - Completed in {} secs",
                        identifier,
                        num_try,
                        start.elapsed().as_secs()
                    );
                    return Some(data.unwrap());
                } else {
                    let err = data.err().unwrap();
                    warn!(
                        "{}th request - {}th try - Convert to struct failed: {}",
                        identifier,
                        num_try,
                        err.to_string()
                    );
                }
            } else {
                let err = body.err().unwrap();
                warn!(
                    "{}th request - {}th try - Parsing response failed: {}",
                    identifier,
                    num_try,
                    err.to_string()
                );
            }
        } else {
            let err = response.err().unwrap();
            warn!(
                "{}th request - {}th try - Sending Request Failed: {}",
                identifier,
                num_try,
                err.to_string()
            );
        }
        sleep(Duration::from_millis(1500)).await;
        num_try += 1;
    }
    error!(
        "{}th request - Failed - Uri: {}",
        identifier,
        uri.to_string()
    );
    None
}

pub async fn fetch_multiple<T: DeserializeOwned>(uris: Vec<Uri>) -> Vec<Option<T>> {
    let uris = stream::iter(uris);
    let dur = Duration::from_millis(250); // 1 request per 250ms
    let max_concurrent_req = 6usize;

    let data = stream::StreamExt::throttle(uris, dur)
        .enumerate()
        .map(|(mut i, uri)| async move {
            i += 1;
            if i > max_concurrent_req {
                trace!("{}th request - Sleeping for 8 secs", i);
                sleep(Duration::from_secs(8)).await;
            }
            fetch::<T>(uri, i).await
        })
        // Send max of 5 requests at a time
        .buffered(max_concurrent_req)
        .collect::<Vec<_>>()
        .await;
    data
}

pub fn write_file<T>(path: &str, data: &T)
where
    T: ?Sized + Serialize,
{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut file = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut file, &data).unwrap();
}
