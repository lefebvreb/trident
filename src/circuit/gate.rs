use std::io::{Read, Write};
use std::num::NonZeroU32;

macro_rules! operations {
    {
        $(
            $name: ident {
                qubits: $qubits: expr,
                bits: $bits: expr,
                parameters: $parameters: expr,
                is_unitary: $is_unitary: literal,
            }
        )*
    } => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub enum Op {
            $($name,)*
        }

        impl Op {
            pub fn qubits(self) -> Arity {
                match self {
                    $(Self::$name => qubits,)*
                }
            }

            pub fn bits(self) -> Arity {
                match self {
                    $(Self::$name => bits,)*
                }
            }

            pub fn parameters(self) -> Arity {
                match self {
                    $(Self::$name => parameters,)*
                }
            }

            pub fn is_unitary(self) -> bool {
                match self {
                    $(Self::$name => is_unitary,)*
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Gate {
    Nop,
    H,
    X,
    Y,
    Z,
    Rx,
    Ry,
    Rz,
    Cx,
    Ccx,
}

impl Default for Gate {
    #[inline]
    fn default() -> Self {
        Self::Nop
    }
}

pub enum Arity {
    Variadic,
    N(NonZeroU32),
}

impl Arity {
    /// Returns an arity with an integer value of `n`. `n` must be
    /// non-zero, or else this function panic.
    ///
    /// # Panics
    /// 
    /// Panics if `n == 0`.
    #[inline]
    pub fn new(n: u32) -> Self {
        Self::N(n.try_into().unwrap())
    }

    #[inline]
    pub fn to_int(self) -> Option<u32> {
        match self {
            Self::N(n) => Some(n.into()),
            _ => None,
        }
    }
}