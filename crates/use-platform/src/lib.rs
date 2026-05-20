#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A plain, owned platform triple value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlatformTriple(String);

impl PlatformTriple {
    /// Creates a platform triple from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformTripleError::Empty`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PlatformTripleError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(PlatformTripleError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the platform triple text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the triple and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PlatformTriple {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PlatformTriple {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PlatformTriple {
    type Err = PlatformTripleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing platform vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformTripleError {
    /// The platform triple was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PlatformTripleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("platform triple cannot be empty"),
        }
    }
}

impl Error for PlatformTripleError {}

/// A plain platform identity value backed by a platform triple.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Platform {
    triple: PlatformTriple,
}

impl Platform {
    /// Creates a platform from an already modeled platform triple.
    #[must_use]
    pub const fn new(triple: PlatformTriple) -> Self {
        Self { triple }
    }

    /// Returns the platform triple.
    #[must_use]
    pub const fn triple(&self) -> &PlatformTriple {
        &self.triple
    }

    /// Consumes the platform and returns the platform triple.
    #[must_use]
    pub fn into_triple(self) -> PlatformTriple {
        self.triple
    }
}

impl From<PlatformTriple> for Platform {
    fn from(triple: PlatformTriple) -> Self {
        Self::new(triple)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.triple.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::{Platform, PlatformTriple, PlatformTripleError};

    #[test]
    fn accepts_common_platform_triples() {
        for triple in [
            "x86_64-unknown-linux-gnu",
            "aarch64-apple-darwin",
            "wasm32-wasip1",
        ] {
            assert_eq!(PlatformTriple::new(triple).unwrap().as_str(), triple);
        }
    }

    #[test]
    fn rejects_empty_platform_triples() {
        assert_eq!(PlatformTriple::new("  "), Err(PlatformTripleError::Empty));
    }

    #[test]
    fn display_round_trips_through_parse() {
        let triple = PlatformTriple::new("x86_64-pc-windows-msvc").unwrap();
        let round_trip: PlatformTriple = triple.to_string().parse().unwrap();

        assert_eq!(round_trip, triple);
    }

    #[test]
    fn platform_wraps_a_triple_without_detection() {
        let triple = PlatformTriple::new("aarch64-apple-darwin").unwrap();
        let platform = Platform::new(triple.clone());

        assert_eq!(platform.triple(), &triple);
        assert_eq!(platform.to_string(), "aarch64-apple-darwin");
    }
}
