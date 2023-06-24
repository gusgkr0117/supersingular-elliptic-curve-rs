//! Implementation of the rational isogenies("accessible" isogenies)
use crate::elliptic_curve::mont_curve::{MontgomeryCurve, MontgomeryCurvePoint};
use crate::field::{Field, FieldElement};

/// The rational isogeny
struct RationalIsogeny<'a, F: Field<'a> + 'a> {
    domain_curve : MontgomeryCurve<'a, F>,
    codomain_curve : Option<MontgomeryCurve<'a, F>>,
    kernel_generator_point : MontgomeryCurvePoint<'a, F>,
    domain_basis : Option<(MontgomeryCurvePoint<'a, F>, MontgomeryCurvePoint<'a, F>)>,
    codomain_basis : Option<(MontgomeryCurvePoint<'a, F>, MontgomeryCurvePoint<'a, F>)>,
}

impl<'a, F: Field<'a> + 'a> RationalIsogeny<'a, F> {
    pub fn new(domain_curve : &MontgomeryCurve<'a, F>, kernel_generator_point : &MontgomeryCurvePoint<'a, F>) -> Self {
        // TODO : compute the codomain curve and pair of corresponding basis of domain and codomain curves
        RationalIsogeny {
            domain_curve : domain_curve.clone(),
            codomain_curve : None,
            kernel_generator_point : kernel_generator_point.clone(),
            domain_basis : None,
            codomain_basis : None,
        }
    }

    pub fn eval(&self, point : MontgomeryCurvePoint<'a, F>) -> MontgomeryCurvePoint<'a, F> {
        // TODO : evaluate the input point on the domain curve
        point
    }
}
