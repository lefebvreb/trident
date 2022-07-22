use std::convert::Infallible;

use bitflags::bitflags;

use crate::bitset::BitSet;

use super::operation::OpKind;
use super::storage;
use super::parameter::Parameter;
use super::symbol::{Qubit, Bit};

bitflags! {
    /// Flags attached to the compact representation of an
    /// instruction.
    #[repr(transparent)]
    pub(crate) struct InstrFlags: u16 {
        /// Wether or not the instruction has a modifier.
        const HAS_MODIFIER = 1;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Compute<'id, T> {
    pub bits: &'id [Bit<'id>],
    pub func: fn(BitSet) -> T,
}

impl<'id, T> Compute<'id, T> {
    /// Writes the compute to the destination.
    #[inline]
    pub(crate) fn write(&self, dest: &mut Vec<u32>) {
        storage::write(dest, self.bits.len() as u32);
        storage::write_slice(dest, self.bits);
        storage::write(dest, self.func);
    }

    /// Reads the compute from the destination.
    #[inline]
    pub(crate) fn read(src: &mut &'id [u32]) -> Self {
        Self {
            bits: {
                let len = storage::read(src);
                storage::read_slice(src, len)
            },
            func: storage::read(src),
        }
    }
}

macro_rules! modifiers {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident = $int: literal $({
                $inner: ident: $payload: ty,
                write: $write: expr,
                read: $read: expr,
            })?,
        )*
    } => {
        #[non_exhaustive]
        #[derive(Clone, PartialEq, Eq, Debug)]
        pub enum Modifier<'id> {
            $(
                $(#[doc$($args)*])*
                $name $(($payload))?,
            )*
        }

        impl<'id> Modifier<'id> {
            /// Writes the modifier to the destination.
            #[inline]
            pub(crate) fn write(&self, dest: &mut Vec<u32>) {
                match self {
                    $(
                        Self::$name $(($inner))? => {
                            storage::write(dest, $int as u32);
                            $($write(dest);)?
                        }
                    )*
                }
            }

            /// Reads the modifier from the destination.
            #[inline]
            pub(crate) fn read(src: &mut &'id [u32]) -> Self {
                match storage::read::<u32>(src) {
                    $(
                        $int => Self::$name $(($read(src)))?,
                    )*
                    _ => panic!("invalid modifier")
                }
            }
        }
    }
}

modifiers! {
    /// Only perform the instruction if the bit is `true`.
    IfBit = 0 {
        inner: Bit<'id>,
        write: |dest| storage::write(dest, inner),
        read: storage::read,
    },
    /// Only perform the instruction if the result of the compute is `true`.
    IfCompute = 1 {
        inner: Compute<'id, bool>,
        write: |dest| inner.write(dest),
        read: Compute::read,
    },
    /// Perform the instruction while the bit is `true`.
    WhileBit = 2 {
        inner: Bit<'id>,
        write: |dest| storage::write(dest, inner),
        read: storage::read,
    },
    /// Perform the instruction while the result of the compute is `true`.
    WhileCompute = 3 {
        inner: Compute<'id, bool>,
        write: |dest| inner.write(dest),
        read: Compute::read,
    },
    /// Perform the instruction as many times as the provided integer.
    ForConst = 4 {
        inner: u32,
        write: |dest| storage::write(dest, inner),
        read: storage::read,
    },
    /// Perform the instruction as many times as the result of the compute.
    ForCompute = 5 {
        inner: Compute<'id, u32>,
        write: |dest| inner.write(dest),
        read: Compute::read,
    },
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Instr<'id> {
    /// The base operation type.
    pub op: OpKind<'id>,
    /// The qubits to apply this operation to.
    pub qubits: &'id [Qubit<'id>],
    /// The bits to apply this operation to.
    pub bits: &'id [Bit<'id>],
    /// The parameters this operation depends on.
    pub parameters: &'id [Parameter<'id>],
    /// This operation's modifier, if there is one.
    pub modifier: Option<Modifier<'id>>,
}

impl<'id> Instr<'id> {
    #[inline]
    pub(crate) fn write(&self, dest: &mut Vec<u32>) {
        let flags = {
            let mut res = InstrFlags::empty();

            if self.modifier.is_some() {
                res |= InstrFlags::HAS_MODIFIER;
            }

            res
        };

        self.op.write(dest, flags);

        macro_rules! write_slices {
            ( $($name: ident),* ) => {
                $(
                    if self.op.$name().is_variadic() {
                        storage::write(dest, self.$name.len() as u32);
                    }
                    storage::write_slice(dest, self.$name);
                )*
            }
        }

        write_slices!(qubits, bits, parameters);

        self.modifier.as_ref().map(|modifier| modifier.write(dest));
    }

    #[inline]
    pub(crate) fn read(&mut self, src: &mut &'id [u32]) {
        let (op, flags) = OpKind::read(src);

        macro_rules! read_slices {
            ( $($name: ident),* ) => {
                $(
                    let len = self.op.$name().get().unwrap_or_else(|| storage::read(src));
                    self.$name = storage::read_slice(src, len);
                )*
            };
        }

        read_slices!(qubits, bits, parameters);

        self.modifier = flags.contains(InstrFlags::HAS_MODIFIER).then(|| Modifier::read(src));
    }
}

pub trait InstrSet {
    type Error;

    fn transpile(src: &mut &[u32], dest: &mut Vec<u32>) -> Result<(), Self::Error>;
}

pub struct CompleteInstrSet;

impl InstrSet for CompleteInstrSet {
    type Error = Infallible;

    #[inline]
    fn transpile(src: &mut &[u32], dest: &mut Vec<u32>) -> Result<(), Self::Error> {
        dest.extend(*src);
        Ok(())
    }
}