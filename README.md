# Logcall

[![Crates.io](https://img.shields.io/crates/v/logcall?style=flat-square&logo=rust)](https://crates.io/crates/logcall)
[![Downloads](https://img.shields.io/crates/d/logcall?style=flat-square&logo=rust)](https://crates.io/crates/logcall)
[![Documentation](https://img.shields.io/docsrs/logcall?style=flat-square&logo=rust)](https://docs.rs/logcall/)
[![CI Status](https://img.shields.io/github/actions/workflow/status/fast/logcall/ci.yml?style=flat-square&logo=github)](https://github.com/fast/logcall/actions)
[![License](https://img.shields.io/crates/l/logcall?style=flat-square&logo=)](https://crates.io/crates/logcall)

Logcall is a Rust procedural macro crate that automatically logs function calls, their inputs, and outputs. It keeps boilerplate low while making debugging and observability easy.

This is a re-implementation of [`log-derive`](https://crates.io/crates/log-derive) with [`async-trait`](https://crates.io/crates/async-trait) compatibility.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
logcall = "0.1"
```

## Quick Start

Annotate functions with `#[logcall]` and configure logging with `logforth`:

```rust
use logcall::logcall;
use logforth::append;
use logforth::filter::EnvFilter;

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

/// Logs the function call with custom output logging format.
#[logcall(output = ": {:?}")]
fn negate(a: i32) -> i32 {
    -a
}

/// Omits the return value from the log output.
#[logcall(output = "")]
fn ping(a: i32) -> i32 {
    a
}

fn main() {
    logforth::builder()
        .dispatch(|d| {
            d.filter(EnvFilter::from_default_env_or("trace"))
                .append(append::Stderr::default())
        })
        .apply();

    add(2, 3);
    multiply(2, 3);
    divide(2, 0).ok();
    divide2(2, 0).ok();
    subtract(3, 2);
    negate(5);
    ping(42);
}
```

### Example Run

```bash
cargo run --example main
```

Sample output (from 2025-12-11):

```plaintext
2025-12-11T23:08:39.201289+08:00[Asia/Shanghai] DEBUG main: main.rs:6 main::add(a = 2, b = 3) => 5
2025-12-11T23:08:39.211065+08:00[Asia/Shanghai]  INFO main: main.rs:12 main::multiply(a = 2, b = 3) => 6
2025-12-11T23:08:39.211086+08:00[Asia/Shanghai] ERROR main: main.rs:18 main::divide(a = 2, b = 0) => Err("Division by zero")
2025-12-11T23:08:39.211118+08:00[Asia/Shanghai] ERROR main: main.rs:28 main::divide2(a = 2, b = 0) => Err("Division by zero")
2025-12-11T23:08:39.211148+08:00[Asia/Shanghai] DEBUG main: main.rs:38 main::subtract(a = 3, ..) => 1
2025-12-11T23:08:39.211162+08:00[Asia/Shanghai] DEBUG main: main.rs:44 main::negate(a = 5): -5
2025-12-11T23:08:39.211172+08:00[Asia/Shanghai] DEBUG main: main.rs:50 main::ping(a = 42)
```

## Minimum Supported Rust Version (MSRV)

This crate is built against the latest stable release, and its minimum supported rustc version is 1.80.0.

The policy is that the minimum Rust version required to use this crate can be increased in minor version updates. For example, if Logcall 1.0 requires Rust 1.20.0, then Logcall 1.0.z for all values of z will also require Rust 1.20.0 or newer. However, Logcall 1.y for y > 0 may require a newer minimum version of Rust.

## Contributing

Contributions are welcome! Please submit pull requests or open issues to improve the crate.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
