use ark_bn254::{Bn254, Fr, G1Projective as G1, G2Projective as G2};
use ark_ec::pairing::{Pairing, PairingOutput};

pub fn ark_g1_add(p1: G1, p2: G1) -> G1 { p1 + p2 }

pub fn ark_g1_scalar_mul(p: G1, s: Fr) -> G1 { p * s }

pub fn ark_scalar_mul(m: Fr, n: Fr) -> Fr { m * n }

pub fn ark_pairing(p1: G1, p2: G2) -> PairingOutput<Bn254> { Bn254::pairing(p1, p2) }
