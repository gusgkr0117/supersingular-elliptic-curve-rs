//! Implementation of quadratic extension field
use num::BigUint;
use num::bigint::ToBigInt;
use num_prime::buffer::{NaiveBuffer, PrimeBufferExt};
use super::fp::{FiniteField, FiniteFieldElement};
use crate::field::FieldElement;

/// Type for a base of a quadratic field
/// Fp[\alpha]
#[derive(Debug, Clone)]
pub struct FiniteField2 {
    base_field : FiniteField,
    alpha : BigUint,
}

impl FiniteField2 {
    /// The input [BigUint](num::BigUint) must be prime
    /// alpha must be non-quadratic residue
    pub fn new(base_field : &FiniteField, alpha : &BigUint) -> Self {
        let fp_alpha = base_field.gen(&alpha.to_bigint().unwrap());
        assert!(fp_alpha.sqrt() == None, "The alpha is quadratic residue!");
        FiniteField2 {
            base_field : base_field.clone(),
            alpha : alpha.clone(),
        }
    }
}