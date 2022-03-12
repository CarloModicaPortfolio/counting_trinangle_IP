use ark_ff::{Field, PrimeField, FpParameters, BigInteger};
use ark_bls12_381::Fq as F;
use ark_std::{One, Zero, UniformRand};
use ark_poly::domain::EvaluationDomain;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, Polynomial};
use ark_poly::evaluations::univariate::Evaluations;
use ark_poly::GeneralEvaluationDomain;

fn prover(){

}

fn verifier(){

}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main(){
    // define the Adjacency Matrix A for the graph. The entries must be either 0 or 1
    // we use the. We flatten the matrix in a vector.
    let count = 0;
    let array = vec![F::one(), F::one(), F::one(), F::zero(), F::one(),
                                    F::zero(), F::one(), F::one(), F::zero(), F::one(), F::zero(),
                                    F::one(), F::one(), F::zero(), F::one(), F::one()];
    let array_len = array.len();

    //
    //we have calculated the multilinear extension of the array that wee need.
    let array_ext: DenseMultilinearExtension<F> = DenseMultilinearExtension{evaluations: array, num_vars: 4};

    /*
    i still have to figure out a way to multiply the polinomial by itself 3 times
    EvaluationDomain::mul_polynomials_in_evaluation_domain(&array, &[array], &[array]);
    could be a way of doing so. I will continue the protocol pretending that array_ext is
    already multiplied by itself 3 times.
    */

    //PROVER FIRST ROUND

    // Compute sum C
    let c_vect = array_ext.to_evaluations();
    let mut C = F::zero();
    let mut g_1_0 =  F::zero();
    let mut g_1_1  = F::zero();

    for i in 0..array_len{
        C = C + c_vect[i];
        if i < 8{
            g_1_0 = g_1_0 + c_vect[i];
        } else {
            g_1_1 = g_1_1 + c_vect[i];
        }
    }

    println!("{}", C);
    println!("{}", g_1_1+g_1_0);


    // COMPUTE G_1

    let g_1 : Evaluations<F,GeneralEvaluationDomain<F>> = Evaluations::from_vec_and_domain(vec![g_1_0, g_1_1], EvaluationDomain::new(2).unwrap());
    let g_1_poly = g_1.interpolate();
    println!("{:?}", g_1_poly.evaluate(&F::zero())+g_1_poly.evaluate(&F::one()));


    /*
    let mut rng = ark_std::rand::thread_rng();
    // Let's sample uniformly random field elements:
    let a = F::rand(&mut rng);
    let b = F::rand(&mut rng);
    println!("a is: {}", a);
    // We can add...
    //nlet c = a + A[9];
    // ... subtract ...
    let d = a - b;
    println!("d is: {}", d);
    */
}