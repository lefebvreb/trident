use std::convert::Infallible;
use std::ops::Deref;
use std::rc::Rc;

use crate::genericity::Id;

use super::operation::OpKind;
use super::{QuantumCircuit, parameter};
use super::parameter::Parameter;
use super::symbol::{Qubit, Bit};

pub trait InstrRead {
    fn is_empty(&self) -> bool;

    fn len(&self) -> u32;

    fn read_word(&mut self) -> u32;

    fn read_words(&mut self, n: u32) -> &[u32];
}

impl InstrRead for &[u32] {
    #[inline]
    fn is_empty(&self) -> bool {
        <[u32]>::is_empty(self)
    }

    #[inline]
    fn len(&self) -> u32 {
        <[u32]>::len(self) as u32
    }

    #[inline]
    fn read_word(&mut self) -> u32 {
        // TODO: replace with <[T]>::take_first when it is eventually stabilized
        let (&front, tail) = self.split_first().unwrap();
        *self = tail;
        front
    }

    #[inline]
    fn read_words(&mut self, n: u32) -> &[u32] {
        // TODO: replace with <[T]>::take when it is eventually stabilized
        let (left, right) = self.split_at(n as usize);
        *self = right;
        left
    }
}

pub trait InstrWrite {
    fn write_word(&mut self, word: u32);

    fn write_words(&mut self, words: &[u32]);
}

impl InstrWrite for Vec<u32> {
    #[inline]
    fn write_word(&mut self, word: u32) {
        self.push(word);
    }

    #[inline]
    fn write_words(&mut self, words: &[u32]) {
        self.extend(words)
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
            pub fn write<W: InstrWrite>(&self, writer: &mut W) {
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
    pub fn write<W: InstrWrite>(&self, writer: &mut W) {

    }

    pub fn read<R: InstrRead>(&mut self, reader: &mut R) {

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

    fn transpile<R: InstrRead, W: InstrWrite>(reader: &mut R, writer: &mut W) -> Result<(), Self::Error>;
}

pub struct Complete;

impl InstrSet for Complete {
    type Error = Infallible;

    fn transpile<R: InstrRead, W: InstrWrite>(reader: &mut R, writer: &mut W) -> Result<(), Self::Error> {
        writer.write_words(reader.read_words(reader.len()));
        Ok(())
    }
}