use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::univariate::DensePolynomial;

mod prover;
mod verifier;

use prover::proof_generation::prover;
use verifier::verifier_check::verifier;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

    let mut v_random_vector: Vec<F> = vec![];
    let mut g_vector: Vec<DensePolynomial<F>> = vec![];

    // inizialize multiliniar extension of the function over the boolean hypercube
    let array_ext: DenseMultilinearExtension<F> = DenseMultilinearExtension { evaluations: array, num_vars: 4 };
    /*
    i still have to figure out a way to multiply the polinomial by itself 3 times
    EvaluationDomain::mul_polynomials_in_evaluation_domain(&array, &[array], &[array]);
    could be a way of doing so. I will continue the protocol pretending that array_ext is
    already multiplied by itself 3 times.
    */

    // Compute sum C
    let c_vect = array_ext.to_evaluations();
    let C: F = c_vect.iter().sum();
    // compute g_1(0) and g_1(1)
    let mut c_left = F::zero();
    let mut c_right = F::zero();
    let c_comp = c_vect.clone();
    for i in 0..8 {
        c_left = c_left + c_comp[i];
        c_right = c_right + c_comp[15 - i];
    }

    // Using g_1(0) and g_1(1) we can compute g_1(X_1), since we know it is linear
    let g_1: DensePolynomial<F> = DensePolynomial { coeffs: vec![c_left, c_right - c_left] };
    assert!(g_1.evaluate(&F::zero()) + g_1.evaluate(&F::one()) == C);
    let mut rng = ark_std::rand::thread_rng();
    let r1 = F::rand(&mut rng);


    //REST OF THE PROTOCOL
//==================================================================================================

    // Vector where the verifier will store its random values.

    g_vector.push(g_1);
    v_random_vector.push(r1);

    println!("round 0");
    for i in 0..3 {
        g_vector.push(prover(array_ext.clone(), v_random_vector.clone()));
        v_random_vector.push(verifier(g_vector[i].clone(), g_vector[i + 1].clone(), v_random_vector[i].clone()));
        println!("+1 round");
    }

    //VERIFIER LAST ROUDND
//==================================================================================================

    //single oravle query for the verifier
    let r4 = F::rand(&mut rng);
    let oracle_query:F =  array_ext.evaluate(&[v_random_vector[0], v_random_vector[1], v_random_vector[2], r4]).unwrap();
    println!("{}", oracle_query);
    let g_last_check:F = g_vector[3].evaluate(&r4);
    println!("{}", g_last_check);
    assert!(oracle_query == g_last_check);

}