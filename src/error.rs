use std;
use std::io;
use std::fmt::Display;
use handy_async::future::Phase;
use miasht;
use serde::{de, ser};
use serdeconv;
use trackable::error::{TrackableError, IntoTrackableError};
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

/// This crate specific error type.
#[derive(Debug, Clone)]
pub struct Error(TrackableError<ErrorKind>);
derive_traits_for_trackable_error_newtype!(Error, ErrorKind);
impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
        where T: Display
    {
        Error(ErrorKind::Invalid.cause(msg.to_string()))
    }
}
impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
        where T: Display
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
impl IntoTrackableError<io::Error> for ErrorKind {
    fn into_trackable_error(e: io::Error) -> TrackableError<ErrorKind> {
        ErrorKind::Other.cause(e)
    }
}
impl<T> IntoTrackableError<(io::Error, T)> for ErrorKind {
    fn into_trackable_error((e, _): (io::Error, T)) -> TrackableError<ErrorKind> {
        ErrorKind::Other.cause(e)
    }
}
impl IntoTrackableError<std::str::Utf8Error> for ErrorKind {
    fn into_trackable_error(e: std::str::Utf8Error) -> TrackableError<ErrorKind> {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<std::str::ParseBoolError> for ErrorKind {
    fn into_trackable_error(e: std::str::ParseBoolError) -> TrackableError<ErrorKind> {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<std::string::FromUtf8Error> for ErrorKind {
    fn into_trackable_error(e: std::string::FromUtf8Error) -> TrackableError<ErrorKind> {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<std::num::ParseIntError> for ErrorKind {
    fn into_trackable_error(e: std::num::ParseIntError) -> TrackableError<ErrorKind> {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<std::num::ParseFloatError> for ErrorKind {
    fn into_trackable_error(e: std::num::ParseFloatError) -> TrackableError<ErrorKind> {
        ErrorKind::Invalid.cause(e)
    }
}
impl IntoTrackableError<miasht::Error> for ErrorKind {
    fn into_trackable_error(e: miasht::Error) -> TrackableError<ErrorKind> {
        ErrorKind::Other.takes_over(e)
    }
}
impl IntoTrackableError<serdeconv::Error> for ErrorKind {
    fn into_trackable_error(e: serdeconv::Error) -> TrackableError<ErrorKind> {
        if *e.kind() == serdeconv::ErrorKind::Invalid {
            ErrorKind::Invalid.takes_over(e)
        } else {
            ErrorKind::Other.takes_over(e)
        }
    }
}
impl<A, B, C, D, E> IntoTrackableError<Phase<A, B, C, D, E>> for ErrorKind
    where ErrorKind: IntoTrackableError<A>,
          ErrorKind: IntoTrackableError<B>,
          ErrorKind: IntoTrackableError<C>,
          ErrorKind: IntoTrackableError<D>,
          ErrorKind: IntoTrackableError<E>
{
    fn into_trackable_error(from: Phase<A, B, C, D, E>) -> TrackableError<ErrorKind> {
        match from {
            Phase::A(e) => track!(ErrorKind::into_trackable_error(e), "Phase::A"),
            Phase::B(e) => track!(ErrorKind::into_trackable_error(e), "Phase::B"),
            Phase::C(e) => track!(ErrorKind::into_trackable_error(e), "Phase::C"),
            Phase::D(e) => track!(ErrorKind::into_trackable_error(e), "Phase::D"),
            Phase::E(e) => track!(ErrorKind::into_trackable_error(e), "Phase::E"),
        }
    }
}
