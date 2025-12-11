#[logcall::logcall(output = " output = {}")]
fn output_custom(a: u32) -> u32 {
    a + 1
}

#[logcall::logcall(output = "")]
fn output_suppressed(a: u32) -> u32 {
    a
}

fn main() {
    output_custom(1);
    output_suppressed(3);
}
