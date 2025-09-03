use core::{
    cell::UnsafeCell,
    future::{poll_fn, Future},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    task::Poll,
};

use embassy_sync::waitqueue::AtomicWaker;

/// RawAtomicMutex must not be used in blocking mutexes otherwise
/// deadlock will occur. Use only within embassy_sync::mutex
///
pub struct AtomicMutex<T> {
    locked: AtomicBool,
    waker: AtomicWaker,
    inner: UnsafeCell<T>,
}

impl<T> AtomicMutex<T> {
    pub const fn new(value: T) -> Self {
        AtomicMutex {
            locked: AtomicBool::new(false),
            waker: AtomicWaker::new(),
            inner: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> impl Future<Output = AtomicMutexGuard<'_, T>> {
        poll_fn(|cx| {
            let locked = self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok();

            if locked {
                Poll::Ready(AtomicMutexGuard { mutex: self })
            } else {
                self.waker.register(cx.waker());
                Poll::Pending
            }
        })
    }
}

/// Safety: AtomicMutex ensures unique access
unsafe impl<T> Send for AtomicMutex<T> {}
unsafe impl<T> Sync for AtomicMutex<T> {}

#[clippy::has_significant_drop]
#[must_use = "if unused the Mutex will immediately unlock"]
pub struct AtomicMutexGuard<'mutex, T> {
    mutex: &'mutex AtomicMutex<T>,
}

impl<'a, T> Drop for AtomicMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
        self.mutex.waker.wake();
    }
}

impl<'a, T> Deref for AtomicMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: the AtomicMutexGuard represents exclusive access to the contents
        // of the mutex, so it's OK to get it.
        unsafe { &*(self.mutex.inner.get() as *const T) }
    }
}

impl<'a, T> DerefMut for AtomicMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the AtomicMutexGuard represents exclusive access to the contents
        // of the mutex, so it's OK to get it.
        unsafe { &mut *(self.mutex.inner.get()) }
    }
}
