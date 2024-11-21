use ark_bn254::{Fr as Scalar, G1Projective as G1};

pub fn ark_g1_add(p1: G1, p2: G1) -> G1 { p1 + p2 }

pub fn ark_g1_scalar_mul(p: G1, s: Scalar) -> G1 { p * s }
