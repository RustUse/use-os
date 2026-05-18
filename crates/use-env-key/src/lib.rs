#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A validated environment variable key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnvKey(String);

impl EnvKey {
    /// Creates an environment variable key.
    ///
    /// # Errors
    ///
    /// Returns [`EnvKeyError::Empty`] when the trimmed input is empty and
    /// [`EnvKeyError::ContainsEquals`] when the key contains `=`.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EnvKeyError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(EnvKeyError::Empty);
        }

        if trimmed.contains('=') {
            return Err(EnvKeyError::ContainsEquals);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EnvKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for EnvKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for EnvKey {
    type Err = EnvKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing environment keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvKeyError {
    /// The key was empty after trimming whitespace.
    Empty,
    /// The key contained `=`.
    ContainsEquals,
}

impl fmt::Display for EnvKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("environment key cannot be empty"),
            Self::ContainsEquals => formatter.write_str("environment key cannot contain '='"),
        }
    }
}

impl Error for EnvKeyError {}

#[cfg(test)]
mod tests {
    use super::{EnvKey, EnvKeyError};

    #[test]
    fn accepts_environment_keys_and_preserves_case() {
        let key = EnvKey::new("Path").unwrap();

        assert_eq!(key.as_str(), "Path");
        assert_eq!(key.to_string(), "Path");
    }

    #[test]
    fn rejects_empty_keys() {
        assert_eq!(EnvKey::new("  "), Err(EnvKeyError::Empty));
    }

    #[test]
    fn rejects_keys_containing_equals() {
        assert_eq!(EnvKey::new("KEY=value"), Err(EnvKeyError::ContainsEquals));
    }

    #[test]
    fn display_round_trips_through_parse() {
        let key = EnvKey::new("RUST_LOG").unwrap();
        let round_trip: EnvKey = key.to_string().parse().unwrap();

        assert_eq!(round_trip, key);
    }
}
