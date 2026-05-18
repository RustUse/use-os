# use-thread-id

Primitive thread identity vocabulary.

`use-thread-id` models stable thread labels or numeric thread-like identifiers as plain data. It does not create, inspect, name, manage, or pool threads, and it does not depend on unstable thread ID internals.

```rust
use use_thread_id::ThreadIdLabel;

let label = ThreadIdLabel::label("worker-1").unwrap();
let number = ThreadIdLabel::number(1);

assert_eq!(label.to_string(), "worker-1");
assert_eq!(number.to_string(), "1");
```
