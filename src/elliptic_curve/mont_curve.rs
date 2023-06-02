//! The elliptic curves of montgomery form
use crate::field::{Field};

/// The structure of a montgomery curve
pub struct MontgomeryCurve<'a, F> where F: Field<'a> + 'a {
    field : &'a F,
    coeff : F::Element,
}

/// The structure of a (projective coordinate)point of a montgomery curve
pub struct MontgomeryCurvePoint<'a, F> where F: Field<'a> + 'a {
    curve : &'a MontgomeryCurve<'a, F>,
    x : F::Element,
    y : F::Element,
    z : F::Element,
}