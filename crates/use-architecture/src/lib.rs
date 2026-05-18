#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// CPU or target architecture vocabulary.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Architecture {
    /// 32-bit x86 architecture.
    X86,
    /// 64-bit x86 architecture.
    X86_64,
    /// 32-bit ARM architecture.
    Arm,
    /// 64-bit ARM architecture.
    Aarch64,
    /// 32-bit RISC-V architecture.
    RiscV32,
    /// 64-bit RISC-V architecture.
    RiscV64,
    /// 32-bit WebAssembly architecture.
    Wasm32,
    /// 64-bit WebAssembly architecture.
    Wasm64,
    /// Unknown architecture.
    Unknown,
    /// A caller-defined architecture name.
    Custom(String),
}

impl fmt::Display for Architecture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86 => formatter.write_str("x86"),
            Self::X86_64 => formatter.write_str("x86_64"),
            Self::Arm => formatter.write_str("arm"),
            Self::Aarch64 => formatter.write_str("aarch64"),
            Self::RiscV32 => formatter.write_str("riscv32"),
            Self::RiscV64 => formatter.write_str("riscv64"),
            Self::Wasm32 => formatter.write_str("wasm32"),
            Self::Wasm64 => formatter.write_str("wasm64"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for Architecture {
    type Err = ArchitectureParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ArchitectureParseError::Empty);
        }

        let key = trimmed.to_ascii_lowercase().replace(['_', '-', ' '], "");

        match key.as_str() {
            "x86" | "i386" | "i486" | "i586" | "i686" => Ok(Self::X86),
            "x8664" | "amd64" | "x64" => Ok(Self::X86_64),
            "arm" => Ok(Self::Arm),
            "aarch64" | "arm64" => Ok(Self::Aarch64),
            "riscv32" | "rv32" => Ok(Self::RiscV32),
            "riscv64" | "rv64" => Ok(Self::RiscV64),
            "wasm32" | "webassembly32" => Ok(Self::Wasm32),
            "wasm64" | "webassembly64" => Ok(Self::Wasm64),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Error returned when parsing an architecture fails.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchitectureParseError {
    /// The architecture name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ArchitectureParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("architecture cannot be empty"),
        }
    }
}

impl Error for ArchitectureParseError {}

#[cfg(test)]
mod tests {
    use super::{Architecture, ArchitectureParseError};

    #[test]
    fn parses_known_architectures() -> Result<(), ArchitectureParseError> {
        assert_eq!("x86".parse::<Architecture>()?, Architecture::X86);
        assert_eq!("x86_64".parse::<Architecture>()?, Architecture::X86_64);
        assert_eq!("arm".parse::<Architecture>()?, Architecture::Arm);
        assert_eq!("aarch64".parse::<Architecture>()?, Architecture::Aarch64);
        assert_eq!("riscv32".parse::<Architecture>()?, Architecture::RiscV32);
        assert_eq!("riscv64".parse::<Architecture>()?, Architecture::RiscV64);
        assert_eq!("wasm32".parse::<Architecture>()?, Architecture::Wasm32);
        assert_eq!("wasm64".parse::<Architecture>()?, Architecture::Wasm64);
        assert_eq!("unknown".parse::<Architecture>()?, Architecture::Unknown);
        Ok(())
    }

    #[test]
    fn parses_obvious_aliases() -> Result<(), ArchitectureParseError> {
        assert_eq!("amd64".parse::<Architecture>()?, Architecture::X86_64);
        assert_eq!("i686".parse::<Architecture>()?, Architecture::X86);
        assert_eq!("arm64".parse::<Architecture>()?, Architecture::Aarch64);
        assert_eq!("risc-v64".parse::<Architecture>()?, Architecture::RiscV64);
        assert_eq!(
            "webassembly32".parse::<Architecture>()?,
            Architecture::Wasm32
        );
        Ok(())
    }

    #[test]
    fn stores_custom_architectures() -> Result<(), ArchitectureParseError> {
        assert_eq!(
            "loongarch64".parse::<Architecture>()?,
            Architecture::Custom("loongarch64".to_string())
        );
        Ok(())
    }

    #[test]
    fn rejects_empty_architecture_names() {
        assert_eq!(
            "  ".parse::<Architecture>(),
            Err(ArchitectureParseError::Empty)
        );
    }

    #[test]
    fn displays_canonical_names() {
        assert_eq!(Architecture::X86_64.to_string(), "x86_64");
        assert_eq!(Architecture::Aarch64.to_string(), "aarch64");
        assert_eq!(Architecture::RiscV32.to_string(), "riscv32");
    }
}
