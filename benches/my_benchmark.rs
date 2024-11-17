use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Faster than the recursive version
fn fibonacci_iter(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b += tmp;
    }
    a
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(30))));
    // c.bench_function("fib_iter 20", |b| b.iter(|| fibonacci_iter(black_box(30))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
