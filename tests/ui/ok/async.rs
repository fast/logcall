#[logcall::logcall("info")]
async fn f(a: u32) -> u32 {
    a
}

#[logcall::logcall(ok = "info")]
async fn g(a: u32) -> Result<u32, u32> {
    Ok(a)
}

#[logcall::logcall(err = "info")]
async fn h(a: u32) -> Result<u32, u32> {
    Ok(a)
}

#[logcall::logcall(ok = "info", err = "info")]
async fn i(a: u32) -> Result<u32, u32> {
    Ok(a)
}

#[tokio::main]
async fn main() {
    f(1).await;
    g(1).await.ok();
    h(1).await.ok();
    i(1).await.ok();
}
