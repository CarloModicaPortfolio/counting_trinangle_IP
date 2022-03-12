extern crate core;
use ark_ff::{Field, PrimeField, FpParameters, BigInteger, Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::domain::EvaluationDomain;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::evaluations::univariate::Evaluations;
use ark_poly::GeneralEvaluationDomain;
use ark_poly::univariate::DensePolynomial;

use ark_r1cs_std::poly::domain::Radix2DomainVar;

fn prover(mut array_ext: DenseMultilinearExtension<F>, mut random_values: Vec<F>) -> DensePolynomial<Fq> {
    let iteration = random_values.len();
    while random_values.len() < 4{
        random_values.push(F::zero());
    }

    let mut c_left = F::zero();
    let mut c_right = F::zero();

    let mut random_values_0 = random_values.clone();
    random_values_0[iteration] = F::zero();
    let mut random_values_1 = random_values.clone();
    random_values_1[iteration] = F::one();



    for i in 0..i32::pow((4 - (iteration + 1)) as i32 , 2){
        let mut n = i.clone();
        let mut binary_number:Vec<F> = vec![];
        while n > 0 {
            if n % 2 == 0 {
                binary_number.push(F::zero());
            }else{
                binary_number.push(F::one());
            }
            n = n / 2;
        }
        while (binary_number.len()) < (4 - (iteration + 1)){
            binary_number.push(F::zero());
        }
        binary_number.reverse();

        let modif_bits = binary_number.len();
        let rand_len = random_values_0.len();

        // first value past verifier random value fixed at 0
        for j in 0..modif_bits{
            random_values_0[rand_len-j-1] = binary_number[modif_bits-j-1];
        }
        c_left = c_left + array_ext.evaluate(&random_values_0).unwrap();

        // first value past verifier random value fixed at 1
        for j in 0..modif_bits{
            random_values_1[rand_len-j-1] = binary_number[modif_bits-j-1];
        }

        // first value past verifier random value fixed at 1
        c_right = c_right + array_ext.evaluate(&random_values_1).unwrap();

    }
    if iteration == 3{}
    let g_i: DensePolynomial<F> = DensePolynomial{ coeffs: vec![c_left, c_right - c_left]};
    println!("Evaluation of g_i(1)+g_i(0) @ prover: {}", g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one()));
    return(g_i)
}

fn verifier(g_i: DensePolynomial<Fq>, C: F) -> Fp384<FqParameters> {

    //Check that the prover claim is correct
    let g_check:F = g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one());
    println!("g_check @ verifier {}", g_check);
    assert!(g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one()) == C);

    // GENERATE A RANDOM NUMBER TO SUPPLY THE PROVER
    let mut rng = ark_std::rand::thread_rng();
    let a = F::rand(&mut rng);
    return (a);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main(){
    // define the Adjacency Matrix A for the graph. The entries must be either 0 or 1
    // we use the. We flatten the matrix in a vector.
    let array = vec![F::one(), F::one(), F::one(), F::zero(),
                                            F::one(),F::zero(), F::one(), F::one(),
                                            F::zero(), F::one(), F::zero(), F::one(),
                                            F::one(), F::zero(), F::one(), F::one()];

    //PROVER FIRST ROUND
//==================================================================================================

    // inizialize multiliniar extension of the function over the boolean hypercube
    let array_ext: DenseMultilinearExtension<F> = DenseMultilinearExtension{evaluations: array, num_vars: 4};
    /*
    i still have to figure out a way to multiply the polinomial by itself 3 times
    EvaluationDomain::mul_polynomials_in_evaluation_domain(&array, &[array], &[array]);
    could be a way of doing so. I will continue the protocol pretending that array_ext is
    already multiplied by itself 3 times.
    */
    // Compute sum C
    let c_vect = array_ext.to_evaluations();
    let mut C: F = c_vect.iter().sum();
    // compute g_1(0) and g_1(1)
    let mut c_left =  F::zero();
    let mut c_right  = F::zero();
    let mut c_comp = c_vect.clone();
    for i in 0..8{
        c_left = c_left + c_comp[i];
        c_right = c_right + c_comp[15-i];
    }

    // Using g_1(0) and g_1(1) we can compute g_1(X_1), since we know it is linear
    let mut g_i: DensePolynomial<F> = DensePolynomial{ coeffs: vec![c_left, c_right - c_left]};

    //REST OF THE PROTOCOL
//==================================================================================================

    // Vector where the verifier will store its random values.
    let mut v_random_vector:Vec<F> = vec![];

    for i in 0..4{
        let r_i = verifier(g_i.clone(), C);
        println!("+1 round");
        v_random_vector.push(r_i);
        C = g_i.evaluate(&r_i);
        println!("{}", C);
        let g_i = prover(array_ext.clone(), v_random_vector.clone());
        //println!("C: {}", C);
    }
    //VERIFIER LAST ROUDND
//==================================================================================================

}