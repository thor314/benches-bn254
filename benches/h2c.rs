#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// bench docs: https://bheisler.github.io/criterion.rs/book/index.html

use benches_bn254::{h2c_g1_add, h2c_g1_scalar_mul, h2c_pairing, h2c_scalar_mul};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use halo2curves::{
    bn256::{Fr, G1, G2},
    ff::Field,
    group::Group,
};
use rand::{thread_rng, Rng};

fn native_mul(a: u64, b: u64) -> u64 { a * b }

pub fn h2c_bn256_operations(c: &mut Criterion) {
    let mut rng = thread_rng();

    // Generate random points and scalars
    let p1 = G1::random(&mut rng);
    let p2 = G1::random(&mut rng);
    let q = G2::random(&mut rng);
    let s = Fr::random(&mut rng);
    let m = Fr::random(&mut rng);
    let n = Fr::random(&mut rng);
    let x = rng.gen::<u64>();
    let y = rng.gen::<u64>();

    let mut group = c.benchmark_group("halo2curves BN256");

    group.bench_function("G1 point addition", |b| b.iter(|| black_box(h2c_g1_add(p1, p2))));

    group.bench_function("G1 scalar multiplication", |b| {
        b.iter(|| black_box(h2c_g1_scalar_mul(p1, s)))
    });

    group.bench_function("Pairing", |b| b.iter(|| black_box(h2c_pairing(p1, q))));

    group.bench_function("Fr scalar field multiplication", |b| {
        b.iter(|| black_box(h2c_scalar_mul(m, n)))
    });

    group.bench_function("native mul", |b| b.iter(|| black_box(native_mul(x, y))));

    group.finish();
}

criterion_group!(benches, h2c_bn256_operations);
criterion_main!(benches);
