//! Polynomial with field coefficient
use crate::fp::{Field, FieldElement, DynZero};
use std::ops::{Add, Sub, Mul, Neg};
use std::cmp::max;

/// Polynomial refers to a [Field](crate::fp::Field) for [FieldElement](crate::fp::FieldElement)
/// It takes the lifetime of the [Field](crate::fp::Field)
#[derive(Clone, Debug)]
pub struct Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a> {
    field : &'a F, 
    coefficient : Vec<E>,
}

impl<'a, F, E> Polynomial<'a, F, E> where &'a F : Field<'a, E>, E: FieldElement<'a> {
    pub fn new(field : &'a F, coefficient : Vec<E>) -> Self {
        Polynomial {
            field,
            coefficient,
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficient.len()
    }
}

impl<'a, F, E> Add for Polynomial<'a, F, E> where &'a F: Field<'a, E> , E: FieldElement<'a> {
    type Output = Polynomial<'a, F, E>;
    fn add(self, rhs:Self) -> Self::Output {
        let mut result_coeff = vec![self.field.zero(); max(self.degree(), rhs.degree())];

        for i in 0..result_coeff.len() {
            if i < self.degree() { result_coeff[i] = result_coeff[i].clone() + self.coefficient[i].clone();}
            if i < rhs.degree() { result_coeff[i] = result_coeff[i].clone() + rhs.coefficient[i].clone();}
        }

        Polynomial {
            field : self.field,
            coefficient : result_coeff
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::fp::{FiniteField};
    use num::{BigInt, BigUint};
    use num::bigint::Sign;

    #[test]
    fn polynomial_test() {
        // Fp11
        let fp = FiniteField::new(&BigUint::new(vec![11]));
        let coeff1 = fp.gen(&BigInt::new(Sign::Plus, vec![2]));
        let coeff2 = fp.gen(&BigInt::new(Sign::Minus, vec![7]));
        let coeff3 = fp.gen(&BigInt::new(Sign::Plus, vec![4]));
        let poly1 = Polynomial::new(&fp, vec![coeff1.clone(), coeff2.clone(), coeff3.clone()]);
        let poly2 = Polynomial::new(&fp, vec![coeff1, coeff2, coeff3]);

        let poly3 = poly1 + poly2;
        println!("{:?}", poly3);
    }
}