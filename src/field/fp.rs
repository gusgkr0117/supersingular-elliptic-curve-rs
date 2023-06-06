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

    /// Compute the power operation
    fn pow(&self, exponent : &BigInt) -> Self {
        let mut result = self.field.one();
        let mut tmp_value = self.clone();

        for digit in exponent.iter_u32_digits() {
            for i in 0..32 {
                if (digit >> i) & 1 == 1 {
                    result = result.clone() * tmp_value.clone();
                }
                tmp_value = tmp_value.clone() * tmp_value.clone();
            }
        }
        match exponent.sign() {
            Sign::Minus => result.inv(),
            _ => result,
        }
    }

    /// Compute the square root using [Peralta's algorithm](https://arxiv.org/pdf/2206.07145.pdf)
    fn sqrt(&self) -> Option<Self> {
        let (mut u, mut v) : (FiniteFieldElement, FiniteFieldElement);
        let prime = self.field.prime();

        // Check the quadratic residuosity
        let check_exp : BigUint = (prime.clone() - BigUint::one()) >> 1;
        if self.pow(&check_exp.to_bigint().unwrap()) != self.field.one() {
            return None;
        }

        // the case of p=4k+3
        if prime.clone() % (4 as u32) == BigUint::from(3 as u32) {
            let exp : BigUint = (prime.clone() + BigUint::one()) >> 2;
            return Some(self.pow(&exp.to_bigint().unwrap()));
        }
        
        // Compute the square root
        loop {
            u = self.field.rand(None);
            v = self.field.one();

            // u + 1 * \sqrt{-a}
            // choose a non-zero-divisor
            if u.clone() * u.clone() == -self.clone() {continue}
            
            // 1 + 0 * \sqrt{-a}
            let (mut result_u, mut result_v) = (self.field.one(), self.field.zero());
        
            // compute odd number m such that (p - 1) = 2^e * m
            let e = (prime.clone() - BigUint::one()).trailing_zeros().unwrap();
            let m = (prime.clone() - BigUint::one()) >> e;

            // compute (u + 1 * \sqrt{-a})^{m}
            for digit in m.iter_u32_digits() {
                for i in 0..32 {
                    // compute result_u + result_v * \sqrt{-a} = (result_u + result_v * \sqrt{-a})(u + v * \sqrt{-a})
                    if (digit >> i) & 1 == 1 {
                        (result_u, result_v) = (result_u.clone() * u.clone() - self.clone() * result_v.clone() * v.clone(),
                                                result_v.clone() * u.clone() + result_u.clone() * v.clone());
                    }

                    // compute u + v * \sqrt{-a} = (u + v * \sqrt{a})^2
                    (u,v) = (u.clone() * u.clone() - self.clone() * v.clone() * v.clone(),
                            u.clone() * v.clone() + v.clone() * u.clone());
                }
            }

            if result_u.is_zero() || result_v.is_zero() {continue}

            for _ in 0..e {
                // (u + v * \sqrt{-a}) = (result_u + result_v * \sqrt{-a})^2
                u = result_u.clone() * result_u.clone() - self.clone() * result_v.clone() * result_v.clone();
                if u.is_zero() {
                    return Some(result_u.clone() * result_v.clone().inv());
                }
                
                // result_u + result_v * \sqrt{-a} = (result_u + result_v * \sqrt{-a})^2
                (result_u, result_v) = (u, result_u.clone() * result_v.clone() + result_v.clone() * result_u.clone());
            }

            panic!("Unreachable!");
        }
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
                tmp_value = tmp_value.clone() + tmp_value.clone();
            }
        }

        result
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn field_test() {
        let prime_num = 97;
        let p = BigUint::new(vec![prime_num]);
        let fp = FiniteField::new(&p);

        for _ in 0..100 {
            let mut a = fp.rand(None);
            while a.is_zero() {a = fp.rand(None);}
            let b = &a + &a;
            let c = &b * &b;

            assert_eq!(c.pow(&BigInt::from(prime_num - 1)), fp.one());
            let d = c.clone().sqrt().unwrap();
            println!("{:?}^2 -> {:?}", d, c);
            assert_eq!(d.clone() * d.clone(), c, "Wrong sqrt"); 
        }
    }
}