#[logcall::logcall("info")]
fn f(a: u32) -> u32 {
    a
}

#[logcall::logcall(ok = "info")]
fn g(a: u32) -> Result<u32, u32> {
    Ok(a)
}

#[logcall::logcall(err = "info")]
fn h(a: u32) -> Result<u32, u32> {
    Ok(a)
}

#[logcall::logcall(ok = "info", err = "info")]
fn i(a: u32) -> Result<u32, u32> {
    Ok(a)
}

fn main() {
    f(1);
    g(1).ok();
    h(1).ok();
    i(1).ok();
}
