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
use num_traits::{Float, FromPrimitive, NumAssign, NumCast, NumOps, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::iter::{Product, Sum};
use std::ops::Neg;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;

pub trait RealScalar: Scalar + NumOps<Self, Self> + PartialOrd {
    /// Approximate number of significant digits in base 10.
    const DIGITS: u32;
    /// Machine epsilon value for f32.
    const EPSILON: Self;
    /// Infinity (∞).
    const INFINITY: Self;
    /// Number of significant digits in base 2.
    const MANTISSA_DIGITS: u32;
    /// Largest finite f32 value.
    const MAX: Self;
    /// Maximum possible power of 10 exponent.
    const MAX_10_EXP: i32;
    /// Maximum possible power of 2 exponent.
    const MAX_EXP: i32;
    /// Smallest finite f32 value.
    const MIN: Self;
    /// Minimum possible normal power of 10 exponent.
    const MIN_10_EXP: i32;
    /// One greater than the minimum possible normal power of 2 exponent.
    const MIN_EXP: i32;
    /// Smallest positive normal f32 value.
    const MIN_POSITIVE: Self;
    /// Not a Number (NaN).
    const NAN: Self;
    /// Negative infinity (-∞).
    const NEG_INFINITY: Self;
    /// The radix or base of the internal representation of f32.
    const RADIX: u32;

    // Mathematical constants

    /// Euler's number (e)
    const E: Self;
    /// 1/π
    const FRAC_1_PI: Self;
    /// 2/π
    const FRAC_2_PI: Self;
    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;
    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;
    /// ln(2)
    const LN_2: Self;
    /// ln(10)
    const LN_10: Self;
    /// log2(e)
    const LOG2_E: Self;
    /// log10(e)
    const LOG10_E: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// sqrt(2)
    const SQRT_2: Self;
}

macro_rules! impl_float_const {
    ($float:ident, $const_type:ty, $constant:ident) => {
        const $constant: $const_type = std::$float::$constant;
    };
}

macro_rules! impl_float_math_const {
    ($float:ident, $constant:ident) => {
        const $constant: Self = std::$float::consts::$constant;
    };
}

macro_rules! impl_real_scalar {
    ($float:ident) => {
        impl RealScalar for $float {
            impl_float_const!($float, u32, DIGITS);
            impl_float_const!($float, Self, EPSILON);
            impl_float_const!($float, Self, INFINITY);
            impl_float_const!($float, u32, MANTISSA_DIGITS);
            impl_float_const!($float, Self, MAX);
            impl_float_const!($float, i32, MAX_10_EXP);
            impl_float_const!($float, i32, MAX_EXP);
            impl_float_const!($float, Self, MIN);
            impl_float_const!($float, i32, MIN_10_EXP);
            impl_float_const!($float, i32, MIN_EXP);
            impl_float_const!($float, Self, MIN_POSITIVE);
            impl_float_const!($float, Self, NAN);
            impl_float_const!($float, Self, NEG_INFINITY);
            impl_float_const!($float, u32, RADIX);

            impl_float_math_const!($float, E);
            impl_float_math_const!($float, FRAC_1_PI);
            impl_float_math_const!($float, FRAC_2_PI);
            impl_float_math_const!($float, FRAC_2_SQRT_PI);
            impl_float_math_const!($float, FRAC_1_SQRT_2);
            impl_float_math_const!($float, FRAC_PI_2);
            impl_float_math_const!($float, FRAC_PI_3);
            impl_float_math_const!($float, FRAC_PI_4);
            impl_float_math_const!($float, FRAC_PI_6);
            impl_float_math_const!($float, FRAC_PI_8);
            impl_float_math_const!($float, LN_2);
            impl_float_math_const!($float, LN_10);
            impl_float_math_const!($float, LOG2_E);
            impl_float_math_const!($float, LOG10_E);
            impl_float_math_const!($float, PI);
            impl_float_math_const!($float, SQRT_2);
        }
    };
}

impl_real_scalar!(f32);
impl_real_scalar!(f64);

pub trait Scalar:
    NumAssign
    + FromPrimitive
    + NumCast
    + Neg<Output = Self>
    + Copy
    + Debug
    + Sum
    + Product
    + Serialize
    + for<'de> Deserialize<'de>
    + 'static
{
    type Real: Scalar<Real = Self::Real, Complex = Self::Complex> + RealScalar;
    type Complex: Scalar<Real = Self::Real, Complex = Self::Complex>
        + NumOps<Self::Real, Self::Complex>
        + NumOps<Self::Complex, Self::Complex>;

    /// Create a new real number
    fn real<T: ToPrimitive>(re: T) -> Self::Real;
    /// Create a new complex number
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex;

    fn from_real(re: Self::Real) -> Self;

    fn pow(&self, n: Self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: Self::Real) -> Self;
    fn powc(&self, n: Self::Complex) -> Self::Complex;

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
    fn square(&self) -> Self::Real;

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
            fn from_real(re: Self::Real) -> Self {
                re
            }

            fn pow(&self, n: Self) -> Self {
                self.powf(n)
            }
            fn powi(&self, n: i32) -> Self {
                Float::powi(*self, n)
            }
            fn powf(&self, n: Self::Real) -> Self {
                Float::powf(*self, n)
            }
            fn powc(&self, n: Self::Complex) -> Self::Complex {
                self.as_c().powc(n)
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
            fn square(&self) -> Self::Real {
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
            fn from_real(re: Self::Real) -> Self {
                Self::new(re, Zero::zero())
            }

            fn pow(&self, n: Self) -> Self {
                self.powc(n)
            }
            fn powi(&self, n: i32) -> Self {
                self.powf(n as Self::Real)
            }
            fn powf(&self, n: Self::Real) -> Self {
                self.powf(n)
            }
            fn powc(&self, n: Self::Complex) -> Self::Complex {
                self.powc(n)
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
            fn square(&self) -> Self::Real {
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
