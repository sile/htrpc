//! `Deserializer` trait implementations.
//!
//! Note that this module has been exported only for the documentation purpose.
//! It is not intended that this module is used by users explicitly.
pub use self::http_body::HttpBodyDeserializer;
pub use self::http_header::HttpHeaderDeserializer;
pub use self::rpc_request::RpcRequestDeserializer;
pub use self::rpc_response::RpcResponseDeserializer;
pub use self::url_query::UrlQueryDeserializer;
pub use self::url_path::UrlPathDeserializer;

mod http_body;
mod http_header;
mod rpc_request;
mod rpc_response;
mod url_path;
mod url_query;
