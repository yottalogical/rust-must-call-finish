use must_call_finish::Example;

#[tokio::test]
async fn calls_finish() {
    let example = Example::new();
    example.do_something();
    example.finish().await;
}

#[test]
fn doesnt_call_finish() {
    let example = Example::new();
    example.do_something();
}
