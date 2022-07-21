use std::convert::Infallible;
use std::mem;

use crate::bitset::BitSet;
use crate::genericity::Id;

use super::operation::OpKind;
use super::{QuantumCircuit, parameter, storage};
use super::parameter::Parameter;
use super::symbol::{Qubit, Bit};

pub struct Compute<'id, T> {
    pub bits: &'id [Bit<'id>],
    pub func: fn(BitSet) -> T,
}

impl<'id, T> Compute<'id, T> {
    #[inline]
    pub(crate) fn read(src: &mut &[u32]) -> Self {
        Self {
            bits: {
                let n = storage::read(src);
                Bit::reads(src, n)
            },
            func: {
                let data = [(); storage::USIZE_LEN].map(|_| storage::read(src));
                // SAFETY: we know there is such a function pointer there.
                unsafe { mem::transmute(data) }
            },
        }
    }

    #[inline]
    pub(crate) fn write(&self, dest: &mut Vec<u32>) {
        storage::write(dest, self.bits.len() as u32);
        Bit::writes(dest, self.bits);
        // SAFETY: array of u32 accepts any bit pattern.
        let data: [u32; storage::USIZE_LEN] = unsafe { mem::transmute(self.func) };
        data.iter().for_each(|&n| storage::write(dest, n));
    }
}

macro_rules! modifiers {
    {
        $(
            $name: ident $(($inner: ident: $payload: ty))? => $int: literal,
        )*
    } => {
        pub enum Modifier<'id> {
            $(
                $name $(($payload))?,
            )*
        }

        impl Modifier<'_> {
            pub fn write(&self, dest: &mut Vec<u32>) {
                match self {
                    $(
                        Self::$name $(($inner))? => {
                            
                        }
                        _ => (),
                    )*
                }
            }
        }
    }
}

modifiers! {
    IfBit(inner: Bit<'id>) => 0,
    IfCompute(inner: Compute<'id, ()>) => 1,
}

#[derive(Default)]
pub struct Instr<'id> {
    pub kind: OpKind,
    pub qubits: &'id [Qubit<'id>],
    pub bits: &'id [Bit<'id>],
    pub parameters: &'id [Parameter<'id>],
    pub modifier: Option<Modifier<'id>>,
}

impl Instr<'_> {
    #[inline]
    pub(crate) fn read(&mut self, src: &mut &[u32]) {

    }

    #[inline]
    pub(crate) fn write(&self, dest: &mut Vec<u32>) {

    }
}

// flags: has_modifier + kind
// qubit_count?
// qubits
// bit_count?
// bits
// parameter_count?
// parameters
// modifier_kind
// modifier_payload?

// examples:

// flags: no + h
// qubits: t
// => 8

// flags: no + compute
// payload: 
//  out_count
//  out_bits
//  fn(&mut BitSet)
// => 16 + 4n

// flags: yes + cx
// qubits: c, t
// condition_kind: loop
// condition_payload: bit

pub trait InstrSet {
    type Error;

    fn transpile(src: &mut &[u32], dest: &mut Vec<u32>) -> Result<(), Self::Error>;
}

pub struct Complete;

impl InstrSet for Complete {
    type Error = Infallible;

    fn transpile(src: &mut &[u32], dest: &mut Vec<u32>) -> Result<(), Self::Error> {
        dest.extend(*src);
        Ok(())
    }
}