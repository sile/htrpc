//! [RFC 7807](https://tools.ietf.org/html/rfc7807).
use serde::{Serialize, Deserialize};

use RpcResponse;
use types::HttpStatus;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse<T>
    where T: Serialize + for<'a> Deserialize<'a>
{
    status: u16,
    header: ProblemHeader,
    #[serde(with = "::json_pretty")]
    body: T,
}
impl<T> ErrorResponse<T>
    where T: Serialize + for<'a> Deserialize<'a>
{
    pub fn new(status: HttpStatus, body: T) -> Self {
        ErrorResponse {
            status: status.code(),
            header: ProblemHeader::new(),
            body: body,
        }
    }
}
impl<T> RpcResponse for ErrorResponse<T> where T: Serialize + for<'a> Deserialize<'a> {}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemHeader {
    #[serde(rename = "Content-Type")]
    content_type: ContentTypeProblemJson,
}
impl ProblemHeader {
    pub fn new() -> Self {
        ProblemHeader { content_type: ContentTypeProblemJson }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "application/problem+json")]
struct ContentTypeProblemJson;

/// https://tools.ietf.org/html/rfc7807#section-4.2
#[derive(Debug, Serialize, Deserialize)]
pub struct AboutBlankProblem {
    #[serde(rename = "type")]
    type_uri: String,
    title: String,
    status: u16,
}
impl AboutBlankProblem {
    /// Makes a new `AboutBlankProblem` instance.
    pub fn new(status: HttpStatus) -> Self {
        AboutBlankProblem {
            type_uri: "about:blank".to_string(),
            title: status.reason_phrase().to_string(),
            status: status.code(),
        }
    }
}
