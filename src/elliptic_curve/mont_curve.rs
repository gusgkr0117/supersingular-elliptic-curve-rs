//! The elliptic curves of montgomery form
use crate::field::{Field, FieldElement};
use crate::group::{Group, GroupElement};
use std::ops::{Add, Sub, Neg};
use num::BigInt;
use impl_ops::impl_bin_ops;

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

    pub fn gen(&'a self, (x,y,z) : (&F::Element, &F::Element, &F::Element)) -> MontgomeryCurvePoint<'a, F> {
        MontgomeryCurvePoint {
            curve : self,
            x: x.clone(), y: y.clone(), z: z.clone(),
        }
    }

    pub fn j_invariant(&'a self) -> F::Element {
        (self.A.clone() * self.A.clone() - self.field.one() * BigInt::from(3)).pow(&BigInt::from(3)) * BigInt::from(256) * 
        (self.A.clone() * self.A.clone() - self.field.one() * BigInt::from(4)).inv()
    }

    /// Generate a nonzero random point
    pub fn rand(&'a self) -> MontgomeryCurvePoint<'a, F> {
        let mut x : F::Element;
        let y : F::Element;
        loop {
            x = self.field.rand(None);

            // y^2 = x^3 + A * x^2 + x
            let y_sqr = x.clone().pow(&BigInt::from(3)) + 
            self.A.clone() * x.clone().pow(&BigInt::from(2)) +
            x.clone();

            y = match y_sqr.sqrt(){
                Some(y) => y,
                None => continue,
            };
        
            break       
        }

        MontgomeryCurvePoint { curve: self, x, y, z: self.field.one() }
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

#[impl_bin_ops]
impl<'a, F> Add for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn add(self, rhs : MontgomeryCurvePoint<'a, F>) -> MontgomeryCurvePoint<'a, F> {
        if self.is_zero() {
            return rhs.clone();
        }

        if rhs.is_zero() {
            return self.clone();
        }

        if self.clone() == -rhs.clone() {
            return self.curve.zero();
        }

        let lambda = match self.clone() == rhs.clone() {
            false => {(self.y.clone() - rhs.y.clone()) * (self.x.clone() - rhs.x.clone()).inv()},
            true => {(self.x.clone() * self.x.clone() * BigInt::from(3) + self.curve.A.clone() * self.x.clone() * BigInt::from(2) + self.curve.field.one()) * (self.y.clone() * BigInt::from(2)).inv()},
        };

        let new_x = lambda.clone() * lambda.clone() - (self.x.clone() + rhs.x.clone()) - self.curve.A.clone();
        let new_y = lambda.clone() * (self.x.clone() - new_x.clone()) - self.y.clone();

        assert!(new_y.clone() * new_y.clone() == new_x.clone() * new_x.clone() * new_x.clone() + self.curve.A.clone() * new_x.clone() * new_x.clone() + new_x.clone(),
        "Addition is wrong!!!");
        self.curve.gen((&new_x, &new_y, &self.curve.field.one()))
    }
}

#[impl_bin_ops]
impl<'a, F> Sub for MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    fn sub(self, rhs : MontgomeryCurvePoint<'a, F>) -> MontgomeryCurvePoint<'a, F> {
        self + (-rhs.clone())
    }
}

#[cfg(test)]
mod tests{
    use crate::field::{fp::FiniteField, Field};
    use num::{BigUint};

    use super::{MontgomeryCurve};

    #[test]
    fn montgomery_curve_test() {
        let fp = FiniteField::new(&BigUint::from(97 as u32));
        let curve = MontgomeryCurve::new(&fp, fp.zero());

        println!("j-invariant : {:?}", curve.j_invariant());
        for _ in 0..1000{
            let (p1, p2, p3) = (curve.rand(), curve.rand(), curve.rand());
            // println!("{:?} {:?} {:?}", p1, p2, p3);
            assert_eq!((&p1 +&p2) + &p3, &p1 + (&p2 + &p3), "Associativity fail");
        }
    }
}