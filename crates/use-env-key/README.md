# use-env-key

Primitive environment variable key vocabulary.

`use-env-key` models the key itself. It rejects empty keys and keys containing `=`, preserves case, and does not read from, write to, or mutate `std::env`.

```rust
use use_env_key::EnvKey;

let key = EnvKey::new("Path").unwrap();

assert_eq!(key.as_str(), "Path");
assert_eq!(key.to_string(), "Path");
```
