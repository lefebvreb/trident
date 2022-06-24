pub mod bit;
pub mod instruction;
pub mod parameter;
pub mod qubit;

use thiserror::Error;

use crate::genericity::Id;

use bit::{Bit, BitList};
use instruction::Instruction;
use parameter::{FormalParameter, FormalParameterList, Parameter};
use qubit::{Qubit, QubitList};

pub struct QuantumCircuit {
    num_qubits: u32,
    num_bits: u32,
    num_parameters: u32,
    instructions: Box<[Instruction]>,
}

pub struct CircuitBuilder<'id> {
    id: Id<'id>,
    num_qubits: u32,
    num_bits: u32,
    num_parameters: u32,
    instructions: Vec<Instruction>,
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum QuantumCircuitError {

}

impl QuantumCircuit {
    #[inline]
    pub fn new<F>(init: F) -> Result<Self, QuantumCircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), QuantumCircuitError>
    {
        let mut builder = CircuitBuilder {
            id: Id::default(),
            num_qubits: 0,
            num_bits: 0,
            num_parameters: 0,
            instructions: vec![],
        };
        
        init(&mut builder)?;

        Ok(QuantumCircuit {
            num_qubits: builder.num_qubits, 
            num_bits: builder.num_bits, 
            num_parameters: builder.num_parameters, 
            instructions: builder.instructions.into_boxed_slice(), 
        })
    }
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub fn parameter(&mut self) -> FormalParameter<'id> {
        let res = FormalParameter::new(self.num_parameters);
        self.num_parameters += 1;
        res
    }

    #[inline]
    pub fn parameters<const N: usize>(&mut self) -> [FormalParameter<'id>; N] {
        [0; N].map(|_| self.parameter())
    }

    #[inline]
    pub fn parameter_list(&mut self, len: u32) -> FormalParameterList<'id> {
        let end = self.num_parameters + len;
        let res = FormalParameterList::new(self.num_parameters .. end);
        self.num_parameters = end;
        res
    }

    #[inline]
    pub fn parameter_count(&self) -> u32 {
        self.num_parameters
    }

    #[inline]
    pub fn qubit(&mut self) -> Qubit<'id> {
        let res = Qubit::new(self.num_qubits);
        self.num_qubits += 1;
        res
    }

    #[inline]
    pub fn qubits<const N: usize>(&mut self) -> [Qubit<'id>; N] {
        [0; N].map(|_| self.qubit())
    }

    #[inline]
    pub fn qubit_list(&mut self, len: u32) -> QubitList<'id> {
        let end = self.num_qubits + len;
        let res = QubitList::new(self.num_qubits .. end);
        self.num_qubits = end;
        res
    }

    #[inline]
    pub fn qubit_count(&self) -> u32 {
        self.num_qubits
    }

    #[inline]
    pub fn bit(&mut self) -> Bit<'id> {
        let res = Bit::new(self.num_bits);
        self.num_bits += 1;
        res
    }

    #[inline]
    pub fn bits<const N: usize>(&mut self) -> [Bit<'id>; N] {
        [0; N].map(|_| self.bit())
    }

    #[inline]
    pub fn bit_list(&mut self, len: u32) -> BitList<'id> {
        let end = self.num_bits + len;
        let res = BitList::new(self.num_bits .. end);
        self.num_bits = end;
        res
    }

    #[inline]
    pub fn bit_count(&self) -> u32 {
        self.num_bits
    }

    #[inline]
    pub fn h(&mut self, target: Qubit<'id>) {
        todo!()
    }

    #[inline]
    pub fn rx(&mut self, target: Qubit<'id>, angle: impl Into<Parameter<'id>>) {
        todo!()
    }

    #[inline]
    pub fn cx(&mut self, target: Qubit<'id>, control: Qubit<'id>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use this as a doctest with compile_fail maybe
    // #[test]
    // fn genericity() {
    //     fn try_unify<'a, T>(x: &'a T, b: &'a T) {}

    //     let qc1 = QuantumCircuit::new(|b1| {
    //         let qc2 = QuantumCircuit::new(|b2| {
    //             try_unify(b1, b2);

    //             Ok(())
    //         }).unwrap();

    //         Ok(())
    //     }).unwrap();
    // }

    #[test]
    fn bell() {
        let qc = QuantumCircuit::new(|b| {
            let [q1, q2] = b.qubits::<2>();

            b.h(q1);
            b.cx(q1, q2);

            Ok(())
        });
    }
}