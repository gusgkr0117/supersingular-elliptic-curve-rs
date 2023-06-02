pub mod fp;
use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;

/// Base trait for Field types
pub trait Field<'a> : Clone {
    type Element : FieldElement + 'a;
    /// Trait for the additive identity of a dynamic field type
    /// Output a field element
    fn zero(&'a self) -> Self::Element;
    /// Trait for the multiplicative identity of a dynamic field type
    /// Output a field element
    fn one(&'a self) -> Self::Element;
}

/// FieldElement must refer a Field
/// Thus it must take as input the lifetime of the Field
pub trait FieldElement : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Neg<Output=Self> + 
MultiplicativeInverse + Sized + PartialEq + Clone + fmt::Debug {}

/// Field elements has its multiplicative inverse
pub trait MultiplicativeInverse {
    fn inv(&self) -> Self;
}