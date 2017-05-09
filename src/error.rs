use std::io;
use std::fmt::Display;
use serde::ser;
use trackable::error::{TrackableError, IntoTrackableError};
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

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

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Invalid,
    Other,
}
impl TrackableErrorKind for ErrorKind {}
impl IntoTrackableError<io::Error> for ErrorKind {
    fn into_trackable_error(e: io::Error) -> TrackableError<ErrorKind> {
        ErrorKind::Other.cause(e)
    }
}
