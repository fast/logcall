#[logfn::logfn("info")]
fn f(mut a: u32) -> u32 {
    a += 1;
    a
}

fn main() {
    f(1);
}
