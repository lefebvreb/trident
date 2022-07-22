#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Arity(u32);

impl From<u32> for Arity {
    #[inline]
    fn from(n: u32) -> Self {
        Self(n)
    }
}

impl Arity {
    #[inline]
    pub fn variadic() -> Self {
        Self(u32::MAX)
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.0 != u32::MAX
    }

    #[inline]
    pub fn is_variadic(self) -> bool {
        self.0 == u32::MAX
    }

    #[inline]
    pub fn get(self) -> Option<u32> {
        self.is_finite().then(|| self.0)
    }
}

macro_rules! operations {
    {
        $(
            $name: ident {
                qubits: $qubits: expr,
                bits: $bits: expr,
                parameters: $parameters: expr,
                unitary: $unitary: literal,
            },
        )*
    } => {
        #[non_exhaustive]
        #[derive(Clone, PartialEq, Eq, Default, Debug)]
        pub enum OpKind {
            #[default]
            $($name,)*
        }

        impl OpKind {
            #[inline]
            pub fn qubits(self) -> Arity {
                match self {
                    $(Self::$name => $qubits.into(),)*
                }
            }

            #[inline]
            pub fn bits(self) -> Arity {
                match self {
                    $(Self::$name => $bits.into(),)*
                }
            }

            #[inline]
            pub fn parameters(self) -> Arity {
                match self {
                    $(Self::$name => $parameters.into(),)*
                }
            }

            #[inline]
            pub fn is_unitary(self) -> bool {
                match self {
                    $(Self::$name => $unitary,)*
                }
            }
        }
    }
}

operations! {
    Nop {
        qubits: 0,
        bits: 0,
        parameters: 0,
        unitary: false,
    },
    H {
        qubits: 1,
        bits: 0,
        parameters: 0,
        unitary: true,
    },
}