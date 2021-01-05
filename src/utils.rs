use std::{cmp::Ordering, fs::OpenOptions, io::BufWriter, str::from_utf8, time::Duration};

use anyhow::{Context, Result};
use futures_util::StreamExt;
use log::{error, info, trace, warn};
use reqwest::{Client, Method, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::{sleep, Instant};
use tokio_stream as stream;

lazy_static! {
    // Build Hyper Client with HTTPS support
    pub static ref CLIENT: Client = Client::new();
}

pub fn build_request<Q: Serialize>(method: &str, url: &str, query: Option<Q>) -> RequestBuilder {
    let url = Url::parse(url).unwrap();
    let mut request = CLIENT.request(Method::from_bytes(method.as_bytes()).unwrap(), url);
    if query.is_some() {
        request = request.query(&query.unwrap());
    }
    request
}

/// chain two orderings: the first one gets more priority
pub fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
    match o1 {
        Ordering::Equal => o2,
        _ => o1,
    }
}

pub async fn fetch<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send().await.context("Sending Request Failed")?;
    let status = response.status();
    let body = response.bytes().await.context("Parsing response failed")?;
    let data = if status.is_success() {
        serde_json::from_slice::<T>(&body).context("Deserialize to required struct failed")?
    } else {
        let error = from_utf8(&body).context("Deserialize to error struct failed")?;
        return Err(anyhow!("Error response - {}", error));
    };

    Ok(data)
}

pub async fn fetch_multiple<T: DeserializeOwned>(requests: Vec<RequestBuilder>) -> Vec<Option<T>> {
    let requests = stream::iter(requests);
    let dur = Duration::from_millis(1500); // 1 request per 250ms
    let max_concurrent_req = 6;
    let retries = 3;

    let data = stream::StreamExt::throttle(requests, dur)
        .enumerate()
        .map(|(mut i, request)| async move {
            i += 1;
            let start = Instant::now();
            let mut data: Option<T> = None;

            for num_try in 1..=retries {
                if i > max_concurrent_req {
                    trace!("{}th request - Sleeping for 8 secs", i);
                    sleep(Duration::from_secs(8)).await;
                }
                match fetch::<T>(request.try_clone().unwrap()).await {
                    Ok(resp_data) => {
                        info!("{}th request - {}th try - Completed.", i, num_try,);
                        data.replace(resp_data);
                        break;
                    }
                    Err(err) => {
                        println!("{}", start.elapsed().as_secs());
                        if num_try == retries {
                            error!(
                                "{}th request - {}th try - {}",
                                i,
                                num_try,
                                err.to_string(),
                            );
                        } else {
                            warn!("{}th request - {}th try - {}", i, num_try, err.to_string(),);
                            sleep(Duration::from_secs(2 * num_try)).await;
                        };
                    }
                }
            }
            data
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
