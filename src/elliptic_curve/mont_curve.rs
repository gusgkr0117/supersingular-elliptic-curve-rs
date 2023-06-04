//! The elliptic curves of montgomery form
use crate::field::{Field, FieldElement};
use crate::group::{Group, GroupElement};
use std::ops::{Add, Sub, Neg};
use num::BigInt;

/// The structure of a montgomery curve
#[derive(Clone, Debug)]
pub struct MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    field : &'a F,
    A : F::Element,
}

impl<'a, F> MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    pub fn new(field : &'a F, A : F::Element) -> Self {
        MontgomeryCurve {
            field,
            A,
        }
    }

    fn gen(&'a self, (x,y,z) : (&F::Element, &F::Element, &F::Element)) -> MontgomeryCurvePoint<'a, F> {
        MontgomeryCurvePoint {
            curve : self,
            x: x.clone(), y: y.clone(), z: z.clone(),
        }
    }
}

impl<'a, F> Group<'a> for MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    type Element = MontgomeryCurvePoint<'a, F>;
    /// Implement DynZero for [FiniteField](FiniteField)
    fn zero(&'a self) -> Self::Element {
        self.gen((&self.field.zero(), &self.field.one(), &self.field.zero()))
    }
}

/// The structure of a (projective coordinate)point of a montgomery curve
#[derive(Debug, Clone)]
pub struct MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    curve : &'a MontgomeryCurve<'a, F>,
    x : F::Element,
    y : F::Element,
    z : F::Element,
}

impl<'a, F> MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn reduce(&mut self) {
        let lambda_inv = match self.z.is_zero() {
            true => self.y.clone().inv(),
            false => self.z.clone().inv(),
        };

        self.x = self.x.clone() *lambda_inv.clone();
        self.y = self.y.clone() *lambda_inv.clone();
        self.z = self.z.clone() *lambda_inv.clone();
    }
}

impl<'a, F> GroupElement for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn is_zero(&self) -> bool {
        self.z.is_zero()
    }
}

impl<'a, F> PartialEq for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn eq(&self, rhs : &Self) -> bool {
        if !self.x.is_zero() {
            let lambda = self.x.inv().clone() * rhs.x.clone();
            return self.y.clone() * lambda.clone() == rhs.y && self.z.clone() * lambda == rhs.z;
        }

        if !self.y.is_zero() {
            let lambda = self.y.inv().clone() * rhs.y.clone();
            return self.x.clone() * lambda.clone() == rhs.x && self.z.clone() * lambda == rhs.z;
        }
        
        let lambda = self.z.inv().clone() * rhs.z.clone();
        self.x.clone() * lambda.clone() == rhs.x && self.y.clone() * lambda == rhs.y
    }
}

impl<'a, F> Neg for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn neg(self) -> Self {
        if self.z.is_zero() {
            return self;
        }

        let mut result = self.curve.gen((&self.x, &-self.y.clone(), &self.z));
        result.reduce();
        result
    }
}

impl<'a, F> Add for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        if self.is_zero() {
            return rhs;
        }

        if rhs.is_zero() {
            return self;
        }

        if self == -rhs.clone() {
            return self.curve.zero();
        }

        let lambda = match self == rhs {
            true => {(self.y.clone() - rhs.y.clone()) * (self.x.clone() - rhs.x.clone()).inv()},
            false => {(self.x.clone() * self.x.clone() * BigInt::from(3) + self.curve.A.clone() * self.x.clone() * BigInt::from(2) + self.curve.field.one()) * (self.y.clone() + self.y.clone()).inv()},
        };

        let new_x = lambda.clone() * lambda.clone() - (self.x.clone() + rhs.x.clone()) - self.curve.A.clone();
        let new_y = lambda.clone() * (self.x.clone() - new_x.clone()) - self.y.clone();

        self.curve.gen((&new_x, &new_y, &self.curve.field.one()))
    }
}

impl<'a, F> Sub for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self {
        self + (-rhs)
    }
}

#[cfg(test)]
mod tests{
    use crate::{field::{fp::FiniteField, Field}, group::Group};
    use num::{BigUint, BigInt};

    use super::{MontgomeryCurve, MontgomeryCurvePoint};

    #[test]
    fn montgomery_curve_test() {
        let fp = FiniteField::new(&BigUint::from(13 as u32));
        let curve = MontgomeryCurve::new(&fp, fp.zero());
        let point = curve.gen((&fp.gen(&BigInt::from(0)), &fp.gen(&BigInt::from(1)), &fp.gen(&BigInt::from(0))));

        let Q = point.clone() + point.clone();
        assert!(Q == curve.zero());
    }
}