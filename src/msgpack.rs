//! MessagePack serialize/deserialize function.
//!
//! This module can be used as the value of the `serde(with)` attribute
//! (i.e., `#[serde(with = "htrpc::msgpack")]`).
use serde::{de, ser};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serdeconv;

/// Serializes the value as a MessagePack binary.
///
/// This function can be used as the value of the `serde(serialize_with)` attribute
/// (i.e., `#[serde(serialize_with = "htrpc::msgpack::serialize")]`).
pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where T: Serialize,
          S: Serializer
{
    let msgpack = track!(serdeconv::to_msgpack_vec(&value))
        .map_err(|e| ser::Error::custom(e))?;
    msgpack.serialize(serializer)
}

/// Deserializes a MessagePack binary.
///
/// This function can be used as the value of the `serde(deserialize_with)` attribute
/// (i.e., `#[serde(deserialize_with = "htrpc::msgpack::deserialize")]`).
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where T: for<'a> Deserialize<'a>,
          D: Deserializer<'de>
{
    let msgpack = Vec::deserialize(deserializer)?;
    track!(serdeconv::from_msgpack_slice(&msgpack)).map_err(|e| de::Error::custom(e))
}
