# use-os-family

Primitive operating system family vocabulary.

`use-os-family` models broad OS family names as data. It does not detect the current OS or make behavioral assumptions based on the family.

```rust
use use_os_family::OsFamily;

let family: OsFamily = "darwin".parse().unwrap();

assert_eq!(family, OsFamily::Unix);
assert_eq!(family.to_string(), "unix");
```
