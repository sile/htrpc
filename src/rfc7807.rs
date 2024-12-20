//! "Problem Details for HTTP APIs ([RFC 7807][RFC 7807])" related components.
//!
//! [RFC 7807]: https://tools.ietf.org/html/rfc7807
use serdeconv;
use std::error;
use std::fmt;
use trackable::Trackable;
use url::Url;

use types::HttpStatus;
use RpcResponse;

/// An RPC response that comforms [RFC 7807](RFC 7807).
///
/// This is created by calling the `Problem::into_response` method.
///
/// [RFC 7807]: https://tools.ietf.org/html/rfc7807
#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemResponse {
    status: Option<u16>,
    header: ProblemHeader,
    #[serde(skip)]
    body: Problem,
}
impl ProblemResponse {
    fn new(body: Problem) -> Self {
        ProblemResponse {
            status: Some(body.get_status_code()),
            header: ProblemHeader::new(),
            body,
        }
    }

    /// Returns the `Problem` instance of this response.
    pub fn problem(&self) -> &Problem {
        &self.body
    }

    /// Sets the `Problem` instance of this response.
    pub fn set_problem(&mut self, problem: Problem) {
        self.body = problem;
    }
}
impl RpcResponse for ProblemResponse {
    fn body(&mut self) -> Box<dyn AsRef<[u8]> + Send + 'static> {
        let json = serdeconv::to_json_string_pretty(&self.body).expect("Never fails");
        Box::new(json.into_bytes())
    }
    fn set_body(&mut self, body: Vec<u8>) {
        if let Ok(body) = serdeconv::from_json_slice(&body) {
            // TODO
            self.body = body;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProblemHeader {
    #[serde(rename = "content-type")]
    content_type: ContentTypeProblemJson,
}
impl ProblemHeader {
    pub fn new() -> Self {
        ProblemHeader {
            content_type: ContentTypeProblemJson,
        }
    }
}

/// Problem.
///
/// # Examples
///
/// `AboutBlankProblem`:
///
/// ```
/// extern crate htrpc;
/// extern crate serdeconv;
///
/// use htrpc::rfc7807::Problem;
/// use htrpc::types::HttpStatus;
///
/// # fn main() {
/// let problem = Problem::about_blank(HttpStatus::NotFound);
/// let http_body = serdeconv::to_json_string_pretty(&problem).unwrap();
/// assert_eq!(http_body, r#"{
///   "type": "about:blank",
///   "title": "Not Found",
///   "status": 404
/// }"#);
/// # }
/// ```
///
/// `TrackableProblem`:
///
/// ```
/// extern crate htrpc;
/// extern crate serdeconv;
/// extern crate trackable;
///
/// use htrpc::ErrorKind;
/// use htrpc::rfc7807::Problem;
/// use htrpc::types::HttpStatus;
/// use trackable::error::ErrorKindExt;
///
/// # fn main() {
/// let error = ErrorKind::Other.cause("something wrong");
/// let problem = Problem::trackable(HttpStatus::NotFound, error);
/// let http_body = serdeconv::to_json_string_pretty(&problem).unwrap();
/// assert_eq!(http_body, r#"{
///   "type": "https://docs.rs/htrpc/0.0.2/htrpc/rfc7807/struct.TrackableProblem.html",
///   "title": "Other (cause; something wrong)\nHISTORY:\n",
///   "status": 404,
///   "history": []
/// }"#);
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Problem {
    /// `type = "about:blank"`.
    #[serde(rename = "about:blank")]
    AboutBlank(AboutBlankProblem),

    /// `type = "https://docs.rs/htrpc/0.0.2/htrpc/rfc7807/struct.TrackableProblem.html"`.
    #[serde(rename = "https://docs.rs/htrpc/0.0.2/htrpc/rfc7807/struct.TrackableProblem.html")]
    Trackable(TrackableProblem),
}
impl Problem {
    /// Makes a new `AboutBlankProblem` problem.
    pub fn about_blank(status: HttpStatus) -> Self {
        Problem::AboutBlank(AboutBlankProblem::new(status))
    }

    /// Makes a new `TrackableProblem` problem.
    pub fn trackable<E>(status: HttpStatus, error: E) -> Self
    where
        E: error::Error + Trackable,
        E::Event: fmt::Display,
    {
        Problem::Trackable(TrackableProblem::new(status, error))
    }

    /// Converts into `ProblemResponse`.
    pub fn into_response(self) -> ProblemResponse {
        ProblemResponse::new(self)
    }

    fn get_status_code(&self) -> u16 {
        match *self {
            Problem::AboutBlank(ref p) => p.status,
            Problem::Trackable(ref p) => p.status,
        }
    }
}
impl Default for Problem {
    fn default() -> Self {
        Problem::about_blank(HttpStatus::InternalServerError)
    }
}
impl From<AboutBlankProblem> for Problem {
    fn from(f: AboutBlankProblem) -> Self {
        Problem::AboutBlank(f)
    }
}
impl From<TrackableProblem> for Problem {
    fn from(f: TrackableProblem) -> Self {
        Problem::Trackable(f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "application/problem+json")]
struct ContentTypeProblemJson;

/// "about:blank" problem type.
///
/// See: https://tools.ietf.org/html/rfc7807#section-4.2
#[derive(Debug, Serialize, Deserialize)]
pub struct AboutBlankProblem {
    /// The title of this problem.
    pub title: String,

    /// The status of this problem.
    pub status: u16,
}
impl AboutBlankProblem {
    /// Makes a new `AboutBlankProblem` instance.
    pub fn new(status: HttpStatus) -> Self {
        AboutBlankProblem {
            title: status.reason_phrase().to_string(),
            status: status.code(),
        }
    }
}

/// A problem type which represents trackable errors.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackableProblem {
    /// The title of this problem.
    pub title: String,

    /// The status of this problem.
    pub status: u16,

    /// The detail information of this problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub detail: Option<String>,

    /// The instance URI of this problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub instance: Option<Url>,

    /// The tracking history of this problem (this type specific field).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub history: Option<Vec<String>>,
}
impl TrackableProblem {
    /// Makes a new `TrackableProblem` instance.
    pub fn new<E>(status: HttpStatus, error: E) -> Self
    where
        E: error::Error + Trackable,
        E::Event: fmt::Display,
    {
        TrackableProblem {
            title: error.to_string(),
            status: status.code(),
            detail: error.source().map(|c| c.to_string()),
            instance: None,
            history: error
                .history()
                .map(|h| h.events().iter().map(|e| e.to_string()).collect()),
        }
    }
}
