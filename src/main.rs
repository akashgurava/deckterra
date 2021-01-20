#![allow(dead_code, unused_imports)]

use std::time::{Duration, Instant};

use log::info;
use reqwest::{Client, Method, Request, Response, Url};

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

mod client;
mod request;

// use client::{Client, Method};
// use request::Request;
// use serde::{Deserialize, Serialize};
use tokio_stream as stream;
use tower::limit::{RateLimit, RateLimitLayer};
use tower::util::{CallAll, ServiceFn};
use tower::Service;
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = Client::new();
    let start = Instant::now();
    // #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    // struct Struct {}

    let svc = tower::service_fn(move |req: reqwest::Request| {
        log::info!("{}", &start.elapsed().as_secs());
        client.execute(req)
    });

    let mut svc: RateLimit<ServiceFn<_>> = tower::ServiceBuilder::new()
        .rate_limit(1, Duration::from_secs(3))
        .service(svc);

    let req = Request::new(
        Method::GET,
        Url::parse("https://httpbin.org/status/200").unwrap(),
    );

    let mut reqs = vec![
        req.try_clone().unwrap(),
        req.try_clone().unwrap(),
        req.try_clone().unwrap(),
    ];
    // let reqs = stream::iter(reqs);

    // let svc_gen = svc.ready_and();

    // reqs.into_iter()
    //     .map(|req| async move {
    //         &svc.ready_and();
    //         // .await.unwrap().call(req).await.unwrap();
    //     })
    //     .collect::<Vec<_>>();

    let mut count = 0;
    loop {
        if count == reqs.len() {
            log::info!("Execution complete");
            break;
        }

        let reqq = reqs.pop().unwrap();
        svc.ready_and().await.unwrap().call(reqq).await.unwrap();
        count += 1;
    }

    // let x = svc.ready_and().await.unwrap().call(req).await;

    // svc.call_all(reqs);
    // let alls = CallAll::new(svc, reqs);
    // alls.unordered();

    // let req1 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);
    // let req2 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);
    // let req3 = Request::<Struct>::new(&Method::Get, "https://httpbin.org/status/200", None);

    // client.fetch::<Struct, Struct>(req1).await;

    // client
    //     .fetch_multiple::<Struct, Struct>(vec![req3, req2])
    //     .await;
}

// async fn itte(reqs: Vec<Request>) {
//     reqs.iter()
//         .map(read_feed)
//         .collect::<FuturesUnordered<_>>()
//         .collect::<Vec<_>>()
//         .await
// }
