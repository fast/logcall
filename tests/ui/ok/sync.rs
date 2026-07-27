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

#[logcall::logcall(some = "info")]
fn j(a: u32) -> Option<u32> {
    Some(a)
}

#[logcall::logcall(none = "info")]
fn k(a: u32) -> Option<u32> {
    Some(a)
}

#[logcall::logcall(some = "info", none = "info")]
fn l(a: u32) -> Option<u32> {
    Some(a)
}

#[logcall::logcall]
unsafe fn m(a: u32) -> u32 {
    a
}

fn main() {
    f(1);
    g(1).ok();
    h(1).ok();
    i(1).ok();
    j(1).unwrap();
    k(1).unwrap();
    l(1).unwrap();
    unsafe {
        m(1);
    }
}
