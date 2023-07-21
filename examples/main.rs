#[logfn::logfn("info")]
fn foo(a: usize) -> usize {
    a + 1
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    foo(1);
}
