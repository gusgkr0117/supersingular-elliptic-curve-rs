//! Implementation of finite fields using [num](https://crates.io/crates/num) library
use std::ops::{Add, Sub, Mul};
use num::{BigInt, BigUint, Zero};
use num::bigint::{ToBigInt, Sign};
use num_prime::buffer::NaiveBuffer;
use num_prime::buffer::PrimeBufferExt;
/// Base trait for Field types
pub trait Field<'a, T:FieldElement<'a>> : DynZero<'a, Output=T> {}
/// FieldElement must refer a Field
/// Thus it must take as input the lifetime of the Field
pub trait FieldElement<'a> : Add<Output=Self> + Sub + Mul + Sized + Clone {}

/// Trait for the additive identity of a dynamic field type
/// Output a field element
pub trait DynZero<'a> {
    type Output : FieldElement<'a>;
    fn zero(self) -> Self::Output;
}

/// Type for a base of a finite field(extension degree = 1)
#[derive(Debug)]
pub struct FiniteField {
    prime : BigUint,
}

impl FiniteField {
    /// The input [BigUint](num::BigUint) must be prime or it will panic
    pub fn new(prime : &BigUint) -> Self {
        let pb = NaiveBuffer::new();
        if !pb.is_prime(prime, None).probably() {
            panic!("The number is not prime");
        }
        FiniteField {
            prime : prime.clone()
        }
    }

    /// Output an initialized [FiniteFieldElement](FiniteFieldElement)
    pub fn gen(&self, num : &BigInt) -> FiniteFieldElement {
        FiniteFieldElement::new(self, num)
    }

    /// Output its prime
    fn prime(&self) -> BigUint {
        self.prime.clone()
    }
}

/// FiniteField type is a [Field](Field)
impl<'a> Field<'a, FiniteFieldElement<'a>> for &'a FiniteField {}

/// Implement DynZero for [FiniteField](FiniteField)
impl<'a> DynZero<'a> for &'a FiniteField{
    type Output = FiniteFieldElement<'a>;
    fn zero(self) -> FiniteFieldElement<'a> {
        FiniteFieldElement::new(self, &BigInt::zero())
    }
}

/// FiniteFieldElement type for the elements in [FiniteField](FiniteField)
/// FiniteFieldElement refers to a [FiniteField](FiniteField)
#[derive(Clone, Debug)]
pub struct FiniteFieldElement<'a> {
    field : &'a FiniteField,
    num : BigUint,
}

impl<'a> FieldElement<'a> for FiniteFieldElement<'a> {}

//impl PrimalityBase for BigInt {}
impl<'a> FiniteFieldElement<'a> {
    pub fn new(field : &'a FiniteField, num : &BigInt) -> Self {
        let prime = field.prime().clone();
        let mut tmp = num % prime.to_bigint().unwrap();
        if tmp.sign() == Sign::Minus {
            tmp = &tmp + &prime.to_bigint().unwrap();
        }

        FiniteFieldElement {
            field,
            num : tmp.to_biguint().unwrap(),
        }
    }
}

impl Add for FiniteFieldElement<'_> {
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        let mut result = FiniteFieldElement::new(self.field, &BigInt::zero());
        result.num = (&self.num + &rhs.num) % &self.field.prime();
        result
    }
}

impl Sub for FiniteFieldElement<'_> {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self {
        let mut result = FiniteFieldElement::new(self.field, &BigInt::zero());
        result.num = (&self.num + &self.field.prime() - &rhs.num) % &self.field.prime();
        result
    }
}

impl Mul for FiniteFieldElement<'_> {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self {
        let mut result = FiniteFieldElement::new(self.field, &BigInt::zero());
        result.num = (&self.num * &rhs.num) % &self.field.prime();
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
    let mut a = FiniteFieldElement::new(&fp, &BigInt::new(Sign::Minus,vec![2]));
    a = a.clone() + a.clone();
    let b = a.clone() * a.clone();
    let c = b.clone() * b.clone();

    println!("{:?}", b);
    println!("{:?}", c);
}
}