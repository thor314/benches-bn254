// repo: https://github.com/privacy-scaling-explorations/halo2curves
// docs: https://docs.rs/crate/halo2curves/latest

use halo2curves::{
    bn256::{Bn256, Fr, G1Affine, G2Affine, Gt, G1, G2},
    pairing::Engine,
};

pub fn h2c_g1_add(p1: G1, p2: G1) -> G1 { p1 + p2 }

pub fn h2c_g1_scalar_mul(p: G1, s: Fr) -> G1 { p * s }

// ref:
// https://github.com/privacy-scaling-explorations/halo2curves/blob/8771fe5a5d54fc03e74dbc8915db5dad3ab46a83/benches/pairing.rs
pub fn h2c_pairing(p: G1, q: G2) -> Gt {
    let p_affine = G1Affine::from(p);
    let q_affine = G2Affine::from(q);
    Bn256::pairing(&p_affine, &q_affine)
}

#[cfg(test)]
mod tests {
    use halo2curves::{ff::Field, group::Group};
    use rand::thread_rng;

    use super::*;

    #[test]
    fn test_operations() {
        let mut rng = thread_rng();
        let p1 = G1::random(&mut rng);
        let p2 = G1::random(&mut rng);
        let q = G2::random(&mut rng);
        let s = Fr::random(&mut rng);

        let _ = h2c_g1_add(p1, p2);
        let _ = h2c_g1_scalar_mul(p1, s);
        let _ = h2c_pairing(p1, q);
    }
}
