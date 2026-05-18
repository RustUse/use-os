# use-process-status

Primitive process status and lifecycle vocabulary.

`use-process-status` stores process state, optional numeric status codes, optional messages, and optional process identity as plain metadata. It does not execute processes, model stdout or stderr, define CLI exit-code constants, parse shell status, decode OS signals, log, or render diagnostics.

```rust
use use_process_id::ProcessId;
use use_process_status::{ProcessOutcome, ProcessState, ProcessStatus};

let status = ProcessStatus::new(ProcessState::Exited).with_status_code(0);
let outcome = ProcessOutcome::for_process(ProcessId::new(7).unwrap(), status);

assert_eq!(outcome.status().status_code(), Some(0));
```
