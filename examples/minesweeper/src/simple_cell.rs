use std::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

use cc_wasm_api::prelude::yield_now;

pub struct SimpleCell<T>(UnsafeCell<Option<T>>);
unsafe impl<T> Send for SimpleCell<T> {}
unsafe impl<T> Sync for SimpleCell<T> {}
impl<T> Deref for SimpleCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let r = unsafe { &*self.0.get() };
        r.as_ref().unwrap()
    }
}

impl<T> DerefMut for SimpleCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.get_mut().as_mut().unwrap()
    }
}

impl<T: Sized> SimpleCell<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }
    pub fn init(&self, value: T) {
        unsafe { *self.0.get() = Some(value) };
    }
    #[allow(clippy::mut_from_ref)]
    pub fn get(&self) -> &mut T {
        unsafe { (*self.0.get()).as_mut().unwrap() }
    }
}
#[derive(Debug)]
pub struct Syncer(Cell<usize>);
impl Syncer {
    pub const fn new() -> Self {
        Syncer(Cell::new(0))
    }
    pub fn notify(&self) {
        self.0.set(self.0.get() + 1);
    }
    pub async fn wait(&self, count: usize) {
        loop {
            // show_str(&format!("{}", self.0.get()));
            if self.0.get() >= count {
                self.0.set(self.0.get() - count);
                break;
            } else {
                yield_now().await;
            }
        }
    }
}
