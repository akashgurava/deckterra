use std::fmt::Debug;
use std::str::from_utf8;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_urlencoded::to_string;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

pub async fn fetch<Q, T>(method: Method, url: String, query: Option<Q>) -> Result<T>
where
    Q: Serialize + Debug + Copy,
    T: DeserializeOwned,
{
    // Prepare request
    let mut url = Url::parse(&url).context(format!("Unable to parse url - {}", url))?;
    match query {
        Some(query) => {
            let query =
                &to_string(query).context(format!("Unable to parse query - {:?}", query))?;
            url.set_query(Some(query));
        }
        None => {}
    }
    let response = CLIENT
        .request(method.clone(), url)
        .send()
        .await
        .context("Sending Request Failed")?;

    // Process response
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
