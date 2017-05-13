//! HTTP based RPC library.
//!
//! This crate provides a thin framework to easily implement type-safe RPC channels
//! for client/server model communication.
#![warn(missing_docs)]
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

// TODO
#[macro_export]
macro_rules! htrpc_entry_point {
    ($($segment:tt),*) => {
        {
            static SEGMENTS: &[$crate::types::PathSegment] =
                &[$(htrpc_expand_segment!($segment)),*];
            $crate::EntryPoint::new(SEGMENTS)
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! htrpc_expand_segment {
    (_) => {
        $crate::types::PathSegment::Var
    };
    ($s:expr) => {
        $crate::types::PathSegment::Val($s)
    }
}

pub use miasht::Method;

pub use client::RpcClient;
pub use error::{Error, ErrorKind};
pub use procedure::{EntryPoint, Procedure};
pub use procedure::{HandleRequest, RpcRequest, RpcResponse};
pub use server::{RpcServer, RpcServerBuilder};

pub mod deserializers;
pub mod serializers;
pub mod types;

mod client;
mod error;
mod procedure;
mod server;

pub type Result<T> = ::std::result::Result<T, Error>;
