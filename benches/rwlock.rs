#![feature(test)]

extern crate futures;
extern crate futures_locks;
extern crate test;
extern crate tokio_ as tokio;

use futures::executor::spawn;
use futures::Future;
use futures_locks::*;
use test::Bencher;

/// Benchmark the speed of acquiring a read lock for an uncontested `RwLock`
#[bench]
fn bench_rwlock_read_uncontested(bench: &mut Bencher) {
    let rwlock = RwLock::<()>::new(());

    bench.iter(|| {
        spawn(rwlock.read().map(|_guard| ())).wait_future().unwrap();
    });
}

/// Benchmark the speed of acquiring a read lock for a contested `RwLock`
#[bench]
fn bench_rwlock_read_contested(bench: &mut Bencher) {
    let rwlock = RwLock::<()>::new(());

    bench.iter(|| {
        let fut0 = rwlock.read().map(|_guard| ());
        let fut1 = rwlock.read().map(|_guard| ());
        spawn(fut0.join(fut1)).wait_future().unwrap();
        //spawn(rwlock.lock().map(|_guard| ())).wait_future();
    });
}

/// Benchmark the speed of acquiring a write lock for an uncontested `RwLock`
#[bench]
fn bench_rwlock_write_uncontested(bench: &mut Bencher) {
    let rwlock = RwLock::<()>::new(());

    bench.iter(|| {
        spawn(rwlock.write().map(|_guard| ()))
            .wait_future()
            .unwrap();
    });
}

/// Benchmark the speed of acquiring a write lock for a contested `RwLock`
#[bench]
fn bench_rwlock_write_contested(bench: &mut Bencher) {
    let rwlock = RwLock::<()>::new(());

    bench.iter(|| {
        let fut0 = rwlock.write().map(|_guard| ());
        let fut1 = rwlock.write().map(|_guard| ());
        spawn(fut0.join(fut1)).wait_future().unwrap();
        //spawn(rwlock.lock().map(|_guard| ())).wait_future();
    });
}
