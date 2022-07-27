use std::usize;
use sunscreen::{
    fhe_program,
    types::{bfv::Fractional, Cipher},
    Error,
};

mod polynomials;
mod roles;
mod utils;

const INT_BITS: usize = 1024;
const POLY_SIZE: usize = 12;
const FILL_VALUE: f64 = 0.0;

fn main() -> Result<(), Error> {
    // Researcher is the prover
    // Setup creates the compiler and reads in two polynomials
    // p(x) = l(x)*r(x)-o(x)
    // h(x) = p(x)/t(x)
    let researcher = roles::Researcher::setup()?;
    // Referee is the verifier
    // Setup creates public and private keys
    let referee = roles::Referee::setup(&researcher.compiled_poly_evaluate.metadata.params)?;
    // Query creates encrypted powers of the base upto POLY_SIZE to be sent to the researcher
    // and evaluates the zero polynomial at base in plaintext
    let query = referee.create_query(1.5)?;
    // Researcher calculates the values of the computational and quotient polynomial
    // using FHE
    let response = researcher.run_query(query, &referee.public_key)?;
    // Referee checks that comp_poly(base) ~= quot_poly(base)*zero_poly(base)
    referee.check_response(response)?;

    Ok(())
}
