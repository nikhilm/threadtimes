#[macro_use]
extern crate criterion;

extern crate threadtimes;

use criterion::Criterion;
use std::io::Write;
use std::sync::{Arc, Barrier};

use threadtimes::{get_all_thread_times, get_thread_times, thread_self, ticks_per_second};

fn criterion_benchmark_single(c: &mut Criterion) {
    let tps = ticks_per_second();
    c.bench_function("thread time", move |b| {
        b.iter(|| get_thread_times(thread_self(), tps))
    });
}

fn criterion_benchmark_iter_1(c: &mut Criterion) {
    let tps = ticks_per_second();
    c.bench_function("one thread time", move |b| {
        b.iter(|| get_all_thread_times(tps))
    });
}

fn criterion_benchmark_iter_100(c: &mut Criterion) {
    let tps = ticks_per_second();
    let mut threads = Vec::with_capacity(100);
    let barrier = Arc::new(Barrier::new(threads.capacity() + 1));
    for i in 0..threads.capacity() {
        let barrier2 = barrier.clone();
        threads.push(std::thread::spawn(move || {
            let mut devnull = std::fs::OpenOptions::new()
                .write(true)
                .open("/dev/null")
                .unwrap();
            for i in 0..1000 {
                writeln!(devnull, "{}", (1..100000).sum::<u64>());
            }
            barrier2.wait();
        }));
    }

    c.bench_function("100 thread time", move |b| {
        b.iter(|| get_all_thread_times(tps))
    });

    barrier.wait();
    for thread in threads {
        thread.join().unwrap();
    }
}

criterion_group!(
    benches,
    criterion_benchmark_single,
    criterion_benchmark_iter_1,
    criterion_benchmark_iter_100,
);
criterion_main!(benches);
