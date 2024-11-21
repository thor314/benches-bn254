#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// bench docs: https://bheisler.github.io/criterion.rs/book/index.html

use ark_bn254::{Fr as Scalar, G1Projective as G1};
use ark_ff::UniformRand;
use benches_bn254::{ark_g1_add, ark_g1_scalar_mul};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;

pub fn bn254_operations(c: &mut Criterion) {
    let mut rng = thread_rng();

    let p1 = G1::rand(&mut rng);
    let p2 = G1::rand(&mut rng);
    let s = Scalar::rand(&mut rng);

    let mut group = c.benchmark_group("BN254");

    group.bench_function("G1 point addition", |b| b.iter(|| black_box(ark_g1_add(p1, p2))));

    group.bench_function("G1 scalar multiplication", |b| {
        b.iter(|| black_box(ark_g1_scalar_mul(p1, s)))
    });

    group.finish();
}

criterion_group!(benches, bn254_operations);
criterion_main!(benches);
