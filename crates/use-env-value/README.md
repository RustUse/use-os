# use-env-value

Primitive environment variable value and key/value pair vocabulary.

`use-env-value` stores owned string values and pairs them with `EnvKey`. It does not read or mutate process environment, implement secret storage, merge configuration, or provide CLI-facing environment helpers.

```rust
use use_env_key::EnvKey;
use use_env_value::{EnvPair, EnvValue};

let pair = EnvPair::new(EnvKey::new("RUST_LOG").unwrap(), EnvValue::new("info"));

assert_eq!(pair.to_string(), "RUST_LOG=info");
```
