use crate::bitset::BitSet;

use super::instruction::{Compute, InstrFlags};
use super::storage;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Arity(u32);

impl Arity {
    pub fn new(n: u32) -> Option<Self> {
        (n != u32::MAX).then(|| Self(n))
    }

    pub fn variadic() -> Self {
        Self(u32::MAX)
    }

    pub fn is_variadic(self) -> bool {
        self.0 == u32::MAX
    }

    pub fn is_definite(self) -> bool {
        !self.is_variadic()
    }

    pub fn get(self) -> Option<u32> {
        self.is_definite().then(|| self.0)
    }
}

impl From<u32> for Arity {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

macro_rules! operations {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident = $int: literal {
                qubits: $qubits: expr,
                bits: $bits: expr,
                parameters: $parameters: expr,
                unitary: $unitary: literal,
                label: $label: literal,
                $(
                    payload: {
                        $inner: ident: $payload: ty,
                        write: $write: expr,
                        read: $read: expr,
                    },
                )?
            },
        )*
    } => {
        #[non_exhaustive]
        #[derive(Clone, PartialEq, Eq, Default, Debug)]
        pub enum OpKind<'id> {
            #[default]
            $(
                $(#[doc$($args)*])*
                $name $(($payload))?,
            )*
        }

        impl<'id> OpKind<'id> {
            /// Writes the operation kind along with the given flags to the destination.
            pub(crate) fn write(&self, dest: &mut Vec<u32>, flags: InstrFlags) {
                match self {
                    $(
                        Self::$name $(($inner))? => {
                            storage::write(dest, (flags, $int as u16));
                            $($write(dest);)?
                        }
                    )*
                }
            }

            /// Reads the operation kind along with it's associated flags from the destination.
            pub(crate) fn read(src: &mut &'id [u32]) -> (Self, InstrFlags) {
                let (flags, id): (InstrFlags, u16) = storage::read(src);

                let op = match id {
                    $(
                        $int => Self::$name $(($read(src)))?,
                    )*
                    _ => panic!("invalid operation kind")
                };

                (op, flags)
            }

            #[allow(unused_variables)]
            pub fn qubits(&self) -> Arity {
                match self {
                    $(Self::$name $(($inner))? => $qubits.into(),)*
                }
            }

            #[allow(unused_variables)]
            pub fn bits(&self) -> Arity {
                match self {
                    $(Self::$name $(($inner))? => $bits.into(),)*
                }
            }

            #[allow(unused_variables)]
            pub fn parameters(&self) -> Arity {
                match self {
                    $(Self::$name $(($inner))? => $parameters.into(),)*
                }
            }

            #[allow(unused_variables)]
            pub fn is_unitary(&self) -> bool {
                match self {
                    $(Self::$name $(($inner))? => $unitary,)*
                }
            }

            #[allow(unused_variables)]
            pub fn label(&self) -> &'static str {
                match self {
                    $(Self::$name $(($inner))? => $label,)*
                }
            }
        }
    }
}

operations! {
    /// No-operation: do nothing.
    Nop = 0 {
        qubits: 0,
        bits: 0,
        parameters: 0,
        unitary: false,
        label: "nop",
    },
    /// Hadamard transform.
    H = 1 {
        qubits: 1,
        bits: 0,
        parameters: 0,
        unitary: true,
        label: "h",
    },
    /// Compute node, performs an arbitrary classical compute on bits,
    /// as defined by a custom function.
    Compute = 100 {
        qubits: 0,
        bits: Arity::variadic(),
        parameters: 0,
        unitary: false,
        label: "compute",
        payload: {
            inner: Compute<'id, BitSet>,
            write: |dest| inner.write(dest),
            read: Compute::read,
        },
    },
}