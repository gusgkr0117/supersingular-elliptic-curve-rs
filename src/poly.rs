//! Polynomial with field coefficient
use crate::field::{Field, FieldElement, DynZero};
use std::ops::{Add, Sub, Mul, Neg, Rem};
use std::cmp::max;
use std::fmt;

/// Polynomial refers to a [Field](crate::fp::Field) for [FieldElement](crate::fp::FieldElement)
/// It takes the lifetime of the [Field](crate::fp::Field)
#[derive(Clone)]
pub struct Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    field : &'a F, 
    coefficient : Vec<E>,
}

impl<'a, F, E> fmt::Debug for Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (1..self.coefficient.len()).rev() {
            write!(f, "{:?}x^{:?} + ", self.coefficient[i], i)?;
        }
        
        if self.degree() == 0 {
            write!(f, "0")?;
        }else {
            write!(f, "{:?}", self.coefficient[0])?;
        }

        Ok(())
    }
}


impl<'a, F, E> Polynomial<'a, F, E> where &'a F : Field<'a, E>, E: FieldElement<'a>, F: Clone {
    pub fn new(field : &'a F, coefficient : Vec<E>) -> Self {
        Polynomial {
            field,
            coefficient,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.coefficient.len() == 0
    }

    pub fn degree(&self) -> usize {
        match self.is_zero() {
            true => 0,
            false => self.coefficient.len() - 1,
        }
    }

    pub fn normalize(&mut self) {
        if Some(&self.field.zero()) == self.coefficient.last() {
            let len = self.coefficient.iter().rposition(|d| d.clone() != self.field.zero()).map_or(0, |i| i + 1);
            self.coefficient.truncate(len);
        }

        if self.coefficient.len() < self.coefficient.capacity() / 4 {
            self.coefficient.shrink_to_fit();
        }
    }

}

impl<'a, F, E> Add for Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    type Output = Polynomial<'a, F, E>;
    fn add(self, rhs:Self) -> Self::Output {
        if self.is_zero() {
            return rhs;
        }

        if rhs.is_zero() {
            return self;
        }

        let mut result_coeff = vec![self.field.zero(); max(self.degree(), rhs.degree()) + 1];

        for i in 0..result_coeff.len() {
            if i < self.degree() { result_coeff[i] = result_coeff[i].clone() + self.coefficient[i].clone();}
            if i < rhs.degree() { result_coeff[i] = result_coeff[i].clone() + rhs.coefficient[i].clone();}
        }

        let mut result = Polynomial {
            field : self.field,
            coefficient : result_coeff
        };
        result.normalize();

        result
    }
}

impl<'a, F, E> Neg for Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Polynomial {
            field : self.field,
            coefficient : self.coefficient.iter().map(|x| -x.clone()).collect(),
        }
    }
}

impl<'a, F, E> Sub for Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    type Output = Polynomial<'a, F, E>;
    fn sub(self, rhs:Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<'a, F, E> Mul for Polynomial<'a, F, E> where &'a F: Field<'a, E>, E: FieldElement<'a>, F: Clone {
    type Output = Polynomial<'a, F, E>;
    fn mul(self, rhs:Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Polynomial{field:self.field, coefficient:vec![]}
        }

        let mut result_coeff = vec![self.field.zero(); self.degree() + rhs.degree() + 1];

        for i in 0..self.degree() + 1 {
            for j in 0..rhs.degree() + 1 {
                result_coeff[i + j] = result_coeff[i + j].clone() + self.coefficient[i].clone() * rhs.coefficient[j].clone();
            }
        }

        Polynomial {
            field : self.field,
            coefficient : result_coeff
        }
    }
}

impl<'a, F, E> Rem for Polynomial<'a, F, E> where &'a F: Field<'a, E> + Clone, E: FieldElement<'a>, F: Clone {
    type Output = Polynomial<'a, F, E>;
    fn rem(self, rhs: Self) -> Self::Output {
        assert!(!rhs.is_zero(), "Can't devide by zero");
        let mut result = self;

        while rhs.degree() <= result.degree() {
            if result.is_zero() {
                return result;
            }

            let rhs_lc = rhs.coefficient.last().unwrap().clone();
            let result_lc = result.coefficient.last().unwrap().clone();

            let mut tmp = Polynomial::new(result.field, vec![result.field.zero(); result.degree() - rhs.degree()]);
            tmp.coefficient.push(result_lc * rhs_lc.inv());

            result = result.clone() - rhs.clone() * tmp;
        }

        result
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::field::fp::{FiniteField};
    use num::{BigInt, BigUint};

    #[test]
    fn polynomial_test() {
        // Fp11
        let fp = FiniteField::new(&BigUint::from(11 as u32));
        let coeff1 = fp.gen(&BigInt::from(2));
        let coeff2 = fp.gen(&BigInt::from(-7));
        let coeff3 = fp.gen(&BigInt::from(4));
        let poly1 = Polynomial::new(&fp, vec![coeff1.clone(), coeff2.clone(), coeff3.clone()]);
        let poly2 = Polynomial::new(&fp, vec![coeff3, coeff2, coeff1]);
        

        let poly3 = poly1.clone() * poly1.clone();
        println!("({:?}) % ({:?}) = {:?}", poly3, poly1, poly3.clone() % poly1.clone());
        println!("({:?}) % ({:?}) = {:?}", poly3, poly2, poly3.clone() % poly2.clone());
    }
}