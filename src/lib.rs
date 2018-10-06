//! Scalar trait, which generalizes complex and real number.

extern crate num_complex;
extern crate num_traits;
extern crate rand;

use num_traits::*;
use std::fmt::Debug;
use std::ops::*;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

pub trait Field:
    'static
    + Copy
    + Zero
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Debug
{
}

impl<T> Field for T where
    T: 'static
        + Copy
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Debug
{}

pub trait LeftAction<RHS, Output>:
    Field
    + Add<RHS, Output = Output>
    + Sub<RHS, Output = Output>
    + Mul<RHS, Output = Output>
    + Div<RHS, Output = Output>
{
}

impl LeftAction<f32, f32> for f32 {}
impl LeftAction<f32, c32> for c32 {}
impl LeftAction<c32, c32> for f32 {}
impl LeftAction<c32, c32> for c32 {}
impl LeftAction<f64, f64> for f64 {}
impl LeftAction<f64, c64> for c64 {}
impl LeftAction<c64, c64> for f64 {}
impl LeftAction<c64, c64> for c64 {}

pub trait RightAction<LHS, Output>: Field
where
    LHS: Add<Self, Output = Output>
        + Sub<Self, Output = Output>
        + Mul<Self, Output = Output>
        + Div<Self, Output = Output>,
{
}

impl RightAction<f32, f32> for f32 {}
impl RightAction<f32, c32> for c32 {}
impl RightAction<c32, c32> for f32 {}
impl RightAction<c32, c32> for c32 {}
impl RightAction<f64, f64> for f64 {}
impl RightAction<f64, c64> for c64 {}
impl RightAction<c64, c64> for f64 {}
impl RightAction<c64, c64> for c64 {}

pub trait BiAction<Target, Output>:
    LeftAction<Target, Output> + RightAction<Target, Output>
where
    Target: LeftAction<Self, Output> + RightAction<Self, Output>,
{
}

impl BiAction<f32, f32> for f32 {}
impl BiAction<f32, c32> for c32 {}
impl BiAction<c32, c32> for f32 {}
impl BiAction<c32, c32> for c32 {}
impl BiAction<f64, f64> for f64 {}
impl BiAction<f64, c64> for c64 {}
impl BiAction<c64, c64> for f64 {}
impl BiAction<c64, c64> for c64 {}

pub trait Scalar: Field {
    type Real: Scalar<Real = Self::Real, Complex = Self::Complex>
        + BiAction<Self::Complex, Self::Complex>;
    type Complex: Scalar<Real = Self::Real, Complex = Self::Complex>
        + BiAction<Self::Real, Self::Complex>;
}

impl Scalar for f32 {
    type Real = f32;
    type Complex = c32;
}
impl Scalar for c32 {
    type Real = f32;
    type Complex = c32;
}
impl Scalar for f64 {
    type Real = f64;
    type Complex = c64;
}
impl Scalar for c64 {
    type Real = f64;
    type Complex = c64;
}
