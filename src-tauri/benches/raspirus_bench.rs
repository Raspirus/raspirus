use criterion::{black_box, criterion_group, criterion_main, Criterion};
// TODO: Add the function you want to benchmark here.
// Basically we could downlaod the database, prepare the database, and then
// benchmark a specific folder or file. We could also always ship the test folder with it or something
// Also, remember to test zip files too.
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);