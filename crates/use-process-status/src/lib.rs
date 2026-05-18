#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_process_id::ProcessId;

/// Plain process lifecycle state vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProcessState {
    /// A process-like unit has been described or created.
    Created,
    /// A process-like unit is running.
    Running,
    /// A process-like unit exited.
    Exited,
    /// A process-like unit failed.
    Failed,
    /// The state is unknown.
    Unknown,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => formatter.write_str("created"),
            Self::Running => formatter.write_str("running"),
            Self::Exited => formatter.write_str("exited"),
            Self::Failed => formatter.write_str("failed"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

impl FromStr for ProcessState {
    type Err = ProcessStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ProcessStateParseError::Empty);
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "create" | "created" => Ok(Self::Created),
            "run" | "running" => Ok(Self::Running),
            "exit" | "exited" => Ok(Self::Exited),
            "fail" | "failed" => Ok(Self::Failed),
            "unknown" => Ok(Self::Unknown),
            _ => Err(ProcessStateParseError::Unknown),
        }
    }
}

/// Error returned when parsing a process state fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessStateParseError {
    /// The state was empty after trimming whitespace.
    Empty,
    /// The state was not part of the known vocabulary.
    Unknown,
}

impl fmt::Display for ProcessStateParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("process state cannot be empty"),
            Self::Unknown => formatter.write_str("unknown process state"),
        }
    }
}

impl Error for ProcessStateParseError {}

/// Plain process status metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessStatus {
    state: ProcessState,
    status_code: Option<i32>,
    message: Option<String>,
}

impl ProcessStatus {
    /// Creates process status metadata for a state.
    #[must_use]
    pub const fn new(state: ProcessState) -> Self {
        Self {
            state,
            status_code: None,
            message: None,
        }
    }

    /// Returns the process state.
    #[must_use]
    pub const fn state(&self) -> ProcessState {
        self.state
    }

    /// Returns the optional numeric status code.
    #[must_use]
    pub const fn status_code(&self) -> Option<i32> {
        self.status_code
    }

    /// Returns the optional message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns this status with a numeric status code attached.
    #[must_use]
    pub const fn with_status_code(mut self, status_code: i32) -> Self {
        self.status_code = Some(status_code);
        self
    }

    /// Returns this status with a message attached.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// Plain process outcome metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessOutcome {
    process_id: Option<ProcessId>,
    status: ProcessStatus,
}

impl ProcessOutcome {
    /// Creates an outcome from optional process identity and status metadata.
    #[must_use]
    pub const fn new(process_id: Option<ProcessId>, status: ProcessStatus) -> Self {
        Self { process_id, status }
    }

    /// Creates an outcome for a specific process ID.
    #[must_use]
    pub const fn for_process(process_id: ProcessId, status: ProcessStatus) -> Self {
        Self::new(Some(process_id), status)
    }

    /// Creates an outcome without process identity.
    #[must_use]
    pub const fn without_process(status: ProcessStatus) -> Self {
        Self::new(None, status)
    }

    /// Returns the optional process ID.
    #[must_use]
    pub const fn process_id(&self) -> Option<ProcessId> {
        self.process_id
    }

    /// Returns the status metadata.
    #[must_use]
    pub const fn status(&self) -> &ProcessStatus {
        &self.status
    }

    /// Returns the process state.
    #[must_use]
    pub const fn state(&self) -> ProcessState {
        self.status.state()
    }

    /// Returns the optional numeric status code.
    #[must_use]
    pub const fn status_code(&self) -> Option<i32> {
        self.status.status_code()
    }

    /// Returns the optional message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.status.message()
    }
}

#[cfg(test)]
mod tests {
    use super::{ProcessOutcome, ProcessState, ProcessStateParseError, ProcessStatus};
    use use_process_id::ProcessId;

    #[test]
    fn parses_and_displays_process_states() -> Result<(), ProcessStateParseError> {
        assert_eq!("created".parse::<ProcessState>()?, ProcessState::Created);
        assert_eq!("run".parse::<ProcessState>()?, ProcessState::Running);
        assert_eq!("exit".parse::<ProcessState>()?, ProcessState::Exited);
        assert_eq!("fail".parse::<ProcessState>()?, ProcessState::Failed);
        assert_eq!(ProcessState::Unknown.to_string(), "unknown");
        Ok(())
    }

    #[test]
    fn rejects_empty_or_unknown_process_states() {
        assert_eq!(
            "".parse::<ProcessState>(),
            Err(ProcessStateParseError::Empty)
        );
        assert_eq!(
            "sleeping".parse::<ProcessState>(),
            Err(ProcessStateParseError::Unknown)
        );
    }

    #[test]
    fn status_stores_plain_metadata() {
        let status = ProcessStatus::new(ProcessState::Failed)
            .with_status_code(1)
            .with_message("process failed");

        assert_eq!(status.state(), ProcessState::Failed);
        assert_eq!(status.status_code(), Some(1));
        assert_eq!(status.message(), Some("process failed"));
    }

    #[test]
    fn outcome_stores_optional_process_identity() {
        let process_id = ProcessId::new(42).unwrap();
        let status = ProcessStatus::new(ProcessState::Exited).with_status_code(0);
        let outcome = ProcessOutcome::for_process(process_id, status);

        assert_eq!(outcome.process_id(), Some(process_id));
        assert_eq!(outcome.state(), ProcessState::Exited);
        assert_eq!(outcome.status_code(), Some(0));
    }
}
