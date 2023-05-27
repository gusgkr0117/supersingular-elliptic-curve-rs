//! Implementation of finite fields using [num](https://crates.io/crates/num) library
use std::ops::{Add, Sub, Mul};
use num::{BigInt, BigUint, Zero};
use num::bigint::{ToBigInt, Sign};
use num_prime::buffer::NaiveBuffer;
use num_prime::buffer::PrimeBufferExt;
/// Base trait for Field types
pub trait Field : Add + Sub + Mul + Sized{}
impl<T> Field for T where T: Add + Sub + Mul + Sized{}

#[derive(Clone, Debug)]
pub struct FiniteField {
    prime : BigUint,
    num : BigUint,
}

//impl PrimalityBase for BigInt {}
impl FiniteField {
    fn new(prime : &BigUint, num : &BigInt) -> Self {
        let pb = NaiveBuffer::new();
        if !pb.is_prime(prime, None).probably() {
            panic!("The number is not prime");
        }

        let mut tmp = num % prime.to_bigint().unwrap();
        if tmp.sign() == Sign::Minus {
            tmp = &tmp + &prime.to_bigint().unwrap();
        }

        FiniteField {
            prime : prime.clone(),
            num : tmp.to_biguint().unwrap(),
        }
    }
}

impl Add for FiniteField {
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        let mut result = FiniteField::new(&self.prime, &BigInt::zero());
        result.num = (&self.num + &rhs.num) % &self.prime;
        result
    }
}

impl Sub for FiniteField {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self {
        let mut result = FiniteField::new(&self.prime, &BigInt::zero());
        result.num = (&self.num + &self.prime - &rhs.num) % &self.prime;
        result
    }
}

impl Mul for FiniteField {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self {
        let mut result = FiniteField::new(&self.prime, &BigInt::zero());
        result.num = (&self.num * &rhs.num) % &self.prime;
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
    let mut a = FiniteField::new(&p, &BigInt::new(Sign::Minus,vec![2]));
    a = a.clone() + a.clone();
    let b = a.clone() * a.clone();
    let c = b.clone() * b.clone();

    println!("{:?}", b);
    println!("{:?}", c);
}
}