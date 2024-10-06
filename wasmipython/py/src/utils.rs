use std::{cell::LazyCell, ops::Deref};

pub struct SyncNonsyncLazy<T> {
    inner: LazyCell<T, fn() -> T>,
}
unsafe impl<T> Sync for SyncNonsyncLazy<T> {}
unsafe impl<T> Send for SyncNonsyncLazy<T> {}
impl<T> Deref for SyncNonsyncLazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}
impl<T> SyncNonsyncLazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self {
            inner: LazyCell::new(f),
        }
    }
}
