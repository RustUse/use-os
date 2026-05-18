# use-os

RustUse is “Composable sets of primitive Rust utility crates for fellow crustaceans.”

`use-os` is a primitive OS-facing vocabulary set. It provides small, stable-looking Rust types for platform names, OS families, architecture names, process identity, process status metadata, thread identity and naming vocabulary, and environment key/value modeling.

`use-os` does not replace Rust's standard library. It does not spawn processes, create threads, read environment variables, mutate environment variables, manage runtimes, execute commands, parse shells, render diagnostics, or provide terminal interaction helpers.

## Boundary With use-cli

`use-cli` describes how humans interact with command-line programs. It owns command-line arguments, flags, options, command names, standard stream helpers, prompt primitives, terminal capability primitives, and CLI exit-code conveniences.

`use-os` describes lower-level operating-system-facing primitives used beneath tools, services, and runtimes. It models vocabulary such as platform triples, OS families, architectures, process IDs, process status metadata, thread labels, thread names, thread counts, environment keys, and environment values.

## Crates

- `use-os`: facade crate for the full primitive vocabulary set
- `use-platform`: platform identity and platform triple vocabulary
- `use-os-family`: OS family vocabulary
- `use-architecture`: CPU and target architecture vocabulary
- `use-process-id`: process identity wrappers
- `use-process-status`: process status and outcome metadata
- `use-thread-id`: thread identity label vocabulary
- `use-thread-name`: thread name and thread count vocabulary
- `use-env-key`: environment variable key vocabulary
- `use-env-value`: environment variable value and key/value pair vocabulary

## Scope

- plain data types for OS-facing concepts
- deterministic constructors, accessors, display implementations, and parsing helpers
- small crates with minimal dependencies
- no operating-system abstraction framework
- no process runner, shell, task runner, or command parser
- no CLI framework, terminal UI, prompt helpers, or stdin/stdout/stderr helpers
- no thread pool, scheduler, async runtime, or global runtime assumptions
- no platform detection, environment reader/writer, logging framework, or diagnostic renderer

## Example

```rust
use use_os::prelude::{
    Architecture, EnvKey, EnvPair, EnvValue, OsFamily, Platform, PlatformTriple, ProcessId,
    ProcessOutcome, ProcessState, ProcessStatus,
};

let family: OsFamily = "linux".parse().unwrap();
let architecture: Architecture = "x86_64".parse().unwrap();
let triple = PlatformTriple::new("x86_64-unknown-linux-gnu").unwrap();
let platform = Platform::new(triple);

let process_id = ProcessId::new(42).unwrap();
let status = ProcessStatus::new(ProcessState::Exited).with_status_code(0);
let outcome = ProcessOutcome::for_process(process_id, status);

let env_key = EnvKey::new("RUST_LOG").unwrap();
let env_pair = EnvPair::new(env_key, EnvValue::new("info"));

assert_eq!(family.to_string(), "unix");
assert_eq!(architecture.to_string(), "x86_64");
assert_eq!(platform.to_string(), "x86_64-unknown-linux-gnu");
assert_eq!(outcome.process_id(), Some(process_id));
assert_eq!(env_pair.to_string(), "RUST_LOG=info");
```

This example describes OS-facing concepts. It does not detect the current platform, execute a process, inspect a running process, create a thread, read an environment variable, or mutate process environment.

## Related Sets

- `use-cli`
- `use-fs`
- `use-net`
- `use-config`
- `use-diagnostic`
- `use-rust`

## License

Licensed under either of the following, at your option:

- MIT License
- Apache License, Version 2.0
