#[macro_use]
extern crate criterion;

extern crate transpose;

use criterion::{Criterion, ParameterizedBenchmark, Throughput};
use std::mem;

fn bench_transpose<T: Copy + Default>(c: &mut Criterion, tyname: &str) {
    let ref sizes = [(4, 4), (8, 8), (16, 16), (64, 64), (256, 256), (1024, 1024), (4096, 4096)];

    let bench = ParameterizedBenchmark::new(
        format!("transpose {}", tyname),
        |b, &&(width, height)| {
            let mut buffer = vec![T::default(); width * height];
            let mut scratch = vec![T::default(); width * height];

            b.iter(|| { transpose::transpose(&mut buffer, &mut scratch, width, height); });
        },
        sizes)
        .throughput(|&&(width, height)| Throughput::Bytes((width * height * mem::size_of::<T>()) as u32));

    c.bench("square transposes", bench);
}

fn bench_u8(c: &mut Criterion) { bench_transpose::<u8>(c, "u8") }
fn bench_u32(c: &mut Criterion) { bench_transpose::<u32>(c, "u32") }
fn bench_u64(c: &mut Criterion) { bench_transpose::<u64>(c, "u64") }
fn bench_u128(c: &mut Criterion) { bench_transpose::<u128>(c, "u128") }

criterion_group!(benches, bench_u8, bench_u32, bench_u64, bench_u128);
criterion_main!(benches);
