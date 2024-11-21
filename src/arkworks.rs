use ark_bn254::{Bn254, Fr as Scalar, G1Projective as G1, G2Projective as G2};
use ark_ec::pairing::{Pairing, PairingOutput};

pub fn ark_g1_add(p1: G1, p2: G1) -> G1 { 
    p1 + p2 
}

pub fn ark_g1_scalar_mul(p: G1, s: Scalar) -> G1 { 
    p * s 
}

pub fn ark_pairing(p1: G1, p2: G2) -> PairingOutput<Bn254> {
    Bn254::pairing(p1, p2)
}

pub fn ark_multiple_g1_adds(p1: G1, p2: G1, iterations: usize) -> G1 {
    let mut result = p1;
    for _ in 0..iterations {
        result = result + p2;
    }
    result
}
