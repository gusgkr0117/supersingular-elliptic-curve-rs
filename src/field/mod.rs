pub mod fp;
use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;

/// Base trait for Field types
pub trait Field<'a, T:FieldElement<'a>> : DynZero<'a, Output=T> + DynOne<'a, Output=T> + Clone {}
/// FieldElement must refer a Field
/// Thus it must take as input the lifetime of the Field
pub trait FieldElement<'a> : Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Neg<Output=Self> + 
MultiplicativeInverse + Sized + PartialEq + Clone + fmt::Debug {}

/// Trait for the additive identity of a dynamic field type
/// Output a field element
pub trait DynZero<'a> {
    type Output : FieldElement<'a>;
    fn zero(self) -> Self::Output;
}

/// Trait for the multiplicative identity of a dynamic field type
/// Output a field element
pub trait DynOne<'a> {
    type Output : FieldElement<'a>;
    fn one(self) -> Self::Output;
}

/// Field elements has its multiplicative inverse
pub trait MultiplicativeInverse {
    fn inv(&self) -> Self;
}