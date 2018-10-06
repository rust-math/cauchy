//! Scalar trait, which generalizes complex and real number.
//!
//! Examples
//! --------
//!
//! ```
//! # use num_scalar::Scalar;
//! fn add_int<A: Scalar>(a: A) -> A {
//!     a + A::from(1).unwrap()  // A::from is inhereted from num_traits::NumCast
//! }
//! fn add_float<A: Scalar>(a: A) -> A {
//!     a + A::from(1.0).unwrap()
//! }
//! fn add_real<A: Scalar>(a: A) -> A::Real {
//!     a.re() + A::real(1.0)
//! }
//! fn add_complex<A: Scalar>(a: A) -> A::Complex {
//!     a.as_complex() + A::complex(1.0, 1.0)
//! }
//! ```

extern crate num_complex;
extern crate num_traits;
extern crate rand;

use num_traits::*;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

pub trait Scalar: NumAssign + FromPrimitive + NumCast + Copy {
    type Real: Scalar<Real = Self::Real, Complex = Self::Complex> + NumOps<Self::Real, Self::Real>;
    type Complex: Scalar<Real = Self::Real, Complex = Self::Complex>
        + NumOps<Self::Real, Self::Complex>
        + NumOps<Self::Complex, Self::Complex>;

    fn real<T: ToPrimitive>(re: T) -> Self::Real {
        <Self::Real as NumCast>::from(re).unwrap()
    }

    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex;

    fn re(self) -> Self::Real;
    fn im(self) -> Self::Real;
    fn as_complex(self) -> Self::Complex;
}

impl Scalar for f32 {
    type Real = f32;
    type Complex = c32;
    fn re(self) -> Self::Real {
        self
    }
    fn im(self) -> Self::Real {
        0.0
    }
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
        c32 {
            re: re.to_f32().unwrap(),
            im: im.to_f32().unwrap(),
        }
    }
    fn as_complex(self) -> Self::Complex {
        c32::new(self, 0.0)
    }
}
impl Scalar for c32 {
    type Real = f32;
    type Complex = c32;
    fn re(self) -> Self::Real {
        self.re
    }
    fn im(self) -> Self::Real {
        self.im
    }
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
        c32 {
            re: re.to_f32().unwrap(),
            im: im.to_f32().unwrap(),
        }
    }
    fn as_complex(self) -> Self::Complex {
        self
    }
}
