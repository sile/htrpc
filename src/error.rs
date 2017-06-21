use std;
use std::io;
use std::fmt::Display;
use std::sync::mpsc::RecvError;
use handy_async::future::Phase;
use miasht;
use serde::{de, ser};
use serdeconv;
use trackable::error::TrackableError;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};
use url;

/// This crate specific error type.
#[derive(Debug, Clone)]
pub struct Error(TrackableError<ErrorKind>);
derive_traits_for_trackable_error_newtype!(Error, ErrorKind);
impl From<RecvError> for Error {
    fn from(f: RecvError) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<io::Error> for Error {
    fn from(f: io::Error) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl<T> From<(io::Error, T)> for Error {
    fn from((f, _): (io::Error, T)) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(f: std::str::Utf8Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<std::str::ParseBoolError> for Error {
    fn from(f: std::str::ParseBoolError) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(f: std::string::FromUtf8Error) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(f: std::num::ParseIntError) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<std::num::ParseFloatError> for Error {
    fn from(f: std::num::ParseFloatError) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<url::ParseError> for Error {
    fn from(f: url::ParseError) -> Self {
        ErrorKind::Invalid.cause(f).into()
    }
}
impl From<miasht::Error> for Error {
    fn from(f: miasht::Error) -> Self {
        ErrorKind::Other.takes_over(f).into()
    }
}
impl From<serdeconv::Error> for Error {
    fn from(f: serdeconv::Error) -> Self {
        if *f.kind() == serdeconv::ErrorKind::Invalid {
            ErrorKind::Invalid.takes_over(f).into()
        } else {
            ErrorKind::Other.takes_over(f).into()
        }
    }
}
impl<A, B, C, D, E> From<Phase<A, B, C, D, E>> for Error
where
    Error: From<A>,
    Error: From<B>,
    Error: From<C>,
    Error: From<D>,
    Error: From<E>,
{
    fn from(f: Phase<A, B, C, D, E>) -> Self {
        match f {
            Phase::A(e) => track!(Error::from(e), "Phase::A"),
            Phase::B(e) => track!(Error::from(e), "Phase::B"),
            Phase::C(e) => track!(Error::from(e), "Phase::C"),
            Phase::D(e) => track!(Error::from(e), "Phase::D"),
            Phase::E(e) => track!(Error::from(e), "Phase::E"),
        }
    }
}
impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error(ErrorKind::Invalid.cause(msg.to_string()))
    }
}
impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error(ErrorKind::Invalid.cause(msg.to_string()))
    }
}

/// The list of the possible error kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Input data is invalid.
    Invalid,

    /// Other error.
    Other,
}
impl TrackableErrorKind for ErrorKind {}
