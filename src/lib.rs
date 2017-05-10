extern crate fibers;
extern crate futures;
extern crate handy_async;
extern crate miasht;
#[macro_use]
extern crate serde;
extern crate serdeconv;
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

pub mod deserializers;
pub mod path_template;
pub mod serializers;

mod error;

pub type Result<T> = ::std::result::Result<T, Error>;
