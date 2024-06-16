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
    subtract(2, 3);
}
