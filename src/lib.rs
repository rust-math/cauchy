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
//!     a.as_c() + A::complex(1.0, 1.0)
//! }
//! ```

extern crate num_complex;
extern crate num_traits;
extern crate rand;

use num_complex::Complex;
use num_traits::*;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

pub trait Scalar: NumAssign + FromPrimitive + NumCast + Copy {
    type Real: Scalar<Real = Self::Real, Complex = Self::Complex> + NumOps<Self::Real, Self::Real>;
    type Complex: Scalar<Real = Self::Real, Complex = Self::Complex>
        + NumOps<Self::Real, Self::Complex>
        + NumOps<Self::Complex, Self::Complex>;

    /// Create a new real number
    fn real<T: ToPrimitive>(re: T) -> Self::Real;
    /// Create a new complex number
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex;

    /// Real part
    fn re(&self) -> Self::Real;
    /// Imaginary part
    fn im(&self) -> Self::Real;
    /// As a complex number
    fn as_c(&self) -> Self::Complex;
    /// Complex conjugate
    fn conj(&self) -> Self;

    /// Square root
    fn sqrt(&self) -> Self;
    /// Absolute value
    fn abs(&self) -> Self::Real;
    /// Sqaure of absolute value
    fn abs_sqr(&self) -> Self::Real;

    /// Exponential
    fn exp(&self) -> Self;
    /// Natural Logarithm
    fn ln(&self) -> Self;
}

impl Scalar for f32 {
    type Real = f32;
    type Complex = c32;

    fn re(&self) -> Self::Real {
        *self
    }
    fn im(&self) -> Self::Real {
        0.0
    }
    fn real<T: ToPrimitive>(re: T) -> Self::Real {
        NumCast::from(re).unwrap()
    }
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
        Complex {
            re: NumCast::from(re).unwrap(),
            im: NumCast::from(im).unwrap(),
        }
    }
    fn as_c(&self) -> Self::Complex {
        c32::new(*self, 0.0)
    }
    fn conj(&self) -> Self {
        *self
    }

    fn sqrt(&self) -> Self {
        Float::sqrt(*self)
    }
    fn abs(&self) -> Self::Real {
        Float::abs(*self)
    }
    fn abs_sqr(&self) -> Self::Real {
        self * self
    }
    fn exp(&self) -> Self {
        Float::exp(*self)
    }
    fn ln(&self) -> Self {
        Float::ln(*self)
    }
}

impl Scalar for c32 {
    type Real = f32;
    type Complex = c32;

    fn re(&self) -> Self::Real {
        self.re
    }
    fn im(&self) -> Self::Real {
        self.im
    }
    fn real<T: ToPrimitive>(re: T) -> Self::Real {
        NumCast::from(re).unwrap()
    }
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
        c32 {
            re: NumCast::from(re).unwrap(),
            im: NumCast::from(im).unwrap(),
        }
    }
    fn as_c(&self) -> Self::Complex {
        *self
    }
    fn conj(&self) -> Self {
        Complex::conj(&self)
    }
    fn sqrt(&self) -> Self {
        Complex::sqrt(self)
    }
    fn abs(&self) -> Self::Real {
        Complex::norm(self)
    }
    fn abs_sqr(&self) -> Self::Real {
        Complex::norm_sqr(self)
    }
    fn exp(&self) -> Self {
        Complex::exp(self)
    }
    fn ln(&self) -> Self {
        Complex::ln(self)
    }
}
