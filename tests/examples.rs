use async_drop::{AsyncDrop, AsyncDropper};
use async_trait::async_trait;
use tokio::time::Duration;

struct Example;

impl Example {
    pub async fn do_something(&self) {
        println!("do_something: start");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("do_something: done");
    }
}

#[async_trait]
impl AsyncDrop for Example {
    async fn async_drop(&mut self) {
        println!("async_drop: start");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("async_drop: done");
    }
}

#[tokio::test]
async fn calls_finish() {
    let example = AsyncDropper::new(Example);
    example.do_something().await;
    example.finish().await;
}

#[tokio::test]
#[should_panic(
    expected = "AsyncDropper must not be dropped implicitly. Instead call AsyncDropper::finish."
)]
async fn doesnt_call_finish() {
    let example = AsyncDropper::new(Example);
    example.do_something().await;
}
