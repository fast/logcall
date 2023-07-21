#![feature(async_fn_in_trait)]
#![allow(unused_mut)]

trait MyTrait {
    async fn work(&self) -> usize;
}

struct MyStruct;

impl MyTrait for MyStruct {
    #[logcall::logcall("debug")]
    #[logcall::logcall("debug")]
    async fn work(&self) -> usize {
        1
    }
}

fn main() {}
