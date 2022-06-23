use thiserror::Error;

use crate::bounded::{FormalParameter, FormalParameterList};
use crate::genericity::Id;
use crate::instruction::Instruction;

pub struct QuantumCircuit {
    num_qubits: u32,
    num_cbits: u32,
    num_parameters: u32,
    instructions: Box<[Instruction]>,
}

pub struct CircuitBuilder<'id> {
    id: Id<'id>,
    num_qubits: u32,
    num_cbits: u32,
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
            num_cbits: 0,
            num_parameters: 0,
            instructions: vec![],
        };
        
        init(&mut builder)?;

        Ok(QuantumCircuit {
            num_qubits: builder.num_qubits, 
            num_cbits: builder.num_cbits, 
            num_parameters: builder.num_parameters, 
            instructions: builder.instructions.into_boxed_slice(), 
        })
    }
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub parameter(&mut self) -> FormalParameter<'id> {
        let res = FormalParameter::new(self.num_parameters);
        self.num_parameters += 1;
        res
    }

    #[inline]
    pub parameter_list(&mut self, len: u32) -> FormalParameter<'id> {
        let end = self.num_parameters + len;
        let res = FormalParameterList::new(self.num_parameters .. end);
        self.num_parameters = end;
        res
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
}