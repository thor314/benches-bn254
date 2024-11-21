// repo: https://github.com/privacy-scaling-explorations/halo2curves
// docs: https://docs.rs/crate/halo2curves/latest

use halo2curves::bn256::{Fr, G1};

pub fn h2c_g1_add(p1: G1, p2: G1) -> G1 { p1 + p2 }

pub fn h2c_g1_scalar_mul(p: G1, s: Fr) -> G1 { p * s }

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
        let s = Fr::random(&mut rng);

        let _ = h2c_g1_add(p1, p2);
        let _ = h2c_g1_scalar_mul(p1, s);
    }
}
