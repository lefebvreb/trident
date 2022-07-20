use std::convert::Infallible;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::rc::Rc;

use crate::genericity::Id;

use super::gate::Gate;
use super::{QuantumCircuit, parameter};
use super::parameter::Parameter;
use super::symbol::{Qubit, Bit};

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
            pub fn write<W: Write>(&self, writer: &mut W) {
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
    pub kind: Gate,
    pub qubits: &'id [Qubit<'id>],
    pub bits: &'id [Bit<'id>],
    pub parameters: &'id [Parameter<'id>],
    pub modifier: Option<Modifier<'id>>,
}

impl Instr<'_> {
    pub fn write<W: Write>(&self, writer: &mut W) {

    }

    pub fn read<R: Read>(&mut self, reader: &mut R) {

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

    fn transpile<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Self::Error>;
}

pub struct Complete;

impl InstrSet for Complete {
    type Error = Infallible;

    fn transpile<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Self::Error> {
        io::copy(reader, writer).expect("Cannot copy from reader to writer");
        Ok(())
    }
}