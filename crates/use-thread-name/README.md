# use-thread-name

Primitive thread name and thread count vocabulary.

`use-thread-name` models thread names and non-zero thread counts as plain values. It does not name actual running threads, create threads, manage threads, or replace `std::thread`.

```rust
use use_thread_name::{ThreadCount, ThreadName};

let name = ThreadName::new("worker").unwrap();
let count = ThreadCount::new(4).unwrap();

assert_eq!(name.as_str(), "worker");
assert_eq!(count.get(), 4);
```
