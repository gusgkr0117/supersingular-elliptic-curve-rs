use std::ops::{Add, Sub, Neg};
use std::fmt;

/// Base trait for Field types
pub trait Group<'a> : Clone {
    type Element : GroupElement + 'a;
    /// Trait for the additive identity of a dynamic field type
    /// Output a field element
    fn zero(&'a self) -> Self::Element;
}

/// FieldElement must refer a Field
/// Thus it must take as input the lifetime of the Field
pub trait GroupElement : Add<Output=Self> + Sub<Output=Self> + Neg<Output=Self> + 
Sized + PartialEq + Clone + fmt::Debug {}