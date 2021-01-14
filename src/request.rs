use serde::Serialize;

use crate::client::Method;

#[derive(Clone, Copy)]
struct Request<'a, Q>
where
    Q: Serialize,
{
    method: &'a Method,
    url: &'a str,
    query: Option<&'a Q>,
}
