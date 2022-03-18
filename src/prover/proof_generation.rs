use ark_ff::{Fp384};
use ark_bls12_381::{Fq as F, Fq, FqParameters};
use ark_std::{One, Zero, UniformRand};
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::univariate::DensePolynomial;


//inputs: MLE of the function representing the adjacency  matrix A; Vector with the random values
// history from the verifier
//output: Polynomial g_i to give to the verifier
pub fn prover(array_ext: DenseMultilinearExtension<F>, mut random_values: Vec<F>) -> DensePolynomial<Fq> {

    //The iteration is given by the length of the random values vector
    let iteration = random_values.len();

    //Fill the random value vector with zero
    while random_values.len() < 4{
        random_values.push(F::zero());
    }

    //'c_left' and 'c_right' are going to be used to compute the coefficients of the new polynomial
    // g_i.
    let mut c_left = F::zero();
    let mut c_right = F::zero();

    let mut random_values_0 = random_values.clone();
    random_values_0[iteration] = F::zero();
    let mut random_values_1 = random_values.clone();
    random_values_1[iteration] = F::one();

    //
    let i_n = i32::pow(2, (4 - (iteration + 1)) as u32);


    for i in 0..i_n {
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

    //here we define the new polynomial
    let g_i: DensePolynomial<F> = DensePolynomial{ coeffs: vec![c_left, c_right - c_left]};
    return g_i
}