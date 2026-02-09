use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use fuscum::kgram::{Kgram, RollingHashKgram, StdHashKgram};
use std::hint::black_box;

/// Generate test data of specified size
fn generate_test_data(size: usize) -> Vec<u8> {
    // Generate pseudo-random data that resembles real text
    // Using a simple repeating pattern with some variation
    (0..size)
        .map(|i| {
            let base = (i % 26) as u8 + b'a';
            let variation = ((i / 26) % 10) as u8;
            base.wrapping_add(variation)
        })
        .collect()
}

fn bench_kgram_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("kgram_throughput");

    // Test with different data sizes (in bytes)
    let sizes = vec![1_000, 10_000];
    let k = 35; // Standard k-gram size

    for size in sizes {
        let data = generate_test_data(size);

        // Set throughput to measure bytes per second
        group.throughput(Throughput::Bytes(size as u64));

        // Benchmark StdHashKgram
        group.bench_with_input(BenchmarkId::new("StdHashKgram", size), &data, |b, data| {
            b.iter(|| {
                let kgram = StdHashKgram;
                black_box(kgram.k_gram(black_box(data), black_box(k)))
            });
        });

        // Benchmark RollingHashKgram
        group.bench_with_input(
            BenchmarkId::new("RollingHashKgram", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let kgram: RollingHashKgram<257, { u64::MAX }> = RollingHashKgram;
                    black_box(kgram.k_gram(black_box(data), black_box(k)))
                });
            },
        );
    }

    group.finish();
}

fn bench_kgram_varying_k(c: &mut Criterion) {
    let mut group = c.benchmark_group("kgram_varying_k");

    // Fixed data size, varying k values
    let size = 100_000;
    let data = generate_test_data(size);
    let k_values = vec![10, 50];

    group.throughput(Throughput::Bytes(size as u64));

    for k in k_values {
        // Benchmark StdHashKgram
        group.bench_with_input(BenchmarkId::new("StdHashKgram", k), &data, |b, data| {
            b.iter(|| {
                let kgram = StdHashKgram;
                black_box(kgram.k_gram(black_box(data), black_box(k)))
            });
        });

        // Benchmark RollingHashKgram with wrapping arithmetic
        group.bench_with_input(
            BenchmarkId::new("RollingHashKgramWrapping", k),
            &data,
            |b, data| {
                b.iter(|| {
                    let kgram: RollingHashKgram<257, { u64::MAX }> = RollingHashKgram;
                    black_box(kgram.k_gram(black_box(data), black_box(k)))
                });
            },
        );

        // Benchmark RollingHashKgram with Mersenne Primes
        const MERSENNE_PRIME_61: u64 = (1u64 << 61) - 1;
        group.bench_with_input(
            BenchmarkId::new("RollingHashKgramMersenne", k),
            &data,
            |b, data| {
                b.iter(|| {
                    let kgram: RollingHashKgram<257, MERSENNE_PRIME_61> = RollingHashKgram;
                    black_box(kgram.k_gram(black_box(data), black_box(k)))
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_kgram_throughput, bench_kgram_varying_k);
criterion_main!(benches);
