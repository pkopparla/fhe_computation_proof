use crate::{FILL_VALUE, INT_BITS, POLY_SIZE};
use csv;
use std::fs::File;
use sunscreen::types::bfv::Fractional;

fn read_csv(infile: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut out_vec = Vec::new();
    let file = File::open(infile)?;
    // let mut rdr = csv::Reader::from_reader(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: f64 = result?;
        out_vec.push(record);
    }
    out_vec.reverse();
    while POLY_SIZE - out_vec.len() > 0 {
        //ideally want 0 here but multiplying ciphers by 0 causes runtime errors
        out_vec.push(FILL_VALUE);
    }
    Ok(out_vec)
}

pub fn get_coefs(
    infile: &str,
) -> Result<[Fractional<INT_BITS>; POLY_SIZE], Box<dyn std::error::Error>> {
    // Returns an array of size POLY_SIZE containing polynomial coefficients
    // read in from a CSV file as type Fractional
    let raw_coefs = read_csv(infile)?;
    let coefs_vec: Vec<Fractional<INT_BITS>> = raw_coefs
        .iter()
        .map(|&x| Fractional::<INT_BITS>::from(x))
        .collect();
    let coefs: [Fractional<INT_BITS>; POLY_SIZE] = coefs_vec.try_into().unwrap();
    Ok(coefs)
}

pub fn get_powers(
    base: f64,
) -> Result<[Fractional<INT_BITS>; POLY_SIZE], Box<dyn std::error::Error>> {
    let mut out_vec = Vec::new();
    for i in 0..POLY_SIZE {
        out_vec.push(base.powi(i.try_into().unwrap()));
    }
    let pow_vec: Vec<Fractional<INT_BITS>> = out_vec
        .iter()
        .map(|&x| Fractional::<INT_BITS>::from(x))
        .collect();
    let powers: [Fractional<INT_BITS>; POLY_SIZE] = pow_vec.try_into().unwrap();
    Ok(powers)
}
