//! Implementation of finite fields using [num](https://crates.io/crates/num) library
use core::fmt;
use super::*;
use std::ops::{Add, Sub, Mul, Neg};
use num::integer::ExtendedGcd;
use num::{BigInt, BigUint, Zero, Integer, One, Signed};
use num::bigint::{ToBigInt, Sign, RandomBits};
use num_prime::buffer::NaiveBuffer;
use num_prime::buffer::PrimeBufferExt;
use impl_ops::*;
use rand::Rng;
use std::ops;

/// Type for a base of a finite field(extension degree = 1)
#[derive(Debug, Clone)]
pub struct FiniteField {
    prime : BigUint,
}

impl FiniteField {
    /// The input [BigUint](num::BigUint) must be prime or it will panic
    pub fn new(prime : &BigUint) -> Self {
        let pb = NaiveBuffer::new();
        assert!(pb.is_prime(prime, None).probably(), "The base number is not prime!");
        
        FiniteField {
            prime : prime.clone()
        }
    }

    /// Output an initialized [FiniteFieldElement](FiniteFieldElement)
    pub fn gen(&self, num : &BigInt) -> FiniteFieldElement {
        let mut tmp = num % self.prime().to_bigint().unwrap();
        if tmp.sign() == Sign::Minus {
            tmp = &tmp + &self.prime().to_bigint().unwrap();
        }

        FiniteFieldElement{
            field : self,
            num : tmp.to_biguint().unwrap(),    
        }
    }

    /// Output its prime
    fn prime(&self) -> BigUint {
        self.prime.clone()
    }
}

/// FiniteField type is a [Field](Field)
impl<'a> Field<'a> for FiniteField {
    type Element = FiniteFieldElement<'a>;
    /// Implement DynZero for [FiniteField](FiniteField)
    fn zero(&'a self) -> Self::Element {
        self.gen(&BigInt::zero())
    }
    /// Implement DynOne for [FiniteField](FiniteField)
    fn one(&'a self) -> Self::Element {
        self.gen(&BigInt::one())
    }

    /// Generate a random element
    fn rand(&'a self, size : Option<usize>) -> Self::Element {
        let mut rng = rand::thread_rng();
        let num : BigInt = match size {
            Some(size) => {
                rng.sample(RandomBits::new(size.try_into().unwrap()))   
            },
            None => {
                rng.sample(RandomBits::new(self.prime.bits()))
            }
        };

        self.gen(&num)
    }
}

/// FiniteFieldElement type for the elements in [FiniteField](FiniteField)
/// FiniteFieldElement refers to a [FiniteField](FiniteField)
#[derive(Clone)]
pub struct FiniteFieldElement<'a> {
    field : &'a FiniteField,
    num : BigUint,
}

impl<'a> fmt::Debug for FiniteFieldElement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.num)
    }
}

impl<'a> FieldElement for FiniteFieldElement<'a> {
    fn inv(&self) -> Self {
        let num_i = self.num.to_bigint().unwrap();
        let prime_i = self.field.prime().to_bigint().unwrap();
        let ExtendedGcd {gcd, mut x, y:_y} = num_i.extended_gcd(&prime_i);
        assert!(gcd == BigInt::one(), "There is no multiplicative inverse of {self:?}");

        x %= &prime_i;
        if x.is_negative() {
            x += prime_i;
        }

        FiniteFieldElement {
            field : self.field,
            num : x.to_biguint().unwrap(),
        }
    }

    fn is_zero(&self) -> bool {
        self.num.is_zero()
    }
}

impl<'a> Neg for FiniteFieldElement<'a> {
    type Output = Self;
    fn neg(self) -> Self {
        FiniteFieldElement { field: self.field, num: self.field.prime() - self.num }
    }
}

impl<'a> PartialEq for FiniteFieldElement<'a> {
    fn eq(&self, rhs:&Self) -> bool {
        assert!(self.field.prime() == rhs.field.prime(), "The base field is not equal");
        self.num == rhs.num
    }
}

impl_op!(+ |lhs: &FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(+ |lhs: FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(+ |lhs: &FiniteFieldElement<'a>, rhs: FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &rhs.num) % &lhs.field.prime();
    result
});

impl<'a> Add for FiniteFieldElement<'a> {
    type Output = FiniteFieldElement<'a>;
    fn add(self, rhs : FiniteFieldElement<'a>) -> Self::Output {
        let mut result = self.field.gen(&BigInt::zero());
        result.num = (&self.num + &rhs.num) % &self.field.prime();
        result
    }
}

impl_op!(- |lhs: &FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &lhs.field.prime() - &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(- |lhs: FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &lhs.field.prime() - &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(- |lhs: &FiniteFieldElement<'a>, rhs: FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num + &lhs.field.prime() - &rhs.num) % &lhs.field.prime();
    result
});

impl Sub for FiniteFieldElement<'_> {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self {
        let mut result = self.field.gen(&BigInt::zero());
        result.num = (&self.num + &self.field.prime() - &rhs.num) % &self.field.prime();
        result
    }
}

impl_op!(* |lhs: &FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num * &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(* |lhs: FiniteFieldElement<'a>, rhs: &FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num * &rhs.num) % &lhs.field.prime();
    result
});

impl_op!(* |lhs: &FiniteFieldElement<'a>, rhs: FiniteFieldElement<'a>| -> FiniteFieldElement<'a> {
    let mut result = lhs.field.gen(&BigInt::zero());
    result.num = (&lhs.num * &rhs.num) % &lhs.field.prime();
    result
});

impl Mul for FiniteFieldElement<'_> {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self {
        let mut result = self.field.zero();
        result.num = (&self.num * &rhs.num) % &self.field.prime();
        result
    }
}

/// Scalar multiplication as Z-module
impl Mul<BigInt> for FiniteFieldElement<'_> {
    type Output = Self;
    fn mul(self, rhs : BigInt) -> Self {
        let mut result = self.field.zero();
        let mut tmp_value = self.clone();
        for digit in rhs.iter_u32_digits() {
            for i in 0..32{
                if ((digit >> i) & 1) == 1 {
                    result = result.clone() + tmp_value.clone();
                }
                tmp_value = tmp_value.clone() * tmp_value.clone();
            }
        }

        result
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use num::bigint::Sign;
#[test]
fn field_test() {
    let p = BigUint::new(vec![11]);
    let fp = FiniteField::new(&p);
    let a = fp.gen(&BigInt::new(Sign::Minus,vec![2]));
    let b = &a + &a;
    let c = &b * &b;

    println!("{:?}", c);
    println!("{:?}", c.inv());
}
}