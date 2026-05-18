# use-platform

Primitive platform identity vocabulary.

`use-platform` stores target-triple-like platform identity as plain data. It does not detect the current platform, call `std::env::consts`, normalize every possible target triple, or replace target-triple crates.

```rust
use use_platform::{Platform, PlatformTriple};

let triple = PlatformTriple::new("aarch64-apple-darwin").unwrap();
let platform = Platform::new(triple);

assert_eq!(platform.to_string(), "aarch64-apple-darwin");
```