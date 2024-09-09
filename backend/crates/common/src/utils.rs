//! # Utils
//!

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
