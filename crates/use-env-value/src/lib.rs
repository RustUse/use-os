#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

use use_env_key::EnvKey;

/// A plain owned environment variable value.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnvValue(String);

impl EnvValue {
    /// Creates an environment value from owned or borrowed string data.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the value text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EnvValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for EnvValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for EnvValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for EnvValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A modeled environment key/value pair.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnvPair {
    key: EnvKey,
    value: EnvValue,
}

impl EnvPair {
    /// Creates an environment key/value pair.
    #[must_use]
    pub const fn new(key: EnvKey, value: EnvValue) -> Self {
        Self { key, value }
    }

    /// Creates an environment key/value pair from a key and value-like input.
    #[must_use]
    pub fn from_value(key: EnvKey, value: impl Into<EnvValue>) -> Self {
        Self::new(key, value.into())
    }

    /// Returns the environment key.
    #[must_use]
    pub const fn key(&self) -> &EnvKey {
        &self.key
    }

    /// Returns the environment value.
    #[must_use]
    pub const fn value(&self) -> &EnvValue {
        &self.value
    }

    /// Consumes the pair and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (EnvKey, EnvValue) {
        (self.key, self.value)
    }
}

impl fmt::Display for EnvPair {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}={}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{EnvPair, EnvValue};
    use use_env_key::EnvKey;

    #[test]
    fn values_store_plain_owned_strings() {
        let value = EnvValue::new("info");

        assert_eq!(value.as_str(), "info");
        assert_eq!(value.to_string(), "info");
    }

    #[test]
    fn values_allow_empty_strings() {
        let value = EnvValue::new("");

        assert_eq!(value.as_str(), "");
    }

    #[test]
    fn pairs_store_keys_and_values() {
        let key = EnvKey::new("RUST_LOG").unwrap();
        let value = EnvValue::new("debug");
        let pair = EnvPair::new(key.clone(), value.clone());

        assert_eq!(pair.key(), &key);
        assert_eq!(pair.value(), &value);
        assert_eq!(pair.to_string(), "RUST_LOG=debug");
    }
}
