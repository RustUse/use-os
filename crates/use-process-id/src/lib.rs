#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, num::NonZeroU32};
use std::error::Error;

/// A non-zero numeric process identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessId(NonZeroU32);

impl ProcessId {
    /// Creates a process ID from a non-zero numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessIdError::Zero`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, ProcessIdError> {
        NonZeroU32::new(value).map(Self).ok_or(ProcessIdError::Zero)
    }

    /// Returns the numeric process ID.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl From<NonZeroU32> for ProcessId {
    fn from(value: NonZeroU32) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for ProcessId {
    type Error = ProcessIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for ProcessId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// A non-zero numeric parent process identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParentProcessId(NonZeroU32);

impl ParentProcessId {
    /// Creates a parent process ID from a non-zero numeric value.
    ///
    /// # Errors
    ///
    /// Returns [`ProcessIdError::Zero`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, ProcessIdError> {
        NonZeroU32::new(value).map(Self).ok_or(ProcessIdError::Zero)
    }

    /// Returns the numeric parent process ID.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl From<NonZeroU32> for ParentProcessId {
    fn from(value: NonZeroU32) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for ParentProcessId {
    type Error = ProcessIdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for ParentProcessId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// Errors returned while constructing process IDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessIdError {
    /// Process ID zero is not accepted by these wrappers.
    Zero,
}

impl fmt::Display for ProcessIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("process ID must be non-zero"),
        }
    }
}

impl Error for ProcessIdError {}

#[cfg(test)]
mod tests {
    use super::{ParentProcessId, ProcessId, ProcessIdError};
    use std::collections::BTreeSet;

    #[test]
    fn process_ids_reject_zero() {
        assert_eq!(ProcessId::new(0), Err(ProcessIdError::Zero));
        assert_eq!(ParentProcessId::new(0), Err(ProcessIdError::Zero));
    }

    #[test]
    fn process_ids_store_numeric_values() {
        let process_id = ProcessId::new(42).unwrap();

        assert_eq!(process_id.get(), 42);
        assert_eq!(process_id.to_string(), "42");
    }

    #[test]
    fn parent_process_ids_store_numeric_values() {
        let parent_process_id = ParentProcessId::try_from(7).unwrap();

        assert_eq!(parent_process_id.get(), 7);
        assert_eq!(parent_process_id.to_string(), "7");
    }

    #[test]
    fn process_ids_have_deterministic_ordering() {
        let mut process_ids = BTreeSet::new();

        process_ids.insert(ProcessId::new(2).unwrap());
        process_ids.insert(ProcessId::new(1).unwrap());

        assert_eq!(
            process_ids
                .into_iter()
                .map(ProcessId::get)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }
}
