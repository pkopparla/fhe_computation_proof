use std::cell::Cell;
use crate::{
    encrypted_poly_evaluate, polynomials,
    utils::{get_coefs, get_powers},
    INT_BITS, POLY_SIZE,
};
use sunscreen::{
    types::bfv::Fractional, Ciphertext, CompiledFheProgram, 
    Compiler, Error, FheProgramInput,
    Params, PrivateKey, PublicKey, Runtime,
};

pub struct Researcher {
    pub compiled_poly_evaluate: CompiledFheProgram,
    runtime: Runtime,
    computation_polynomial: [Fractional<INT_BITS>; POLY_SIZE],
    quotient_polynomial: [Fractional<INT_BITS>; POLY_SIZE],
}
impl Researcher {
    pub fn setup() -> Result<Researcher, Error> {
        let app = Compiler::new()
            .fhe_program(encrypted_poly_evaluate)
            .compile()?;

        let runtime = Runtime::new(app.params())?;

        let computation_polynomial: [Fractional<INT_BITS>; POLY_SIZE] = get_coefs("../P.csv").unwrap();
        let quotient_polynomial: [Fractional<INT_BITS>; POLY_SIZE] = get_coefs("../H.csv").unwrap();
        Ok(Researcher {
            compiled_poly_evaluate: app.get_program(encrypted_poly_evaluate).unwrap().clone(),
            runtime,
            computation_polynomial,
            quotient_polynomial,
        })
    }

    pub fn run_query(
        &self,
        query: Ciphertext,
        public_key: &PublicKey,
    ) -> Result<(Ciphertext, Ciphertext), Error> {
        // Our database will consist of values between 400 and 500.
        let poly_coefs = self.runtime.encrypt(self.computation_polynomial, &public_key)?;
        let quot_coefs = self.runtime.encrypt(self.quotient_polynomial, &public_key)?;

        let poly_args: Vec<FheProgramInput> = vec![query.clone().into(), poly_coefs.into()];
        let quot_args: Vec<FheProgramInput> = vec![query.clone().into(), quot_coefs.into()];

        let poly_results =
            self.runtime
                .run(&&self.compiled_poly_evaluate, poly_args, public_key)?;
        let quot_results =
            self.runtime
                .run(&&self.compiled_poly_evaluate, quot_args, public_key)?;
        Ok((poly_results[0].clone(), quot_results[0].clone()))
    }
}

// Verifier
pub struct Referee {
    pub public_key: PublicKey,
    private_key: PrivateKey,
    runtime: Runtime,
    t_val: Cell<f64>,
}

impl Referee {
    pub fn setup(params: &Params) -> Result<Referee, Error> {
        let runtime = Runtime::new(params)?;

        let (public_key, private_key) = runtime.generate_keys()?;

        let t_val = Cell::new(0.0);

        Ok(Referee {
            public_key,
            private_key,
            runtime,
            t_val,
        })
    }

    pub fn create_query(&self, base: f64) -> Result<Ciphertext, Error> {
        let raw_powers = get_powers(base).unwrap();
        let raw_coefs = get_coefs("../T.csv").unwrap();
        let t_frac: f64 = polynomials::poly_evaluate(&raw_powers, &raw_coefs).into();
        self.t_val.set(t_frac);
        Ok(self.runtime.encrypt(raw_powers, &self.public_key)?)
    }

    pub fn check_response(&self, value: (Ciphertext, Ciphertext)) -> Result<(), Error> {
        let p_frac: Fractional<INT_BITS> = self.runtime.decrypt(&value.0, &self.private_key)?;
        let h_frac: Fractional<INT_BITS> = self.runtime.decrypt(&value.1, &self.private_key)?;

        let p_val: f64 = p_frac.into();
        let h_val: f64 = h_frac.into();

        println!("Referee received {} {}", p_val, self.t_val.get() * h_val);

        Ok(())
    }
}
