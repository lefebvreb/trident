use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

macro_rules! complex_impl {
    {
        $(#[doc$($args: tt)*])*
        $name: ident => $float: ident,
    } => {
        $(#[doc$($args)*])*
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, PartialEq, Default, Debug)]
        pub struct $name {
            /// Real part of the complex number.
            pub re: $float,
            /// Imaginary part of the complex number.
            pub im: $float,
        }

        impl $name {
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
            pub const fn new(re: $float, im: $float) -> Self {
                Self { re, im }
            }

            /// Creates a new complex number in [Euler](https://en.wikipedia.org/wiki/Polar_coordinate_system#Complex_numbers)
            /// form (or exponential, or polar), from it's radius and argument.
            /// 
            /// $ \textrm{euler} (r, \theta) \coloneqq r e^{i \theta} $
            pub fn euler(r: $float, theta: $float) -> Self {
                let (sin, cos) = theta.sin_cos();
                Self::new(r * cos, r * sin)
            }

            /// Creates a new complex number in Euler form (or exponential, or polat), 
            /// with a radius of 1 and the given argument. See 
            /// [this](https://en.wikipedia.org/wiki/Cis_(mathematics)) wikipedia page on
            /// the cis function in mathematics.
            /// 
            /// $ \textrm{cis} (\theta) \coloneqq \cos (\theta) + i \sin (\theta) = e^{i \theta} $
            pub fn cis(theta: $float) -> Self {
                let (sin, cos) = theta.sin_cos();
                Self::new(cos, sin)
            }

            // Accessors
        
            /// Returns the [real part](https://en.wikipedia.org/wiki/Complex_number#Notation) of the complex number.
            /// 
            /// $ \textrm{re} (a + i b) \coloneqq a $
            pub const fn re(self) -> $float {
                self.re
            }
        
            /// Returns the [imaginary part](https://en.wikipedia.org/wiki/Complex_number#Notation) of the complex number.
            /// 
            /// $ \textrm{im} (a + i b) \coloneqq b $
            pub const fn im(self) -> $float {
                self.im
            }

            /// Returns the [radius and the argument](https://en.wikipedia.org/wiki/Polar_coordinate_system#Complex_numbers) of the complex number.
            /// 
            /// $ \textrm{to\textunderscore euler} (z) \coloneqq |z|, \arg (z) $
            pub fn to_euler(self) -> ($float, $float) {
                (self.abs(), self.arg())
            }

            // Complex-exclusive operations
        
            /// Returns the complex [conjugate](https://en.wikipedia.org/wiki/Complex_number#Conjugate) of the complex number.
            /// 
            /// $ \textrm{conj} (a + i b) \coloneqq a - i b $
            pub fn conj(self) -> Self {
                Self::new(self.re, -self.im)
            }

            /// Returns the [modulus](https://en.wikipedia.org/wiki/Modulus_(mathematics)) (or radius, or absolute value) of this complex number.
            /// 
            /// $ \textrm{abs} (a + ib) = |a+ib| = a^2 + b^2 $
            pub fn abs(self) -> $float {
                self.re.hypot(self.im)
            }

            /// Returns the square of the [modulus](https://en.wikipedia.org/wiki/Modulus_(mathematics)) (or radius, or absolute value) of the complex number.
            /// This spares a square root operation if unecessary.
            /// 
            /// $ \textrm{abs\textunderscore sqr} (a + i b) \coloneqq a^2 + b^2 $
            pub fn abs_sqr(self) -> $float {
                self.re * self.re + self.im * self.im
            }
        
            /// Returns the [argument](https://en.wikipedia.org/wiki/Argument_(complex_analysis)) of the complex number.
            /// 
            /// $ \textrm{arg} (a + i b) \coloneqq \textrm{atan2} (b, a) $
            pub fn arg(self) -> $float {
                self.im.atan2(self.re)
            }

            /// Returns the [multiplicative inverse](https://en.wikipedia.org/wiki/Multiplicative_inverse#Complex_numbers) (or reciprocal) of this complex number.
            /// 
            /// $ \textrm{inv} (z) = z^{-1} $
            pub fn recip(self) -> Self {
                self.conj() * self.abs_sqr().recip()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}{:+}i", self.re, self.im)
            }
        }

        // Implements the arithmetic operation $op for this complex type.
        macro_rules! complex_op {
            { $op: ident, $fn: ident, $op_assign: ident, $fn_assign: ident, $complex_complex: expr, $complex_float: expr, $float_complex: expr } => {
                // complex ? complex
        
                impl $op<$name> for $name {
                    type Output = $name;
        
                    fn $fn(self, rhs: $name) -> $name {
                        $complex_complex(self, rhs)
                    }
                }
        
                impl $op<$name> for &$name {
                    type Output = $name;
        
                    fn $fn(self, rhs: $name) -> $name {
                        $complex_complex(*self, rhs)
                    }
                }
        
                impl $op<&$name> for $name {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$name) -> $name {
                        $complex_complex(self, *rhs)
                    }
                }
        
                impl $op<&$name> for &$name {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$name) -> $name {
                        $complex_complex(*self, *rhs)
                    }
                }
        
                // complex ? float
        
                impl $op<$float> for $name {
                    type Output = $name;
        
                    fn $fn(self, rhs: $float) -> $name {
                        $complex_float(self, rhs)
                    }
                }
        
                impl $op<$float> for &$name {
                    type Output = $name;
        
                    fn $fn(self, rhs: $float) -> $name {
                        $complex_float(*self, rhs)
                    }
                }
        
                impl $op<&$float> for $name {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$float) -> $name {
                        $complex_float(self, *rhs)
                    }
                }
        
                impl $op<&$float> for &$name {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$float) -> $name {
                        $complex_float(*self, *rhs)
                    }
                }
        
                // float ? complex
        
                impl $op<$name> for $float {
                    type Output = $name;
        
                    fn $fn(self, rhs: $name) -> $name {
                        $float_complex(self, rhs)
                    }
                }
        
                impl $op<$name> for &$float {
                    type Output = $name;
        
                    fn $fn(self, rhs: $name) -> $name {
                        $float_complex(*self, rhs)
                    }
                }
        
                impl $op<&$name> for $float {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$name) -> $name {
                        $float_complex(self, *rhs)
                    }
                }
        
                impl $op<&$name> for &$float {
                    type Output = $name;
        
                    fn $fn(self, rhs: &$name) -> $name {
                        $float_complex(*self, *rhs)
                    }
                }

                // complex ?= complex
        
                impl $op_assign<$name> for $name {
                    fn $fn_assign(&mut self, rhs: $name) {
                        *self = self.$fn(rhs);
                    }
                }
        
                impl $op_assign<&$name> for $name {
                    fn $fn_assign(&mut self, rhs: &$name) {
                        *self = self.$fn(rhs);
                    }
                }
        
                // complex ?= float
        
                impl $op_assign<$float> for $name {
                    fn $fn_assign(&mut self, rhs: $float) {
                        *self = self.$fn(rhs);
                    }
                }
        
                impl $op_assign<&$float> for $name {
                    fn $fn_assign(&mut self, rhs: &$float) {
                        *self = self.$fn(rhs);
                    }
                }
            }
        }

        // Addition
        complex_op! {
            Add, add, AddAssign, add_assign,
            |a: $name, b: $name| $name::new(a.re + b.re, a.im + b.im),
            |a: $name, b: $float| $name::new(a.re + b, a.im),
            |a: $float, b: $name| $name::new(a + b.re, b.im)
        }

        // Subtraction
        complex_op! {
            Sub, sub, SubAssign, sub_assign,
            |a: $name, b: $name| $name::new(a.re - b.re, a.im - b.im),
            |a: $name, b: $float| $name::new(a.re - b, a.im),
            |a: $float, b: $name| $name::new(a - b.re, -b.im)
        }

        // Multiplication
        complex_op! {
            Mul, mul, MulAssign, mul_assign,
            |a: $name, b: $name| $name::new(a.re * b.re - a.im * b.im, a.re * b.im + a.im * b.re),
            |a: $name, b: $float| $name::new(a.re * b, a.im * b),
            |a: $float, b: $name| $name::new(b.re * a, b.im * a)
        }

        // Division
        complex_op! {
            Div, div, DivAssign, div_assign,
            |a: $name, b: $name| {
                let denom = b.abs_sqr().recip();
                $name::new(a.re * b.re + a.im * b.im, a.im * b.re - a.re * b.im) * denom
            },
            |a: $name, b: $float| a * b.recip(),
            |a: $float, b: $name| a / b.abs_sqr() * b.conj()
        }

        // Negation

        impl Neg for $name {
            type Output = $name;

            /// Negates the complex number.
            fn neg(self) -> $name {
                $name::new(-self.re, -self.im)
            }
        }

        impl Neg for &$name {
            type Output = $name;

            /// Negates the complex number.
            fn neg(self) -> $name {
                $name::new(-self.re, -self.im)
            }
        }
    }
}

complex_impl! {
    /// Represents a [complex number](https://en.wikipedia.org/wiki/Complex_number) in
    /// cartesian coordinates with two [`f32`]s.
    c32 => f32,
}

complex_impl! {
    /// Represents a [complex number](https://en.wikipedia.org/wiki/Complex_number) in
    /// cartesian coordinates with two [`f64`]s.
    c64 => f64,
}

impl From<c32> for c64 {
    /// This conversion is lossless.
    fn from(c: c32) -> c64 {
        c64::new(c.re.into(), c.im.into())
    }
}