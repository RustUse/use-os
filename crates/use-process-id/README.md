# use-process-id

Primitive process identity types.

`use-process-id` wraps numeric process identifiers as plain validated values. It does not inspect running processes, query the current process ID, spawn processes, kill processes, or wrap platform-specific process handles.

```rust
use use_process_id::ProcessId;

let process_id = ProcessId::new(42).unwrap();

assert_eq!(process_id.get(), 42);
assert_eq!(process_id.to_string(), "42");
```
