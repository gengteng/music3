//! # Utils
//!

use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

/// Base64 encoding and decoding
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Base64(pub Vec<u8>);

impl<T> From<T> for Base64
where
    Vec<u8>: From<T>,
{
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl Serialize for Base64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_str::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for Base64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde_str::deserialize(deserializer)
    }
}

impl Deref for Base64 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for Base64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Base64 {
    /// Get the inner `Vec<u8>`
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl FromStr for Base64 {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = base64::engine::general_purpose::STANDARD.decode(s.as_bytes())?;
        Ok(Self(v))
    }
}

impl Display for Base64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = base64::engine::general_purpose::STANDARD.encode(&self.0);
        write!(f, "{}", v)
    }
}

/// Serde implementation of types that implement Display and FromStr
pub mod serde_str {
    use std::{fmt::Display, str::FromStr};

    use serde::{Deserialize, Deserializer, Serializer};

    /// Serialize implementation of Display
    pub fn serialize<T: Display, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = value.to_string();
        serializer.serialize_str(&v)
    }

    /// Deserialize implementation of FromStr
    pub fn deserialize<'de, D, S>(deserializer: D) -> Result<S, D::Error>
    where
        D: Deserializer<'de>,
        S: FromStr,
        S::Err: Display,
    {
        let v = String::deserialize(deserializer)?;
        v.parse().map_err(serde::de::Error::custom)
    }
}
