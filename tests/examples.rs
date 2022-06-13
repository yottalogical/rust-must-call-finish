use must_call_finish::Example;

#[tokio::test]
async fn correct() {
    let example = Example::new();
    example.do_something();

    example.finish().await;
}

#[test]
#[should_panic]
fn incorrect() {
    let example = Example::new();
    example.do_something();
}
