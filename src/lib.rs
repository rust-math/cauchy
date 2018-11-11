//! Scalar trait, which generalizes complex and real number.
//!
//! Examples
//! --------
//!
//! ```
//! # use cauchy::Scalar;
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

use num_complex::Complex;
use num_traits::{Float, FromPrimitive, NumAssign, NumCast, NumOps, ToPrimitive};

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

    /// Absolute value
    fn abs(&self) -> Self::Real;
    /// Sqaure of absolute value
    fn abs_sqr(&self) -> Self::Real;

    fn sqrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
    fn asinh(&self) -> Self;
    fn acosh(&self) -> Self;
    fn atanh(&self) -> Self;
}

macro_rules! impl_float {
    ($name:ident) => {
        #[inline]
        fn $name(&self) -> Self {
            Float::$name(*self)
        }
    }
}

macro_rules! impl_complex {
    ($name:ident) => {
        #[inline]
        fn $name(&self) -> Self {
            Complex::$name(self)
        }
    };
}

macro_rules! impl_scalar {
    ($real:ty, $complex:ty) => {
        impl Scalar for $real {
            type Real = $real;
            type Complex = $complex;

            #[inline]
            fn re(&self) -> Self::Real {
                *self
            }
            #[inline]
            fn im(&self) -> Self::Real {
                0.0
            }
            #[inline]
            fn real<T: ToPrimitive>(re: T) -> Self::Real {
                NumCast::from(re).unwrap()
            }
            #[inline]
            fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
                Complex {
                    re: NumCast::from(re).unwrap(),
                    im: NumCast::from(im).unwrap(),
                }
            }
            #[inline]
            fn as_c(&self) -> Self::Complex {
                Complex::new(*self, 0.0)
            }
            #[inline]
            fn conj(&self) -> Self {
                *self
            }
            #[inline]
            fn abs_sqr(&self) -> Self::Real {
                self * self
            }

            impl_float!(sqrt);
            impl_float!(abs);
            impl_float!(exp);
            impl_float!(ln);
            impl_float!(sin);
            impl_float!(cos);
            impl_float!(tan);
            impl_float!(sinh);
            impl_float!(cosh);
            impl_float!(tanh);
            impl_float!(asin);
            impl_float!(acos);
            impl_float!(atan);
            impl_float!(asinh);
            impl_float!(acosh);
            impl_float!(atanh);
        }

        impl Scalar for $complex {
            type Real = $real;
            type Complex = $complex;

            #[inline]
            fn re(&self) -> Self::Real {
                self.re
            }
            #[inline]
            fn im(&self) -> Self::Real {
                self.im
            }
            #[inline]
            fn real<T: ToPrimitive>(re: T) -> Self::Real {
                NumCast::from(re).unwrap()
            }
            #[inline]
            fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex {
                Complex {
                    re: NumCast::from(re).unwrap(),
                    im: NumCast::from(im).unwrap(),
                }
            }
            #[inline]
            fn as_c(&self) -> Self::Complex {
                *self
            }
            #[inline]
            fn conj(&self) -> Self {
                Complex::conj(self)
            }
            #[inline]
            fn abs_sqr(&self) -> Self::Real {
                Complex::norm_sqr(self)
            }
            #[inline]
            fn abs(&self) -> Self::Real {
                Complex::norm(self)
            }

            impl_complex!(sqrt);
            impl_complex!(exp);
            impl_complex!(ln);
            impl_complex!(sin);
            impl_complex!(cos);
            impl_complex!(tan);
            impl_complex!(sinh);
            impl_complex!(cosh);
            impl_complex!(tanh);
            impl_complex!(asin);
            impl_complex!(acos);
            impl_complex!(atan);
            impl_complex!(asinh);
            impl_complex!(acosh);
            impl_complex!(atanh);
        }
    };
}

impl_scalar!(f32, c32);
impl_scalar!(f64, c64);
