pub struct Example {
    finished: bool,
}

impl Example {
    pub fn new() -> Self {
        Example { finished: false }
    }

    pub fn do_something(&self) {}

    pub async fn finish(mut self) {
        self.async_drop().await;
        self.finished = true;
    }

    async fn async_drop(&mut self) {}
}

impl Drop for Example {
    fn drop(&mut self) {
        if !self.finished {
            futures::executor::block_on(self.async_drop());
        }
    }
}
