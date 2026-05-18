#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Broad operating system family vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OsFamily {
    /// Unix-like operating systems.
    Unix,
    /// Microsoft Windows operating systems.
    Windows,
    /// WebAssembly and WASI targets.
    Wasm,
    /// Unknown operating system family.
    Unknown,
    /// A caller-defined family name.
    Custom(String),
}

impl fmt::Display for OsFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unix => formatter.write_str("unix"),
            Self::Windows => formatter.write_str("windows"),
            Self::Wasm => formatter.write_str("wasm"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for OsFamily {
    type Err = OsFamilyParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(OsFamilyParseError::Empty);
        }

        let key = trimmed.to_ascii_lowercase().replace(['_', ' '], "-");

        match key.as_str() {
            "unix" | "linux" | "macos" | "mac-os" | "darwin" | "bsd" | "freebsd" | "openbsd"
            | "netbsd" => Ok(Self::Unix),
            "windows" | "win" | "win32" | "win64" | "mswindows" => Ok(Self::Windows),
            "wasm" | "webassembly" | "wasi" | "wasip1" | "wasip2" => Ok(Self::Wasm),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing an OS family fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OsFamilyParseError {
    /// The family name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for OsFamilyParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("OS family cannot be empty"),
        }
    }
}

impl Error for OsFamilyParseError {}

#[cfg(test)]
mod tests {
    use super::{OsFamily, OsFamilyParseError};

    #[test]
    fn parses_known_families() -> Result<(), OsFamilyParseError> {
        assert_eq!("unix".parse::<OsFamily>()?, OsFamily::Unix);
        assert_eq!("windows".parse::<OsFamily>()?, OsFamily::Windows);
        assert_eq!("wasm".parse::<OsFamily>()?, OsFamily::Wasm);
        assert_eq!("unknown".parse::<OsFamily>()?, OsFamily::Unknown);
        Ok(())
    }

    #[test]
    fn parses_obvious_aliases() -> Result<(), OsFamilyParseError> {
        assert_eq!("linux".parse::<OsFamily>()?, OsFamily::Unix);
        assert_eq!("darwin".parse::<OsFamily>()?, OsFamily::Unix);
        assert_eq!("win32".parse::<OsFamily>()?, OsFamily::Windows);
        assert_eq!("wasi".parse::<OsFamily>()?, OsFamily::Wasm);
        Ok(())
    }

    #[test]
    fn stores_custom_families() -> Result<(), OsFamilyParseError> {
        assert_eq!(
            "plan9".parse::<OsFamily>()?,
            OsFamily::Custom("plan9".to_string())
        );
        Ok(())
    }

    #[test]
    fn rejects_empty_family_names() {
        assert_eq!("  ".parse::<OsFamily>(), Err(OsFamilyParseError::Empty));
    }

    #[test]
    fn displays_canonical_names() {
        assert_eq!(OsFamily::Unix.to_string(), "unix");
        assert_eq!(OsFamily::Windows.to_string(), "windows");
        assert_eq!(OsFamily::Wasm.to_string(), "wasm");
        assert_eq!(
            OsFamily::Custom("custom-os".to_string()).to_string(),
            "custom-os"
        );
    }
}
