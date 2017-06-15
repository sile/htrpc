//! Pretty-Printed JSON serialize/deserialize function.
//!
//! This module can be used as the value of the `serde(with)` attribute
//! (i.e., `#[serde(with = "htrpc::json_pretty")]`).
use serde::ser;
use serde::{Serialize, Serializer};
use serdeconv;

/// Serializes the value as a pretty printed JSON string.
///
/// This function can be used as the value of the `serde(serialize_with)` attribute
/// (i.e., `#[serde(serialize_with = "htrpc::json_pretty::serialize")]`).
pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let json = track!(serdeconv::to_json_string_pretty(&value)).map_err(
        |e| {
            ser::Error::custom(e)
        },
    )?;
    json.serialize(serializer)
}

pub use json::deserialize;
