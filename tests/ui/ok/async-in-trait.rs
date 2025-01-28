trait MyTrait {
    async fn work(&self) -> Result<usize, usize>;
    async fn run(&self) -> Option<usize>;
}

struct MyStruct;

impl MyTrait for MyStruct {
    #[logcall::logcall("debug")]
    #[logcall::logcall(ok = "debug", err = "error")]
    async fn work(&self) -> Result<usize, usize> {
        Ok(1)
    }

    #[logcall::logcall("debug")]
    #[logcall::logcall(some = "debug", none = "error")]
    async fn run(&self) -> Option<usize> {
        Some(1)
    }
}

fn main() {}
