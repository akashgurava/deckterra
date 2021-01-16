use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::client::Method;

#[derive(Debug, TypedBuilder)]
pub struct Request<'a, Q>
where
    Q: Serialize,
{
    method: &'a Method,
    url: &'a str,
    query: Option<&'a Q>,
}

impl<'a, Q> Request<'a, Q>
where
    Q: Serialize,
{
    pub fn new(method: &'a Method, url: &'a str, query: Option<&'a Q>) -> Self {
        Request { method, url, query }
    }

    pub fn method(&self) -> &Method {
        self.method
    }

    pub fn url(&self) -> &str {
        self.url
    }

    pub fn query(&self) -> Option<&Q> {
        self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Debug, Clone, Copy)]
    struct TestReq {
        start: u32,
        count: u32,
    }

    #[test]
    fn test_request_new() {
        Request::<TestReq>::new(&Method::Get, &"JNJS", None);
        Request::<TestReq>::new(
            &Method::Get,
            &"JNJS",
            Some(&TestReq {
                start: 10,
                count: 100,
            }),
        );
    }

    #[test]
    fn test_request_builder() {
        Request::<TestReq>::builder()
            .url("hu")
            .method(&Method::Get)
            .query(None)
            .build();
    }
}
