#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_architecture as architecture;
pub use use_env_key as env_key;
pub use use_env_value as env_value;
pub use use_os_family as os_family;
pub use use_platform as platform;
pub use use_process_id as process_id;
pub use use_process_status as process_status;
pub use use_thread_id as thread_id;
pub use use_thread_name as thread_name;

/// Common primitive OS-facing vocabulary reexports.
pub mod prelude {
    pub use use_architecture::{Architecture, ArchitectureParseError};
    pub use use_env_key::{EnvKey, EnvKeyError};
    pub use use_env_value::{EnvPair, EnvValue};
    pub use use_os_family::{OsFamily, OsFamilyParseError};
    pub use use_platform::{Platform, PlatformTriple, PlatformTripleError};
    pub use use_process_id::{ParentProcessId, ProcessId, ProcessIdError};
    pub use use_process_status::{
        ProcessOutcome, ProcessState, ProcessStateParseError, ProcessStatus,
    };
    pub use use_thread_id::{ThreadIdLabel, ThreadIdLabelError};
    pub use use_thread_name::{ThreadCount, ThreadCountError, ThreadName, ThreadNameError};
}

#[cfg(test)]
mod tests {
    use super::prelude::{
        Architecture, EnvKey, EnvPair, EnvValue, OsFamily, Platform, PlatformTriple, ProcessId,
        ProcessOutcome, ProcessState, ProcessStatus, ThreadCount, ThreadIdLabel, ThreadName,
    };

    #[test]
    fn facade_exposes_os_vocabulary_without_execution() {
        let family: OsFamily = "linux".parse().unwrap();
        let architecture: Architecture = "amd64".parse().unwrap();
        let platform = Platform::new(PlatformTriple::new("x86_64-unknown-linux-gnu").unwrap());
        let process_id = ProcessId::new(42).unwrap();
        let status = ProcessStatus::new(ProcessState::Exited).with_status_code(0);
        let outcome = ProcessOutcome::for_process(process_id, status);
        let thread_label = ThreadIdLabel::label("worker-1").unwrap();
        let thread_name = ThreadName::new("worker").unwrap();
        let thread_count = ThreadCount::new(4).unwrap();
        let env_pair = EnvPair::new(EnvKey::new("RUST_LOG").unwrap(), EnvValue::new("info"));

        assert_eq!(family, OsFamily::Unix);
        assert_eq!(architecture, Architecture::X86_64);
        assert_eq!(platform.to_string(), "x86_64-unknown-linux-gnu");
        assert_eq!(outcome.process_id(), Some(process_id));
        assert_eq!(outcome.status_code(), Some(0));
        assert_eq!(thread_label.to_string(), "worker-1");
        assert_eq!(thread_name.as_str(), "worker");
        assert_eq!(thread_count.get(), 4);
        assert_eq!(env_pair.to_string(), "RUST_LOG=info");
    }
}
