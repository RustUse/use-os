# use-architecture

Primitive CPU and target architecture vocabulary.

`use-architecture` models architecture names as data. It does not detect the current architecture, model CPU features, or replace target-feature crates.

```rust
use use_architecture::Architecture;

let architecture: Architecture = "arm64".parse().unwrap();

assert_eq!(architecture, Architecture::Aarch64);
assert_eq!(architecture.to_string(), "aarch64");
```
