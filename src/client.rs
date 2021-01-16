use std::str::from_utf8;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use log::info;
use reqwest::{Client as ReqClient, Method as ReqMethod};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::{sleep, Duration, Instant};
use tokio_stream as stream;

use crate::request::Request;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

impl From<&Method> for ReqMethod {
    fn from(method: &Method) -> Self {
        match method {
            Method::Get => ReqMethod::GET,
            // _ => reqwest::Method::GET,
        }
    }
}

pub struct Client {
    client: ReqClient,
    max_tries: u8,
    throttle: Duration,
    concurrency: usize,
    error_backoff: Vec<Duration>,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            client: ReqClient::new(),
            max_tries: 5,
            throttle: Duration::from_millis(1500),
            concurrency: 5,
            error_backoff: vec![
                Duration::from_millis(1200),
                Duration::from_millis(3000),
                Duration::from_millis(6000),
                Duration::from_millis(9000),
            ],
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    async fn try_fetch<Q, T>(&self, method: &Method, url: &str, query: Option<&Q>) -> Result<T>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let mut request = self.client.request(method.into(), url);
        request = match query {
            Some(query) => request.query(query),
            None => request,
        };
        let response = request.send().await?;
        let status = response.status();
        let body = response.bytes().await.context("Parsing response failed")?;
        let data = if status.is_success() {
            serde_json::from_slice::<T>(&body).context("Deserialize to required struct failed")?
        } else {
            let error = from_utf8(&body).context("Unable to parse response body as string")?;
            return Err(anyhow!("Error response - {}", error));
        };
        Ok(data)
    }

    pub async fn fetch<Q, T>(&self, request: Request<'_, Q>) -> Option<T>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let method = request.method();
        let url = request.url();
        let query = request.query();

        let start = Instant::now();
        for attempt in 1..=self.max_tries {
            let response = self.try_fetch::<Q, T>(method, url, query).await;
            match response {
                Ok(data) => {
                    info!(
                        "{}th attempt > Success > Time elapsed {} secs",
                        attempt,
                        start.elapsed().as_secs()
                    );
                    return Some(data);
                }
                Err(err) => {
                    if attempt < self.max_tries {
                        warn!(
                            "{}th attempt failed > Reason - {}",
                            attempt,
                            err.to_string(),
                        );
                        let dur = self.error_backoff.get((attempt - 1) as usize).unwrap();
                        sleep(dur.clone()).await;
                    } else {
                        error!(
                            "{}th attempt failed > Reason - {}",
                            attempt,
                            err.to_string(),
                        );
                    }
                }
            }
        }
        None
    }

    async fn fetch_single<Q, T>(
        &self,
        identifier: usize,
        method: &Method,
        url: &str,
        query: Option<&Q>,
    ) -> Option<T>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let start = Instant::now();
        for attempt in 1..=self.max_tries {
            let response = self.try_fetch::<Q, T>(method, url, query).await;
            match response {
                Ok(data) => {
                    info!(
                        "{}th request > {}th attempt > Success > Time elapsed {} secs",
                        identifier,
                        attempt,
                        start.elapsed().as_secs()
                    );
                    return Some(data);
                }
                Err(err) => {
                    if attempt < self.max_tries {
                        warn!(
                            "{}th request > {}th attempt failed > Reason - {}",
                            identifier,
                            attempt,
                            err.to_string(),
                        );
                        let dur = self.error_backoff.get((attempt - 1) as usize).unwrap();
                        sleep(dur.clone()).await;
                    } else {
                        error!(
                            "{}th request > {}th attempt failed > Reason - {}",
                            identifier,
                            attempt,
                            err.to_string(),
                        );
                    }
                }
            }
        }
        None
    }

    pub async fn fetch_multiple<Q, T>(&self, requests: Vec<Request<'_, Q>>) -> Vec<Option<T>>
    where
        Q: Serialize,
        T: DeserializeOwned,
    {
        let requests = stream::iter(requests);
        let data = stream::StreamExt::throttle(requests, self.throttle)
            .enumerate()
            .map(|(mut identifier, request)| async move {
                identifier += 1;
                let method = request.method();
                let url = request.url();
                let query = request.query();

                self.fetch_single::<_, T>(identifier, method, url, query)
                    .await
            })
            .buffered(self.concurrency)
            .collect::<Vec<_>>()
            .await;

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct TestHW {
        msg: String,
    }

    #[tokio::test]
    async fn simple_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hello"))
            .respond_with(ResponseTemplate::new(200).set_body_json(TestHW {
                msg: "hello world".into(),
            }))
            // Mounting the mock on the mock server - it's now effective!
            .mount(&server)
            .await;
        let client = Client::new();
        let uri = format!("{}/hello", &server.uri());
        let request = Request::<TestHW>::new(&Method::Get, &uri, None);

        let data = client.fetch::<TestHW, TestHW>(request).await;

        assert_eq!(data.is_some(), true);
        assert_eq!(
            data.unwrap(),
            TestHW {
                msg: "hello world".into()
            }
        );
    }

    #[tokio::test]
    async fn simple_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hello"))
            .respond_with(ResponseTemplate::new(500).set_body_json(TestHW {
                msg: "Error".into(),
            }))
            // Mounting the mock on the mock server - it's now effective!
            .mount(&server)
            .await;
        let client = Client::new();
        let uri = format!("{}/hello", &server.uri());
        let request = Request::<TestHW>::new(&Method::Get, &uri, None);

        let data = client.fetch::<TestHW, TestHW>(request).await;

        assert_eq!(data.is_none(), true);
    }
}
