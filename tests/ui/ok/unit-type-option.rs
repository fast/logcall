// Test for issue: logcall should not trigger clippy::ignored_unit_patterns warning
// for Option<()> return types

#![warn(clippy::pedantic)]

#[logcall::logcall(some = "debug", none = "info")]
pub async fn maybe() -> Option<()> {
    Some(())
}

#[logcall::logcall(some = "info")]
pub async fn some_only() -> Option<()> {
    Some(())
}

#[logcall::logcall(none = "info")]
pub async fn none_only() -> Option<()> {
    None
}

#[logcall::logcall(some = "debug", none = "info")]
pub fn sync_maybe() -> Option<()> {
    Some(())
}

#[tokio::main]
async fn main() {
    let _ = maybe().await;
    let _ = some_only().await;
    let _ = none_only().await;
    let _ = sync_maybe();
}
