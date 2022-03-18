use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::univariate::DensePolynomial;

mod protocol;

use protocol::{prover::prover,verifier::verifier};
use protocol::{ProtocolState, initialize};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub(crate) const DIM: usize = 4;

fn main() {
    //PROBLEM DEFINITION
//==================================================================================================
    // define the Adjacency Matrix A for the graph. The entries must be either 0 or 1
    // we use the. We flatten the matrix in a vector.
    let array = vec![F::one(), F::one(), F::one(), F::zero(),
                     F::one(), F::zero(), F::one(), F::one(),
                     F::zero(), F::one(), F::zero(), F::one(),
                     F::one(), F::zero(), F::one(), F::one()];

    //PROVER FIRST ROUND
//==================================================================================================
    // inizialize multiliniar extension of the function over the boolean hypercube
    let array_ext: DenseMultilinearExtension<F> = DenseMultilinearExtension { evaluations: array, num_vars: 4 };
    /*
    i still have to figure out a way to multiply the polinomial by itself 3 times
    EvaluationDomain::mul_polynomials_in_evaluation_domain(&array, &[array], &[array]);
    could be a way of doing so. I will continue the protocol pretending that array_ext is
    already multiplied by itself 3 times.
    */
    let mut state = initialize(&array_ext);


    //PROTOCOL
//==================================================================================================

    // Vector where the verifier will store its random values.
    let mut rng = ark_std::rand::thread_rng();

    for i in 0..DIM {
        state = prover(&array_ext, state);
        state = verifier(state, &array_ext);
    }

}