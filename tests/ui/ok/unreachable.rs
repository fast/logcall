#[logcall::logcall("info")]
async fn f(a: u32) -> u32 {
    if a == 1 {
        return 1;
    }

    unreachable!()
}

#[tokio::main]
async fn main() {
    f(1).await;
}
