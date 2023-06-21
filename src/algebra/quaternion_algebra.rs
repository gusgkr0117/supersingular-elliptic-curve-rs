use num::BigUint;
use num::bigint::ToBigInt;
use num_rational::BigRational;
use std::ops::{Add, Mul};
use num_prime::buffer::{NaiveBuffer, PrimeBufferExt};

/// B_p,\infty
#[derive(Clone, Debug)]
pub struct QuaternionAlgebra {
    prime : BigUint
}

impl<'a> QuaternionAlgebra {
    pub fn new(prime : &BigUint) -> Self {
        let pb = NaiveBuffer::new();
        assert!(pb.is_prime(prime, None).probably(), "The quaternion base number is not prime!");
        
        QuaternionAlgebra { prime: prime.clone() }
    }

    pub fn zero(&'a self) -> QuaternionAlgebraElement<'a> {
        QuaternionAlgebraElement { algebra: self, coefficient: [BigRational::default(), BigRational::default(), BigRational::default(), BigRational::default()] }
    }

    pub fn gen(&'a self, coefficient : [BigRational;4]) -> QuaternionAlgebraElement<'a> {
        QuaternionAlgebraElement {
            algebra : self,
            coefficient
        }
    }
}

/// A quaternion element consists of four rational coefficients
#[derive(Clone, Debug)]
pub struct QuaternionAlgebraElement<'a> {
    algebra : &'a QuaternionAlgebra,
    coefficient : [BigRational;4],
}

impl<'a> Add for QuaternionAlgebraElement<'a> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        QuaternionAlgebraElement {
            algebra : self.algebra,
            coefficient : 
                [&self.coefficient[0] + &rhs.coefficient[0],
                &self.coefficient[1] + &rhs.coefficient[1],
                &self.coefficient[2] + &rhs.coefficient[2], 
                &self.coefficient[3] + &rhs.coefficient[3]],
        }
    }
}

impl<'a> Mul for QuaternionAlgebraElement<'a> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self.algebra.zero();
        result.coefficient[0] = &self.coefficient[0] * &rhs.coefficient[0] -
                                &self.coefficient[1] * &rhs.coefficient[1] -
                                &self.coefficient[2] * &rhs.coefficient[2] * BigRational::from(self.algebra.prime.to_bigint().unwrap()) +
                                &self.coefficient[3] * &rhs.coefficient[3] * BigRational::from(self.algebra.prime.to_bigint().unwrap());
        result.coefficient[1] = &self.coefficient[0] * &rhs.coefficient[1] +
                                &self.coefficient[1] * &rhs.coefficient[0] +
                                &self.coefficient[2] * &rhs.coefficient[3] * BigRational::from(self.algebra.prime.to_bigint().unwrap()) -
                                &self.coefficient[3] * &rhs.coefficient[2] * BigRational::from(self.algebra.prime.to_bigint().unwrap());
        result.coefficient[2] = &self.coefficient[0] * &rhs.coefficient[2] +
                                &self.coefficient[2] * &rhs.coefficient[0] -
                                &self.coefficient[1] * &rhs.coefficient[3] +
                                &self.coefficient[3] * &rhs.coefficient[1];
        result.coefficient[3] = &self.coefficient[0] * &rhs.coefficient[3] +
                                &self.coefficient[3] * &rhs.coefficient[0] +
                                &self.coefficient[1] * &rhs.coefficient[2] -
                                &self.coefficient[2] * &rhs.coefficient[1];
        result
        
    }
}

#[cfg(test)]
mod tests {
    use num::{BigUint, BigInt};
    use num_rational::BigRational;

    use super::QuaternionAlgebra;
    #[test]
    fn quaternion_test() {
        let quaternion_alg = QuaternionAlgebra::new(&BigUint::from(13 as u32));
        let a = quaternion_alg.gen([BigRational::new(BigInt::from(3), BigInt::from(4)), BigRational::new(BigInt::from(3), BigInt::from(4)), BigRational::new(BigInt::from(3), BigInt::from(4)), BigRational::new(BigInt::from(3), BigInt::from(4))]);
        let b = a.clone() * a.clone();
        println!("{:?}", b.clone() + b.clone());
    }
}