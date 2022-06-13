use std::thread::panicking;

pub struct Example {
    finished: bool,
}

impl Example {
    pub fn new() -> Self {
        Example { finished: false }
    }

    pub fn do_something(&self) {}

    pub async fn finish(mut self) {
        self.finished = true
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        if !self.finished && !panicking() {
            panic!("Example must not be dropped implicitly. Instead call Example::finish.");
        }
    }
}
