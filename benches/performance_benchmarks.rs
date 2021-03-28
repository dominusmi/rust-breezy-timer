/// Simple benchmark to get an idea of the latency due to the timer when running in a loop

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use breezy_timer::*;
use std::mem::take;
use global::Global;

prepare_timer!();


fn sum_normal(vec: &Vec<u8>, iterations: usize) -> u32 {
    let mut total = 0;
    for _ in 0..iterations {
        for v in black_box(vec) {
            total += *v as u32;
        }
    }
    total
}

fn sum_timed(vec: &Vec<u8>, iterations: usize) -> u32 {
    let mut total = 0;
    for _ in 0..iterations {
        start_timer!("sum");
        for v in black_box(vec) {
            total += *v as u32
        }
        stop_timer!("sum");
    }
    total
}


fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("repeated sum of vector");
    let vec: Vec<u8> = (0..102400).map(|_| { rand::random::<u8>() }).collect();

    // size: number of times to sum the vector
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("normal", *size), &vec,
                               |b, vec| b.iter(|| sum_normal(vec, *size)));

        group.bench_with_input(BenchmarkId::new("timed", *size), &vec,
                               |b, vec| b.iter(|| sum_timed(vec, *size)));
    }
    group.finish();
}

fn start_and_stop_timer(name: &'static str) {
    start_timer!(name);
    stop_timer!(name);
}

fn bench_timer(c: &mut Criterion) {
    // size: number of times to sum the vector
    prepare_timer!();
    c.bench_with_input(BenchmarkId::new("start and stop timer", &"foo"),
                       &"foo",|b, name| b.iter(|| start_and_stop_timer(name)) );

}

criterion_group!(benches, bench_sum, bench_timer);
criterion_main!(benches);