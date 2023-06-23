pub mod fp;
use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;
use num::BigInt;

/// Base trait for Field types
pub trait Field<'a> : Clone + fmt::Debug {
    type Element : FieldElement + 'a;
    /// Trait for the additive identity of a dynamic field type
    /// Output a field element
    fn zero(&'a self) -> Self::Element;
    /// Trait for the multiplicative identity of a dynamic field type
    /// Output a field element
    fn one(&'a self) -> Self::Element;

    /// Generate a random field element
    /// self.rand(Some(size)) or self.rand(None)
    fn rand(&'a self, size : Option<usize>) -> Self::Element;
}

/// FieldElement must refer a Field
/// Thus it must take as input the lifetime of the Field
pub trait FieldElement : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> +
Mul<BigInt, Output=Self> + Neg<Output=Self> + 
Sized + PartialEq + Clone + fmt::Debug {
    /// Field elements has its multiplicative inverse
    fn inv(&self) -> Self;
    /// Whether it's zero or not
    fn is_zero(&self) -> bool;
    /// Compute the power operation
    fn pow(&self, exponent : &BigInt) -> Self;
    /// Compute the square root
    fn sqrt(&self) -> Option<Self>;
}