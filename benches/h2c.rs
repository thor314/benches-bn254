#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// bench docs: https://bheisler.github.io/criterion.rs/book/index.html

use benches_bn254::{h2c_g1_add, h2c_g1_scalar_mul};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use halo2curves::{
    bn256::{Fr, G1},
    ff::Field,
    group::Group,
};
use rand::thread_rng;

pub fn bn254_operations(c: &mut Criterion) {
    let mut rng = thread_rng();

    // Generate random points and scalars
    let p1 = G1::random(&mut rng);
    let p2 = G1::random(&mut rng);
    let s = Fr::random(&mut rng);

    let mut group = c.benchmark_group("BN254");

    group.bench_function("G1 point addition", |b| b.iter(|| black_box(h2c_g1_add(p1, p2))));

    group.bench_function("G1 scalar multiplication", |b| {
        b.iter(|| black_box(h2c_g1_scalar_mul(p1, s)))
    });

    group.finish();
}

criterion_group!(benches, bn254_operations);
criterion_main!(benches);
