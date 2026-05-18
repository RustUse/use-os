#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, num::NonZeroUsize, str::FromStr};
use std::error::Error;

/// A non-empty thread name value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreadName(String);

impl ThreadName {
    /// Creates a thread name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ThreadNameError::Empty`] when the trimmed input is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ThreadNameError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(ThreadNameError::Empty);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Returns the thread name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the thread name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ThreadName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ThreadName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ThreadName {
    type Err = ThreadNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing thread names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadNameError {
    /// The name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ThreadNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("thread name cannot be empty"),
        }
    }
}

impl Error for ThreadNameError {}

/// A non-zero thread count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ThreadCount(NonZeroUsize);

impl ThreadCount {
    /// Creates a thread count from a non-zero value.
    ///
    /// # Errors
    ///
    /// Returns [`ThreadCountError::Zero`] when `value` is zero.
    pub fn new(value: usize) -> Result<Self, ThreadCountError> {
        NonZeroUsize::new(value)
            .map(Self)
            .ok_or(ThreadCountError::Zero)
    }

    /// Returns the count value.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl From<NonZeroUsize> for ThreadCount {
    fn from(value: NonZeroUsize) -> Self {
        Self(value)
    }
}

impl TryFrom<usize> for ThreadCount {
    type Error = ThreadCountError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for ThreadCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// Errors returned while constructing thread counts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadCountError {
    /// A zero count is not accepted.
    Zero,
}

impl fmt::Display for ThreadCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("thread count must be non-zero"),
        }
    }
}

impl Error for ThreadCountError {}

#[cfg(test)]
mod tests {
    use super::{ThreadCount, ThreadCountError, ThreadName, ThreadNameError};

    #[test]
    fn thread_names_reject_empty_values() {
        assert_eq!(ThreadName::new("  "), Err(ThreadNameError::Empty));
    }

    #[test]
    fn thread_names_store_trimmed_text() {
        let name = ThreadName::new(" worker ").unwrap();

        assert_eq!(name.as_str(), "worker");
        assert_eq!(name.to_string(), "worker");
    }

    #[test]
    fn thread_counts_reject_zero() {
        assert_eq!(ThreadCount::new(0), Err(ThreadCountError::Zero));
    }

    #[test]
    fn thread_counts_store_non_zero_values() {
        let count = ThreadCount::try_from(4).unwrap();

        assert_eq!(count.get(), 4);
        assert_eq!(count.to_string(), "4");
    }
}
