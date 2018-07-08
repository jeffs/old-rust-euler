#[macro_use]
extern crate criterion;

extern crate rust_euler;

use rust_euler::prime;

use criterion::Criterion;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
}

fn criterion_benchmark_prime_generate10(c: &mut Criterion) {
    c.bench_function("prime generate 10", |b| b.iter(|| prime::generate(10)));
}

fn criterion_benchmark_prime_generate100(c: &mut Criterion) {
    c.bench_function("prime generate 100", |b| b.iter(|| prime::generate(100)));
}

fn criterion_benchmark_prime_generate1000(c: &mut Criterion) {
    c.bench_function("prime generate 1000", |b| b.iter(|| prime::generate(1000)));
}

fn criterion_benchmark_prime_generate10000(c: &mut Criterion) {
    c.bench_function("prime generate 10000", |b| {
        b.iter(|| prime::generate(10000))
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    criterion_benchmark_prime_generate10,
    criterion_benchmark_prime_generate100,
    criterion_benchmark_prime_generate1000,
    criterion_benchmark_prime_generate10000,
);

criterion_main!(benches);
