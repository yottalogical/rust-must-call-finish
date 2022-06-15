use async_drop::{AsyncDrop, AsyncDropper};
use async_trait::async_trait;
use tokio::time::Duration;

struct Example<'a>(&'a mut i32);

impl<'a> Example<'a> {
    async fn do_something(&mut self) {
        tokio::time::sleep(Duration::from_millis(10)).await;
        *self.0 = 40;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[async_trait]
impl<'a> AsyncDrop for Example<'a> {
    async fn async_drop(&mut self) {
        tokio::time::sleep(Duration::from_millis(10)).await;
        *self.0 += 2;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn calls_finish() {
    let mut num = 0;

    {
        let mut example = AsyncDropper::new(Example(&mut num));
        example.do_something().await;
        example.finish().await;
    }

    assert_eq!(num, 42);
}

#[tokio::test]
#[should_panic(
    expected = "AsyncDropper must not be dropped implicitly. Call `AsyncDropper::finish` instead."
)]
async fn doesnt_call_finish() {
    let mut num = 0;

    {
        let mut example = AsyncDropper::new(Example(&mut num));
        example.do_something().await;
    }
}

#[tokio::test]
#[should_panic(expected = "the Disco")]
async fn unwinds() {
    let mut num = 0;

    {
        let mut example = AsyncDropper::new(Example(&mut num));

        if true {
            panic!("the Disco");
        }

        example.do_something().await;
        example.finish().await;
    }
}
