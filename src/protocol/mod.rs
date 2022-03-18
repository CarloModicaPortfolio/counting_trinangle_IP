use ark_bls12_381::{Fq as F, FqParameters};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};

pub mod prover;
pub mod verifier;
use ark_poly::univariate::DensePolynomial;

#[derive(Debug)]
pub(crate) struct ProtocolState {
    pub C: F,
    pub random_values: Vec<F>,
    pub g_vector: Vec<DensePolynomial<F>>,
}

impl Initialize for ProtocolState {
    fn initialize (self,array_ext: &DenseMultilinearExtension<F>){
        let c_vect = &array_ext.to_evaluations();
        let self: C = c_vect.iter().sum();
        let self: random_values = vec![];
        let self: g_vectot = vec![];
    }
}
