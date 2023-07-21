# logcall

An attribute macro that logs the return value from function call.

This is a reimplementation of the [`log-derive`](https://crates.io/crates/log-derive) crate with [`async-trait`](https://crates.io/crates/async-trait) compatibility.

## Usage

```rust
use logcall::logcall;

#[logcall("info")]
fn foo(a: usize) -> usize {
    a + 1
}

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    foo(1);
}

// prints:
// [2023-07-21T12:57:43Z INFO  main] foo() => 2
```
