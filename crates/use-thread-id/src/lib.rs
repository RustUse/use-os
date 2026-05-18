#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// A stable label or numeric thread-like identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ThreadIdLabel {
    /// A caller-provided stable thread label.
    Label(String),
    /// A caller-provided numeric thread-like identifier.
    Number(u64),
}

impl ThreadIdLabel {
    /// Creates a thread ID label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ThreadIdLabelError::Empty`] when the trimmed input is empty.
    pub fn label(value: impl AsRef<str>) -> Result<Self, ThreadIdLabelError> {
        let trimmed = value.as_ref().trim();

        if trimmed.is_empty() {
            return Err(ThreadIdLabelError::Empty);
        }

        Ok(Self::Label(trimmed.to_string()))
    }

    /// Creates a numeric thread-like identifier.
    #[must_use]
    pub const fn number(value: u64) -> Self {
        Self::Number(value)
    }

    /// Returns the label text when this value is label-backed.
    #[must_use]
    pub fn as_label(&self) -> Option<&str> {
        match self {
            Self::Label(value) => Some(value),
            Self::Number(_) => None,
        }
    }

    /// Returns the numeric identifier when this value is number-backed.
    #[must_use]
    pub const fn as_number(&self) -> Option<u64> {
        match self {
            Self::Label(_) => None,
            Self::Number(value) => Some(*value),
        }
    }
}

impl fmt::Display for ThreadIdLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(value) => formatter.write_str(value),
            Self::Number(value) => value.fmt(formatter),
        }
    }
}

impl FromStr for ThreadIdLabel {
    type Err = ThreadIdLabelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::label(value)
    }
}

/// Errors returned while constructing thread ID labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadIdLabelError {
    /// The label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ThreadIdLabelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("thread ID label cannot be empty"),
        }
    }
}

impl Error for ThreadIdLabelError {}

#[cfg(test)]
mod tests {
    use super::{ThreadIdLabel, ThreadIdLabelError};

    #[test]
    fn accepts_non_empty_labels() {
        let label = ThreadIdLabel::label(" worker-1 ").unwrap();

        assert_eq!(label.as_label(), Some("worker-1"));
        assert_eq!(label.to_string(), "worker-1");
    }

    #[test]
    fn rejects_empty_labels() {
        assert_eq!(ThreadIdLabel::label("  "), Err(ThreadIdLabelError::Empty));
    }

    #[test]
    fn stores_numeric_identifiers() {
        let label = ThreadIdLabel::number(12);

        assert_eq!(label.as_number(), Some(12));
        assert_eq!(label.to_string(), "12");
    }
}
