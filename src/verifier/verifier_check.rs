use ark_poly::univariate::DensePolynomial;
use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};

pub fn verifier(g_1: DensePolynomial<Fq>, g_2: DensePolynomial<Fq>, r_1: F) -> Fp384<FqParameters> {

    //Check that the prover claim is correct
    let g_check:F = g_2.evaluate(&F::zero()) + g_2.evaluate(&F::one());
    assert!(g_check == g_1.evaluate(&r_1));

    // GENERATE A RANDOM NUMBER TO SUPPLY THE PROVER
    let mut rng = ark_std::rand::thread_rng();
    let r_2 = F::rand(&mut rng);
    return r_2
}
