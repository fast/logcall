#[logcall::logcall("info", ok = "info")]
fn f0() {}

#[logcall::logcall("info", err = "info")]
fn f1() {}

#[logcall::logcall("info", ok = "info", err = "info")]
fn f2() {}

#[logcall::logcall("info", some = "info")]
fn f3() {}

#[logcall::logcall("info", none = "info")]
fn f4() {}

#[logcall::logcall("info", some = "info", none = "info")]
fn f5() {}

#[logcall::logcall(some = "info", ok = "info")]
fn f6() {}

#[logcall::logcall(ok = "info", some = "info")]
fn f7() {}

#[logcall::logcall(some = "info", none = "info", ok = "info", err = "info")]
fn f8() {}

#[logcall::logcall("info", some = "info", none = "info", ok = "info", err = "info")]
fn f9() {}

fn main() {}
