# logcall

[![Crates.io](https://img.shields.io/crates/v/logcall?style=flat-square&logo=rust)](https://crates.io/crates/logcall)
[![Crates.io](https://img.shields.io/crates/d/logcall?style=flat-square&logo=rust)](https://crates.io/crates/logcall)
[![Documentation](https://img.shields.io/docsrs/logcall?style=flat-square&logo=rust)](https://docs.rs/logcall/)
[![CI Status](https://img.shields.io/github/actions/workflow/status/fast/logcall/ci.yml?style=flat-square&logo=github)](https://github.com/fast/logcall/actions)
[![Crates.io](https://img.shields.io/crates/l/logcall?style=flat-square&logo=)](https://crates.io/crates/logcall)

`logcall` is a Rust procedural macro crate designed to automatically log function calls, their inputs, and their outputs. This macro facilitates debugging and monitoring by providing detailed logs of function executions with minimal boilerplate code.

This is a re-implementation of the [`log-derive`](https://crates.io/crates/log-derive) crate with [`async-trait`](https://crates.io/crates/async-trait) compatibility.

## Installation

Add `logcall` to your `Cargo.toml`:

```toml
[dependencies]
logcall = "0.1"
```

## Usage

Import the `logcall` crate and use the macro to annotate your functions:

```rust
use logcall::logcall;

/// Logs the function call at the default `debug` level.
#[logcall]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Logs the function call at the `info` level.
#[logcall("info")]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Logs `Ok` results at the `info` level and `Err` results at the `error` level.
#[logcall(ok = "info", err = "error")]
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Logs errors at the `error` level. No log output for `Ok` variant.
#[logcall(err = "error")]
fn divide2(a: usize, b: usize) -> Result<usize, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Logs the function call with custom input logging format.
#[logcall(input = "a = {a:?}, ..")]
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    add(2, 3);
    multiply(2, 3);
    divide(2, 0).ok();
    divide2(2, 0).ok();
    subtract(3, 2);
}
```

### Log Output

When the `main` function runs, it initializes the logger and logs each function call as specified:

```rust,ignore
[2024-06-19T15:01:23Z DEBUG main] main::add(a = 2, b = 3) => 5
[2024-06-19T15:01:23Z INFO  main] main::multiply(a = 2, b = 3) => 6
[2024-06-19T15:01:23Z ERROR main] main::divide(a = 2, b = 0) => Err("Division by zero")
[2024-06-19T15:01:23Z ERROR main] main::divide2(a = 2, b = 0) => Err("Division by zero")
[2024-06-19T15:01:23Z DEBUG main] main::subtract(a = 3, ..) => 1
```

## Customization

- **Default Log Level**: If no log level is specified, `logcall` logs at the `debug` level:
  ```rust,ignore
  #[logcall]
  ```
- **Specify Log Level**: Use the macro parameters to specify log level:
  ```rust,ignore
  #[logcall("info")]
- **Specify Log Levels for `Result`**: Use the `ok` and `err` parameters to specify log levels for `Ok` and `Err` variants:
  ```rust,ignore
  #[logcall(err = "error")]
  #[logcall(ok = "info", err = "error")]
  ```
- **Customize Input Logging**: Use the `input` parameter to customize the input log format:
  ```rust,ignore
  #[logcall(input = "a = {a:?}, ..")]
  #[logcall("info", input = "a = {a:?}, ..")]
  #[logcall(ok = "info", err = "error", input = "a = {a:?}, ..")]
  ```

## Contributing

Contributions are welcome! Please submit pull requests or open issues to improve the crate.

## License

This project is licensed under the MIT License.
