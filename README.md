# logcall

An attribute macro that logs the function input & return values.

This is a reimplementation of the [`log-derive`](https://crates.io/crates/log-derive) crate with [`async-trait`](https://crates.io/crates/async-trait) compatibility.

## Usage

```rust
use logcall::logcall;

#[logcall("info")]
fn foo(a: usize) -> usize {
    a + 1
}

#[logcall(err = "error")]
fn bar(a: usize) -> Result<usize, usize> {
    Err(a + 1)
}

#[logcall(ok = "info", err = "error")]
fn baz(a: usize) -> Result<usize, usize> {
    Ok(a + 1)
}

fn main() {
    structured_logger::Builder::new().init();
    foo(1);
    bar(1).ok();
    baz(1).ok();
}

// prints:
// [2023-07-22T06:55:10Z INFO  main] foo() => 2
// [2023-07-22T06:55:10Z ERROR main] bar() => Err(2)
// [2023-07-22T06:55:10Z INFO  main] baz() => Ok(2)
```
