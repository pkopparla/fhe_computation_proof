use std::{cell::Cell, usize};

// use assert_approx_eq::assert_approx_eq;

use sunscreen::{
    fhe_program,
    types::{bfv::Fractional, Cipher},
    // Ciphertext, CompiledFheProgram, Compiler, Error, FheProgramInput, Params, PrivateKey,
    // PublicKey, Runtime,
    Error,
};

mod polynomials;
mod roles;
mod utils;

const INT_BITS: usize = 1024;
const POLY_SIZE: usize = 12;
const FILL_VALUE: f64 = 0.0;

#[fhe_program(scheme = "bfv")]
fn encrypted_poly_evaluate<const N: usize>(
    powers: [Cipher<Fractional<INT_BITS>>; POLY_SIZE],
    coefs: [Cipher<Fractional<INT_BITS>>; POLY_SIZE],
) -> Cipher<Fractional<INT_BITS>> {
    polynomials::poly_evaluate(&powers, &coefs)
}

fn main() -> Result<(), Error> {
    // Researcher is the prover
    let researcher = roles::Researcher::setup()?;
    // Referee is the verifier
    let referee = roles::Referee::setup(&researcher.compiled_poly_evaluate.metadata.params)?;

    let query = referee.create_query(1.5)?;

    let response = researcher.run_query(query, &referee.public_key)?;

    referee.check_response(response)?;

    Ok(())
}

