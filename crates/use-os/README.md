# use-os

Facade crate for primitive RustUse OS-facing vocabulary.

`use-os` re-exports the focused crates in this set behind short module names. It is not an operating-system abstraction framework, runtime, process runner, scheduler, thread pool, shell, task runner, CLI framework, or replacement for Rust `std`.

`use-cli` owns human command-line interaction primitives. `use-os` owns lower-level OS-facing vocabulary used beneath tools, services, and runtimes.

```rust
use use_os::prelude::{Architecture, EnvKey, EnvPair, EnvValue, OsFamily, PlatformTriple};

let family: OsFamily = "windows".parse().unwrap();
let architecture: Architecture = "amd64".parse().unwrap();
let triple = PlatformTriple::new("x86_64-pc-windows-msvc").unwrap();
let pair = EnvPair::new(EnvKey::new("Path").unwrap(), EnvValue::new("C:\\Tools"));

assert_eq!(family.to_string(), "windows");
assert_eq!(architecture.to_string(), "x86_64");
assert_eq!(triple.to_string(), "x86_64-pc-windows-msvc");
assert_eq!(pair.key().as_str(), "Path");
```

This crate does not detect platforms, execute commands, inspect processes, create threads, or read or mutate environment variables.
