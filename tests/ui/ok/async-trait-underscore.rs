#[async_trait::async_trait]
trait TestTrait {
    async fn test(&self, value: String) -> String;
}

struct Test;

#[async_trait::async_trait]
impl TestTrait for Test {
    #[logcall::logcall]
    async fn test(&self, _: String) -> String {
        "test".to_string()
    }
}

fn main() {}
