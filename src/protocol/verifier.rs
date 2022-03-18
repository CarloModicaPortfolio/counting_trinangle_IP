use ark_poly::univariate::DensePolynomial;
use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_std::rand::random;
use crate::{ProtocolState, DIM};

pub fn verifier(mut state: ProtocolState, array_ext: &DenseMultilinearExtension<F>,) -> ProtocolState {
    let mut rng = ark_std::rand::thread_rng();
    let r_2 = F::rand(&mut rng);
    state.random_values.push(r_2);

    let g_len = state.g_vector.len();

    if g_len == 0{
        println!("first round of verifier called before prover");
    } else if g_len == 1 {
        assert_eq!(state.C, state.g_vector[0].evaluate(&F::zero()) +  state.g_vector[0].evaluate(&F::one()));
        let r_2 = F::rand(&mut rng);
    } else if g_len == DIM {
        //single oravle query for the verifier
        let r4 = F::rand(&mut rng);
        let random_vector = state.random_values.clone();
        let oracle_query:F =  array_ext.evaluate(&[random_vector[0], random_vector[1], random_vector[2], r4]).unwrap();
        let g_last_check:F = state.g_vector[DIM-1].evaluate(&r4);
        assert!(oracle_query == g_last_check);
        println!("Protocol run successfully");

    } else {
        let g_1 = state.g_vector[g_len-2].clone();
        let g_2 = state.g_vector[g_len-1].clone();
        let r_1 = state.random_values[g_len-2].clone();
        assert_eq!(g_1.evaluate(&r_1), g_2.evaluate(&F::zero()) +  g_2.evaluate(&F::one()));
    }
    // GENERATE A RANDOM NUMBER TO SUPPLY THE PROVER
    return state
}
