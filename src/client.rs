use serde::Serialize;
use tokio::time::Duration;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Method {
    Get,
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => reqwest::Method::GET,
            // _ => reqwest::Method::GET,
        }
    }
}

pub struct Client {
    client: reqwest::Client,
    retries: u8,
    throttle: Duration,
    concurrency: u32,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            client: reqwest::Client::new(),
            retries: 3,
            throttle: Duration::from_millis(1500),
            concurrency: 5,
        }
    }
}

impl Client {
    pub async fn fetch<Q>(&self, method: Method, url: String, query: Option<Q>)
    where
        Q: Serialize,
        //     T: DeserializeOwned,
    {
        // // Prepare request
        // let mut url = Url::parse(&url).context(format!("Unable to parse url - {}", url))?;
        // match query {
        //     Some(query) => {
        //         let query =
        //             &to_string(query).context(format!("Unable to parse query - {:?}", query))?;
        //         url.set_query(Some(query));
        //     }
        //     None => {}
        // }
        // let response = CLIENT
        //     .request(method.clone(), url)
        //     .send()
        //     .await
        //     .context("Sending Request Failed")?;

        // // Process response
        // let status = response.status();
        // let body = response.bytes().await.context("Parsing response failed")?;
        // let data = if status.is_success() {
        //     serde_json::from_slice::<T>(&body).context("Deserialize to required struct failed")?
        // } else {
        //     let error = from_utf8(&body).context("Deserialize to error struct failed")?;
        //     return Err(anyhow!("Error response - {}", error));
        // };

        // Ok(data)
        let mut request = self.client.request(method.into(), &url);
        request = match query {
            Some(query) => request.query(&query),
            None => request,
        };
        let _response = request.build();
        match _response.err() {
            None => {}
            Some(error) => {
                if error.is_builder() {
                    println!("builder error")
                }
                if error.is_connect() {
                    println!("connect error")
                }
                if error.is_decode() {
                    println!("decode error")
                }
                if error.is_redirect() {
                    println!("redirect error")
                }
                if error.is_request() {
                    println!("request error")
                }
                if error.is_status() {
                    println!("status error")
                }
                if error.is_timeout() {
                    println!("timeout error")
                }
                println!("{}", error.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn url_error() {
        #[derive(Serialize)]
        struct Y {}
        let client = Client::default();
        client
            .fetch::<Y>(Method::Get, "https://googlr.com".into(), None)
            .await;
    }
}
