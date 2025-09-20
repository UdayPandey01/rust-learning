use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fib_benchmark::{fibonacci_fast, fibonacci_slow}; // Use your library crate

// TODO: 1. Create a benchmark function named `run_benchmarks`.
fn run_benchmarks(c : &mut Criterion) {
    c.bench_function("fibonacci_slow_20", |b| b.iter(|| fibonacci_slow(black_box(20))));
    c.bench_function("fibonacci_fast_20", |b| b.iter(|| fibonacci_fast(black_box(20))));
}

// TODO: 4. Use the `criterion_group!` and `criterion_main!` macros
criterion_group!(benches, run_benchmarks);
criterion_main!(benches);
// to set up and run your benchmark function.