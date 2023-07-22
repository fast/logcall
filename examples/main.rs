use logcall::logcall;

#[logcall("info")]
fn foo(a: usize) -> usize {
    a + 1
}

#[logcall(err = "error")]
fn bar(a: usize) -> Result<usize, usize> {
    Err(a + 1)
}

#[logcall(ok = "info", err = "error")]
fn baz(a: usize) -> Result<usize, usize> {
    Ok(a + 1)
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    foo(1);
    bar(1).ok();
    baz(1).ok();
}
