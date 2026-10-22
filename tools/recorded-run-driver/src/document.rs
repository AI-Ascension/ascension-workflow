// SPDX-License-Identifier: MIT

//! Syntax admission for delivery plans and process reports, not a bundle validator.

use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Number, Value};
use std::{fmt, io};

/// Decode one bounded JSON document without discarding repeated object fields.
/// Diagnostics contain no input keys, values, parser excerpts, or filesystem paths.
pub fn decode_document(bytes: &[u8]) -> io::Result<Value> {
    if bytes.len() as u64 > crate::REPORT_LIMIT {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "JSON document exceeds report bound",
        ));
    }
    serde_json::from_slice::<UniqueDocument>(bytes)
        .map(|document| document.0)
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid or ambiguous JSON document",
            )
        })
}

struct UniqueDocument(Value);

impl<'de> Deserialize<'de> for UniqueDocument {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(DocumentVisitor)
    }
}

struct DocumentVisitor;

impl<'de> Visitor<'de> for DocumentVisitor {
    type Value = UniqueDocument;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value with unique object keys")
    }

    fn visit_bool<E: Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(UniqueDocument(Value::Bool(value)))
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(UniqueDocument(Value::Number(value.into())))
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(UniqueDocument(Value::Number(value.into())))
    }

    fn visit_f64<E: Error>(self, value: f64) -> Result<Self::Value, E> {
        Number::from_f64(value)
            .map(|number| UniqueDocument(Value::Number(number)))
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        self.visit_string(value.to_owned())
    }

    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(UniqueDocument(Value::String(value)))
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(UniqueDocument(Value::Null))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<UniqueDocument>()? {
            values.push(value.0);
        }
        Ok(UniqueDocument(Value::Array(values)))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<Self::Value, A::Error> {
        let mut values = Map::new();
        while let Some(key) = object.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(A::Error::custom("duplicate JSON field"));
            }
            let value = object.next_value::<UniqueDocument>()?;
            values.insert(key, value.0);
        }
        Ok(UniqueDocument(Value::Object(values)))
    }
}

#[cfg(test)]
mod tests;
