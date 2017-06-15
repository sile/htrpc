//! JSON serialize/deserialize function.
//!
//! This module can be used as the value of the `serde(with)` attribute
//! (i.e., `#[serde(with = "htrpc::json")]`).
use serde::{de, ser};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serdeconv;

/// Serializes the value as a JSON string.
///
/// This function can be used as the value of the `serde(serialize_with)` attribute
/// (i.e., `#[serde(serialize_with = "htrpc::json::serialize")]`).
pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let json = track!(serdeconv::to_json_string(&value)).map_err(|e| {
        ser::Error::custom(e)
    })?;
    json.serialize(serializer)
}

/// Deserializes a JSON string.
///
/// This function can be used as the value of the `serde(deserialize_with)` attribute
/// (i.e., `#[serde(deserialize_with = "htrpc::json::deserialize")]`).
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: for<'a> Deserialize<'a>,
    D: Deserializer<'de>,
{
    let json = String::deserialize(deserializer)?;
    track!(serdeconv::from_json_str(&json)).map_err(|e| de::Error::custom(e))
}
