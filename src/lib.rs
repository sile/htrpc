extern crate futures;
extern crate miasht;
extern crate serde;
extern crate trackable;

pub use miasht::{Method, Status};
pub use error::{Error, ErrorKind};

pub mod client;
pub mod server;
pub mod procedure;

mod error;

pub type Result<T> = ::std::result::Result<T, Error>;
