use reqwest::Error as ReqError;
use serde_urlencoded::ser::Error as UrlSerError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum DecktError {
    #[error("Unable to build request - Reason > {msg}")]
    RequestError { msg: String },
    #[error("An unknown error occured.")]
    Unknown,
}

impl From<UrlSerError> for DecktError {
    fn from(error: UrlSerError) -> Self {
        DecktError::RequestError {
            msg: error.to_string(),
        }
    }
}

impl From<ParseError> for DecktError {
    fn from(error: ParseError) -> Self {
        DecktError::RequestError {
            msg: error.to_string(),
        }
    }
}

impl From<ReqError> for DecktError {
    fn from(error: ReqError) -> Self {
        DecktError::RequestError {
            msg: error.to_string(),
        }
    }
}
