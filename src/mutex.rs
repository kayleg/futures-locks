// vim: tw=80

use futures::{Async, Future, Poll};
use futures::sync::oneshot;
use std::cell::UnsafeCell;
use std::clone::Clone;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync;

/// An RAII mutex guard, much like `std::sync::MutexGuard`.  The wrapped data
/// can be accessed via its `Deref` and `DerefMut` implementations.
pub struct MutexGuard<T: ?Sized> {
    mutex: Mutex<T>
}

impl<T: ?Sized> Drop for MutexGuard<T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

impl<T: ?Sized> Deref for MutexGuard<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {&*self.mutex.inner.data.get()}
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *self.mutex.inner.data.get()}
    }
}

/// A `Future` representation a pending `Mutex` acquisition.
pub struct MutexFut<T: ?Sized> {
    /// Has this Future already acquired the Mutex?
    acquired: bool,
    receiver: Option<oneshot::Receiver<()>>,
    mutex: Mutex<T>,
}

impl<T: ?Sized> MutexFut<T> {
    fn new(rx: Option<oneshot::Receiver<()>>, mutex: Mutex<T>) -> Self {
        MutexFut{acquired: false, receiver: rx, mutex}
    }
}

impl<T: ?Sized> Drop for MutexFut<T> {
    fn drop(&mut self) {
        if ! self.acquired {
            if let Some(ref mut rx) = &mut self.receiver {
                rx.close();
                // TODO: futures-0.2.0 introduces a try_recv method that is
                // better to use here than poll.  Use it after upgrading to
                // futures >= 0.2.0
                match rx.poll() {
                    Ok(Async::Ready(())) => {
                        // This future received ownership of the mutex, but got
                        // dropped before it was ever polled.  Release the
                        // mutex.
                        self.mutex.unlock()
                    },
                    Ok(Async::NotReady) => {
                        // Dropping the Future before it acquires the Mutex is
                        // equivalent to cancelling it.
                    },
                    Err(oneshot::Canceled) => {
                        // Never received ownership of the mutex
                    }
                }
            } else {
                // Even though the future was immediately ready, it never got
                // polled.
                self.mutex.unlock();
            }
        }
    }
}

impl<T> Future for MutexFut<T> {
    type Item = MutexGuard<T>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.receiver.is_none() {
            self.acquired = true;
            Ok(Async::Ready(MutexGuard{mutex: self.mutex.clone()}))
        } else {
            match self.receiver.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                // It's impossible for receiver.poll() to return an error.  The
                // only way that would happen is if the sender got dropped.  But
                // that can't happen because the RwLock owns the sender, and the
                // Fut retains a clone of the RwLock
                Err(_) => unreachable!(),
                Ok(Async::Ready(_)) => {
                    self.acquired = true;
                    Ok(Async::Ready(MutexGuard{mutex: self.mutex.clone()}))
                }
            }
        }
    }
}

// LCOV_EXCL_START
#[derive(Debug)]
struct MutexData {
    owned: bool,
    // FIFO queue of waiting tasks.
    waiters: VecDeque<oneshot::Sender<()>>,
}
// LCOV_EXCL_STOP

// LCOV_EXCL_START
#[derive(Debug)]
struct Inner<T: ?Sized> {
    mutex: sync::Mutex<MutexData>,
    data: UnsafeCell<T>,
}
// LCOV_EXCL_STOP

/// A Futures-aware Mutex.
///
/// `std::sync::Mutex` cannot be used in an asynchronous environment like Tokio,
/// because a mutex acquisition can block an entire reactor.  This class can be
/// used instead.  It functions much like `std::sync::Mutex`.  Unlike that
/// class, it also has a builtin `Arc`, making it accessible from multiple
/// threads.  It's also safe to `clone`.  Also unlike `std::sync::Mutex`, this
/// class does not detect lock poisoning.
///
/// # Examples
///
/// ```
/// # extern crate futures;
/// # extern crate futures_locks;
/// # use futures_locks::*;
/// # use futures::executor::{Spawn, spawn};
/// # use futures::Future;
/// # fn main() {
/// let mtx = Mutex::<u32>::new(0);
/// let fut = mtx.lock().map(|mut guard| { *guard += 5; });
/// spawn(fut).wait_future();
/// assert_eq!(mtx.try_unwrap().unwrap(), 5);
/// # }
/// ```
// LCOV_EXCL_START
#[derive(Debug)]
pub struct Mutex<T: ?Sized> {
    inner: sync::Arc<Inner<T>>,
}
// LCOV_EXCL_STOP

impl<T: ?Sized> Clone for Mutex<T> {
    fn clone(&self) -> Mutex<T> {
        Mutex { inner: self.inner.clone()}
    }
}

impl<T> Mutex<T> {
    /// Create a new `Mutex` in the unlocked state.
    pub fn new(t: T) -> Mutex<T> {
        let mutex_data = MutexData {
            owned: false,
            waiters: VecDeque::new(),
        };
        let inner = Inner {
            mutex: sync::Mutex::new(mutex_data),
            data: UnsafeCell::new(t)
        };  //LCOV_EXCL_LINE    kcov false negative
        Mutex { inner: sync::Arc::new(inner)}
    }

    /// Consumes the `Mutex` and returns the wrapped data.  If the `Mutex` still
    /// has multiple references (not necessarily locked), returns a copy of
    /// `self` instead.
    pub fn try_unwrap(self) -> Result<T, Mutex<T>> {
        match sync::Arc::try_unwrap(self.inner) {
            Ok(inner) => Ok({
                // `unsafe` is no longer needed as of somewhere around 1.25.0.
                // https://github.com/rust-lang/rust/issues/35067
                #[allow(unused_unsafe)]
                unsafe { inner.data.into_inner() }
            }),
            Err(arc) => Err(Mutex {inner: arc})
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Returns a reference to the underlying data, if there are no other
    /// clones of the `Mutex`.
    ///
    /// Since this call borrows the `Mutex` mutably, no actual locking takes
    /// place -- the mutable borrow statically guarantees no locks exist.
    /// However, if the `Mutex` has already been cloned, then `None` will be
    /// returned instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate futures_locks;
    /// # use futures_locks::*;
    /// # fn main() {
    /// let mut mtx = Mutex::<u32>::new(0);
    /// *mtx.get_mut().unwrap() += 5;
    /// assert_eq!(mtx.try_unwrap().unwrap(), 5);
    /// # }
    /// ```
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if let Some(inner) = sync::Arc::get_mut(&mut self.inner) {
            let lock_data = inner.mutex.get_mut().unwrap();
            let data = unsafe { inner.data.get().as_mut() }.unwrap();
            debug_assert!(!lock_data.owned);
            Some(data)
        } else {
            None
        }
    }

    /// Acquires a `Mutex`, blocking the task in the meantime.  When the
    /// returned `Future` is ready, this task will have sole access to the
    /// protected data.
    pub fn lock(&self) -> MutexFut<T> {
        let mut mtx_data = self.inner.mutex.lock().expect("sync::Mutex::lock");
        if mtx_data.owned {
            let (tx, rx) = oneshot::channel::<()>();
            mtx_data.waiters.push_back(tx);
            return MutexFut::new(Some(rx), self.clone());
        } else {
            mtx_data.owned = true;
            return MutexFut::new(None, self.clone())
        }
    }

    /// Attempts to acquire the lock.
    ///
    /// If the operation would block, returns `Err` instead.  Otherwise, returns
    /// a guard (not a `Future`).
    ///
    /// # Examples
    /// ```
    /// # extern crate futures_locks;
    /// # use futures_locks::*;
    /// # fn main() {
    /// let mut mtx = Mutex::<u32>::new(0);
    /// match mtx.try_lock() {
    ///     Ok(mut guard) => *guard += 5,
    ///     Err(()) => println!("Better luck next time!")
    /// };
    /// # }
    /// ```
    pub fn try_lock(&self) -> Result<MutexGuard<T>, ()> {
        let mut mtx_data = self.inner.mutex.lock().expect("sync::Mutex::lock");
        if mtx_data.owned {
            Err(())
        } else {
            mtx_data.owned = true;
            Ok(MutexGuard{mutex: self.clone()})
        }
    }

    /// Release the `Mutex`
    fn unlock(&self) {
        let mut mtx_data = self.inner.mutex.lock().expect("sync::Mutex::lock");
        assert!(mtx_data.owned);
        if let Some(tx) = mtx_data.waiters.pop_front() {
            // Send ownership to the waiter
            tx.send(()).expect("Sender::send");
        } else {
            // Relinquish ownership
            mtx_data.owned = false;
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
