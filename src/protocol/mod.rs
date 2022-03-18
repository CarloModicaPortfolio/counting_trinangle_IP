use ark_bls12_381::{Fq as F, FqParameters};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};

pub mod prover;
pub mod verifier;
use ark_poly::univariate::DensePolynomial;

#[derive(Debug)]
pub struct ProtocolState {
    pub C: F,
    pub random_values: Vec<F>,
    pub g_vector: Vec<DensePolynomial<F>>,
}

pub fn initialize(array_ext: &DenseMultilinearExtension<F>) ->(ProtocolState){
    let c_vect = &array_ext.to_evaluations();
    let C = c_vect.iter().sum();
    let random_values:Vec<F> = vec![];
    let g_vector: Vec<DensePolynomial<F>> = vec![];
    let state = ProtocolState{C: C, random_values: random_values, g_vector: g_vector};
    return state
}