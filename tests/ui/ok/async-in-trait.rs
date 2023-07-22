#![cfg_attr(all(feature = "nightly", test), feature(async_fn_in_trait))]

#[cfg(all(feature = "nightly", test))]
mod tests {
    trait MyTrait {
        async fn work(&self) -> Result<usize, usize>;
    }
    
    struct MyStruct;
    
    impl MyTrait for MyStruct {
        #[logcall::logcall("debug")]
        #[logcall::logcall(ok = "debug", err = "error")]
        async fn work(&self) -> Result<usize, usize> {
            Ok(1)
        }
    }
}

fn main() {}
