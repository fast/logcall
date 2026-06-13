use logcall::logcall;
use logforth::record::LevelFilter;

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
    a.checked_div(b).ok_or("divide by zero".into())
}

/// Logs `Some` values at the `info` level and `None` values at the `warn` level.
#[logcall(some = "info", none = "warn")]
fn find_even(value: i32) -> Option<i32> {
    (value % 2 == 0).then_some(value)
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
    logforth::starter_log::stdout()
        .filter(LevelFilter::All)
        .apply();

    add(2, 3);
    multiply(2, 3);
    divide(2, 0).ok();
    divide2(2, 0).ok();
    find_even(4);
    find_even(3);
    subtract(3, 2);
    negate(5);
    ping(42);
}
