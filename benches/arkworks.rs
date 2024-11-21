#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ark_bn254::{Fr as Scalar, G1Projective as G1, G2Projective as G2};
use ark_ff::UniformRand;
use benches_bn254::{ark_g1_add, ark_g1_scalar_mul, ark_pairing, ark_multiple_g1_adds};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;

pub fn bn254_operations(c: &mut Criterion) {
    let mut rng = thread_rng();

    let p1 = G1::rand(&mut rng);
    let p2 = G1::rand(&mut rng);
    let q1 = G2::rand(&mut rng);
    let s = Scalar::rand(&mut rng);

    let mut group = c.benchmark_group("BN254");

    group.bench_function("G1 point addition", |b| {
        b.iter(|| black_box(ark_g1_add(p1, p2)))
    });

    group.bench_function("G1 scalar multiplication", |b| {
        b.iter(|| black_box(ark_g1_scalar_mul(p1, s)))
    });

    group.bench_function("Pairing", |b| {
        b.iter(|| black_box(ark_pairing(p1, q1)))
    });

    group.bench_function("1000 G1 additions", |b| {
        b.iter(|| black_box(ark_multiple_g1_adds(p1, p2, 1000)))
    });

    group.finish();
}

criterion_group!(benches, bn254_operations);
criterion_main!(benches);
