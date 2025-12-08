// Test for issue: logcall should not trigger clippy::ignored_unit_patterns warning
// for Result<(), E> return types

#![warn(clippy::pedantic)]

#[logcall::logcall(ok = "debug", err = "error")]
pub async fn start() -> Result<(), String> {
    Ok(())
}

#[logcall::logcall(ok = "info")]
pub async fn ok_only() -> Result<(), String> {
    Ok(())
}

#[logcall::logcall(err = "error")]
pub async fn err_only() -> Result<(), String> {
    Ok(())
}

#[logcall::logcall(ok = "debug", err = "error")]
pub fn sync_start() -> Result<(), String> {
    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = start().await;
    let _ = ok_only().await;
    let _ = err_only().await;
    let _ = sync_start();
}
