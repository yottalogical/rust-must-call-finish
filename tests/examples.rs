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

#[tokio::test]
async fn sometimes_correct() {
    // Change this to false for it to panic
    // The fact it will/won't panic depending on a value is undesirable
    static FINISH_CORRECTLY: bool = true;

    let example = Example::new();
    example.do_something();

    if FINISH_CORRECTLY {
        example.finish().await;
    }
}
