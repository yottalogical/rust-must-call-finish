use std::ops::{Deref, DerefMut};
use std::thread::panicking;

use async_trait::async_trait;

#[async_trait]
pub trait AsyncDrop {
    async fn async_drop(&mut self);
}

pub struct AsyncDropper<T: AsyncDrop> {
    item: T,
    finished: bool,
}

impl<T: AsyncDrop> AsyncDropper<T> {
    pub fn new(item: T) -> Self {
        Self {
            finished: false,
            item,
        }
    }

    pub async fn finish(mut self) {
        self.item.async_drop().await;
        self.finished = true;
    }
}

impl<T: AsyncDrop> Deref for AsyncDropper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: AsyncDrop> DerefMut for AsyncDropper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T: AsyncDrop> Drop for AsyncDropper<T> {
    fn drop(&mut self) {
        if !self.finished && !panicking() {
            panic!(
                "AsyncDropper must not be dropped implicitly. Call `AsyncDropper::finish` instead."
            );
        }
    }
}
