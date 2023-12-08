use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Single threaded mutex used across JS async promises
pub struct Mutex<T: ?Sized> {
    wakers: RefCell<VecDeque<Waker>>,
    value: RefCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(inner: T) -> Self {
        Self {
            value: RefCell::new(inner),
            wakers: Default::default(),
        }
    }

    pub fn lock(&self) -> LockFuture<'_, T> {
        LockFuture { mutex: self }
    }
}

pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a Mutex<T>,
    value: RefMut<'a, T>,
}

pub struct LockFuture<'a, T: ?Sized> {
    mutex: &'a Mutex<T>,
}

impl<'a, T: ?Sized> Future for LockFuture<'a, T> {
    type Output = MutexGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.mutex.value.try_borrow_mut() {
            Ok(mut_ref) => Poll::Ready(MutexGuard {
                mutex: self.mutex,
                value: mut_ref,
            }),
            Err(_) => {
                self.mutex.wakers.borrow_mut().push_back(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(waker) = self.mutex.wakers.borrow_mut().pop_front() {
            waker.wake();
        }
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Mutex;
    use futures::channel::mpsc;
    use futures::StreamExt;
    use std::rc::Rc;
    use std::time::Duration;
    use wasm_bindgen_futures::spawn_local;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn sequential() {
        let (tx, mut rx) = mpsc::unbounded();
        let lock_order = Rc::new(Mutex::new(()));

        for i in 0..10 {
            let lock_order = lock_order.clone();
            let tx = tx.clone();
            spawn_local(async move {
                let _guard = lock_order.lock().await; // ensure order

                gloo_timers::future::sleep(Duration::from_millis(10 * (10 - i))).await;
                let _ = tx.unbounded_send(i);
            });
        }

        for i in 0..10 {
            let value = rx.next().await.unwrap();
            assert_eq!(value, i);
        }
    }
}
