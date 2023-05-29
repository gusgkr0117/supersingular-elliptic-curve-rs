//! The elliptic curves of montgomery form
use crate::field::{Field, FieldElement};

/// The structure of a montgomery curve
pub struct MontgomeryCurve<'a, F, E> where F: Field<'a, E>, E: FieldElement<'a> {
    field : &'a F,
    coeff : E,
}

/// The structure of a (projective coordinate)point of a montgomery curve
pub struct MontgomeryCurvePoint<'a, F, E> where F: Field<'a, E>, E: FieldElement<'a> {
    curve : &'a MontgomeryCurve<'a, F, E>,
    x : E,
    y : E,
    z : E,
}