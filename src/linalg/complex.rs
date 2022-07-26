use std::fmt;
use std::iter::{Sum, Product};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

/// Represents a [complex number](https://en.wikipedia.org/wiki/Complex_number) in
/// cartesian coordinates with two [`f64`]s.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, Debug)]
pub struct c64 {
    /// Real part of the complex number.
    pub re: f64,
    /// Imaginary part of the complex number.
    pub im: f64,
}

impl c64 {
    /// The precision $ p $ with which to compare two complex numbers.
    /// Two complex numbers $ z_1 $ and $ z_2 $ will be considered equal when:
    /// 
    /// $ z_1^* z_2 < p^2 $
    pub const PRECISION: f64 = 0.0;

    /// The complex number representing $ 0 $.
    pub const ZERO: Self = Self::new(0.0, 0.0);
    /// The complex number representing $ 1 $.
    pub const ONE: Self = Self::new(1.0, 0.0);
    /// The complex number representing the imaginary unit, $ i $.
    pub const I: Self = Self::new(0.0, 1.0);

    // Constructors

    /// Creates a new complex number, from it's real and imaginary parts.
    /// 
    /// $ \textrm{new} (a, b) \coloneqq a + i b $
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Creates a new complex number in [Euler](https://en.wikipedia.org/wiki/Polar_coordinate_system#Complex_numbers)
    /// form (or exponential, or polar), from it's radius and argument.
    /// 
    /// $ \textrm{euler} (r, \theta) \coloneqq r e^{i \theta} $
    pub fn euler(r: f64, theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self::new(r * cos, r * sin)
    }

    /// Creates a new complex number in Euler form (or exponential, or polat), 
    /// with a radius of 1 and the given argument. See 
    /// [this](https://en.wikipedia.org/wiki/Cis_(mathematics)) wikipedia page on
    /// the cis function in mathematics.
    /// 
    /// $ \textrm{cis} (\theta) \coloneqq \cos (\theta) + i \sin (\theta) = e^{i \theta} $
    pub fn cis(theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self::new(cos, sin)
    }

    // Accessors

    /// Returns the [real part](https://en.wikipedia.org/wiki/Complex_number#Notation) of the complex number.
    /// 
    /// $ \textrm{re} (a + i b) \coloneqq a $
    pub const fn re(&self) -> f64 {
        self.re
    }

    /// Returns the [imaginary part](https://en.wikipedia.org/wiki/Complex_number#Notation) of the complex number.
    /// 
    /// $ \textrm{im} (a + i b) \coloneqq b $
    pub const fn im(&self) -> f64 {
        self.im
    }

    /// Returns the [radius and the argument](https://en.wikipedia.org/wiki/Polar_coordinate_system#Complex_numbers) of the complex number.
    /// 
    /// $ \textrm{to\textunderscore euler} (z) \coloneqq |z|, \arg (z) $
    pub fn to_euler(&self) -> (f64, f64) {
        (self.abs(), self.arg())
    }

    // Complex-exclusive operations

    /// Returns the complex [conjugate](https://en.wikipedia.org/wiki/Complex_number#Conjugate) of the complex number.
    /// 
    /// $ \textrm{conj} (a + i b) \coloneqq a - i b $
    pub fn conj(&self) -> Self {
        Self::new(self.re, -self.im)
    }

    /// Returns the [modulus](https://en.wikipedia.org/wiki/Modulus_(mathematics)) (or radius, or absolute value) of this complex number.
    /// 
    /// $ \textrm{abs} (a + ib) = |a+ib| = a^2 + b^2 $
    pub fn abs(&self) -> f64 {
        self.re.hypot(self.im)
    }

    /// Returns the square of the [modulus](https://en.wikipedia.org/wiki/Modulus_(mathematics)) (or radius, or absolute value) of the complex number.
    /// This spares a square root operation if unecessary.
    /// 
    /// $ \textrm{abs\textunderscore sqr} (a + i b) \coloneqq a^2 + b^2 $
    pub fn abs_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Returns the [argument](https://en.wikipedia.org/wiki/Argument_(complex_analysis)) of the complex number.
    /// 
    /// $ \textrm{arg} (a + i b) \coloneqq \textrm{atan2} (b, a) $
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Returns the [multiplicative inverse](https://en.wikipedia.org/wiki/Multiplicative_inverse#Complex_numbers) (or reciprocal) of this complex number.
    /// 
    /// $ \textrm{inv} (z) = z^{-1} $
    pub fn recip(&self) -> Self {
        self.conj() * self.abs_sqr().recip()
    }
}

impl fmt::Display for c64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:+}i", self.re, self.im)
    }
}

// Implements the arithmetic operation $op for this complex type.
macro_rules! complex_op {
    { $op: ident, $fn: ident, $op_assign: ident, $fn_assign: ident, $complex_complex: expr, $complex_float: expr, $float_complex: expr } => {
        // complex ? complex

        impl $op<c64> for c64 {
            type Output = c64;

            fn $fn(self, rhs: c64) -> c64 {
                $complex_complex(self, rhs)
            }
        }

        impl $op<c64> for &c64 {
            type Output = c64;

            fn $fn(self, rhs: c64) -> c64 {
                $complex_complex(*self, rhs)
            }
        }

        impl $op<&c64> for c64 {
            type Output = c64;

            fn $fn(self, rhs: &c64) -> c64 {
                $complex_complex(self, *rhs)
            }
        }

        impl $op<&c64> for &c64 {
            type Output = c64;

            fn $fn(self, rhs: &c64) -> c64 {
                $complex_complex(*self, *rhs)
            }
        }

        // complex ? float

        impl $op<f64> for c64 {
            type Output = c64;

            fn $fn(self, rhs: f64) -> c64 {
                $complex_float(self, rhs)
            }
        }

        impl $op<f64> for &c64 {
            type Output = c64;

            fn $fn(self, rhs: f64) -> c64 {
                $complex_float(*self, rhs)
            }
        }

        impl $op<&f64> for c64 {
            type Output = c64;

            fn $fn(self, rhs: &f64) -> c64 {
                $complex_float(self, *rhs)
            }
        }

        impl $op<&f64> for &c64 {
            type Output = c64;

            fn $fn(self, rhs: &f64) -> c64 {
                $complex_float(*self, *rhs)
            }
        }

        // float ? complex

        impl $op<c64> for f64 {
            type Output = c64;

            fn $fn(self, rhs: c64) -> c64 {
                $float_complex(self, rhs)
            }
        }

        impl $op<c64> for &f64 {
            type Output = c64;

            fn $fn(self, rhs: c64) -> c64 {
                $float_complex(*self, rhs)
            }
        }

        impl $op<&c64> for f64 {
            type Output = c64;

            fn $fn(self, rhs: &c64) -> c64 {
                $float_complex(self, *rhs)
            }
        }

        impl $op<&c64> for &f64 {
            type Output = c64;

            fn $fn(self, rhs: &c64) -> c64 {
                $float_complex(*self, *rhs)
            }
        }

        // complex ?= complex

        impl $op_assign<c64> for c64 {
            fn $fn_assign(&mut self, rhs: c64) {
                *self = self.$fn(rhs);
            }
        }

        impl $op_assign<&c64> for c64 {
            fn $fn_assign(&mut self, rhs: &c64) {
                *self = self.$fn(rhs);
            }
        }

        // complex ?= float

        impl $op_assign<f64> for c64 {
            fn $fn_assign(&mut self, rhs: f64) {
                *self = self.$fn(rhs);
            }
        }

        impl $op_assign<&f64> for c64 {
            fn $fn_assign(&mut self, rhs: &f64) {
                *self = self.$fn(rhs);
            }
        }
    }
}

// Addition
complex_op! {
    Add, add, AddAssign, add_assign,
    |a: c64, b: c64| c64::new(a.re + b.re, a.im + b.im),
    |a: c64, b: f64| c64::new(a.re + b, a.im),
    |a: f64, b: c64| c64::new(a + b.re, b.im)
}

// Subtraction
complex_op! {
    Sub, sub, SubAssign, sub_assign,
    |a: c64, b: c64| c64::new(a.re - b.re, a.im - b.im),
    |a: c64, b: f64| c64::new(a.re - b, a.im),
    |a: f64, b: c64| c64::new(a - b.re, -b.im)
}

// Multiplication
complex_op! {
    Mul, mul, MulAssign, mul_assign,
    |a: c64, b: c64| c64::new(a.re * b.re - a.im * b.im, a.re * b.im + a.im * b.re),
    |a: c64, b: f64| c64::new(a.re * b, a.im * b),
    |a: f64, b: c64| c64::new(b.re * a, b.im * a)
}

// Division
complex_op! {
    Div, div, DivAssign, div_assign,
    |a: c64, b: c64| {
        let denom = b.abs_sqr().recip();
        c64::new(a.re * b.re + a.im * b.im, a.im * b.re - a.re * b.im) * denom
    },
    |a: c64, b: f64| a * b.recip(),
    |a: f64, b: c64| a / b.abs_sqr() * b.conj()
}

// Negation

impl Neg for c64 {
    type Output = c64;

    /// Negates the complex number.
    fn neg(self) -> c64 {
        c64::new(-self.re, -self.im)
    }
}

impl Neg for &c64 {
    type Output = c64;

    /// Negates the complex number.
    fn neg(self) -> c64 {
        c64::new(-self.re, -self.im)
    }
}

impl Sum for c64 {
    /// Sums an iterator of complex numbers.
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl Product for c64 {
    /// Returns the product of an iterator of complex numbers.
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl PartialEq for c64 {
    fn eq(&self, other: &Self) -> bool {
        (self - other).abs_sqr() <= Self::PRECISION * Self::PRECISION
    }
}