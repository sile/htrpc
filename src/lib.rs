extern crate futures;
extern crate miasht;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trackable;
extern crate url;

pub use miasht::{Method, Status};
pub use error::{Error, ErrorKind};

pub mod client;
pub mod server;
pub mod procedure;

pub mod path_template;
pub mod path_template_ser;

mod error;

pub type Result<T> = ::std::result::Result<T, Error>;
