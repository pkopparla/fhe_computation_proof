# Proof of Computation with Real Numbers

Currently has two parts:
## 1. A jupyter notebook that sets up polynomials from a computation (`Clean_proof.ipynb`)

This bit has comments and describes the construction of a "quadratic arithmetic program"
for computing the standard deviation of a set of three values. The resulting polynomials
are then stored in csv files.
The polynomials are related as follows:

$ p(x) = l(x)*r(x) - o(x)$

$ h(x) = \frac{p(x)}{t(x)}$

## 2. A rust code that does a prover/verifier interaction for these polynomials (`fhe_rs`)

To run this just go into the directory and do `cargo run`. This has a few pieces:

a. `main.rs` runs the verifier-prover interaction to check the relationship 
$p(s) = t(s)*h(s)$ for two secret polynomials $p(x), h(x)$ and a shared polynomial $t(x)$ at some point $x=s$. The interesting bit here is that all coefficients are real numbers not integers or field elements.

b. `polynomials.rs` has a generic polynomial evaluation function and an fhe version of the same.

c. `utils.rs` has utitilies to read in coefficients from csv files and to generate powers of a given number.

d. `roles.rs` has structures for the prover and verifier with functions to generating and executing queries.

All code borrows heavily from examples in the sunscreen documentation.




