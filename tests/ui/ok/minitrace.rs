#[logcall::logcall]
#[minitrace::trace]
fn f() {}

#[logcall::logcall]
#[minitrace::trace]
async fn g() {
    std::future::ready(1).await;
}

fn main() {
    f();
    pollster::block_on(g());
}
