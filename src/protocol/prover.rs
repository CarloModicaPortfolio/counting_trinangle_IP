use std::panic::resume_unwind;
use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::univariate::DensePolynomial;
use crate::{DIM, print_type_of};


//inputs: MLE of the function representing the adjacency  matrix A; Vector with the random values
// history from the verifier
//output: Polynomial g_i to give to the verifier
pub fn prover(array_ext: &DenseMultilinearExtension<F>, mut random_values: Vec<F>) -> DensePolynomial<Fq> {

    //The iteration is given by the length of the random values vector
    let iteration = random_values.len();

    //Fill the random value vector with zero.
    //Example: [r_1,r_2] --> [r_1,r_2, 0, 0]
    while random_values.len() < DIM{
        random_values.push(F::zero());
    }

    //'c_left' and 'c_right' are going to be used to compute the coefficients of the new polynomial
    // g_i.
    //Following the previous example: c_left = [r_1,r_2, 0, 0] + [r_1,r_2,  0, 1]
    //                                c_right = [r_1,r_2, 1, 0] + [r_1,r_2, 1, 1]
    let mut c_left = F::zero();
    let mut c_right = F::zero();

    //k is the number of possible bistrings in the random vector after the random value form the verifier
    //and either the 0 or 1 from c_left or c_right are fixed. In the previous example k = 2
    let k = i32::pow(2, (4 - (iteration + 1)) as u32);

    // clone the random_value vector and create 2 vectors that have either 0 or 1 at the first
    // bit after the fixed random vector
    let mut random_values_left = random_values.clone();
    random_values_left[iteration] = F::zero();
    let mut random_values_right = random_values.clone();
    random_values_right[iteration] = F::one();

    for i in 0..k {
        //get binary rapresentation of i, and its lenght
        let binary_number = get_binary(i, iteration as usize);
        let binary_lenght = binary_number.len();

        // apply the binary rappresenation of i to at the end of the random values vector,
        // past the fixed random values from the verifier and the following first bit
        for j in 0..binary_lenght{
            random_values_right[DIM-1-j] = binary_number[binary_lenght-1-j].clone();
            random_values_left[DIM-1-j] = binary_number[binary_lenght-1-j].clone();
        }

        // update the values of c_right and c_left. We are executing the sum.
        c_right = c_right + &array_ext.evaluate(&random_values_right).unwrap();
        c_left = c_left + &array_ext.evaluate(&random_values_left).unwrap();
    }

    if iteration == 0{
        let c_vect = array_ext.to_evaluations();
        let C: F = c_vect.iter().sum();
        print_type_of(&C);
        println!("{:?}", assert_eq!(C, c_right+c_left));
        println!("Iteration 0 check passed");
    }

    //here we define the new polynomial
    let g_i: DensePolynomial<F> = DensePolynomial{ coeffs: vec![c_left, c_right - c_left]};
    return g_i
}

fn get_binary(mut n: i32,iteration: usize) -> Vec<Fq> {
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
    return binary_number
}

