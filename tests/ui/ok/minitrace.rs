#[logcall::logcall]
#[minitrace::trace]
fn f() {}

#[logcall::logcall]
#[minitrace::trace]
async fn g() {}

fn main() {
    f();
    pollster::block_on(g());
}
