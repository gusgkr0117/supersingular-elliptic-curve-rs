//! The elliptic curves of montgomery form
use crate::field::Field;
use crate::group::{Group, GroupElement};
use std::ops::{Add, Sub, Neg};

/// The structure of a montgomery curve
#[derive(Clone, Debug)]
pub struct MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    field : &'a F,
    coeff : F::Element,
}

impl<'a, F> MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    fn new(field : &'a F, coeff : F::Element) -> Self {
        MontgomeryCurve {
            field,
            coeff,
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

impl<'a, F> GroupElement for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {}

impl<'a, F> PartialEq for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn eq(&self, rhs : &Self) -> bool {
        // TODO
        true
    }
}

impl<'a, F> Neg for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn neg(self) -> Self {
        // TODO
        self
    }
}

impl<'a, F> Add for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        // TODO
        self
    }
}

impl<'a, F> Sub for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self {
        // TODO
        self
    }
}