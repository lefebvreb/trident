use thiserror::Error;

use crate::genericity::Id;

use super::{FormalParameter, Instruction, Parameter, Qubit, List, CircuitSymbol, Bit};
use super::symbol::CircuitSymbolPrivate;

pub struct QuantumCircuit {
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
    instructions: Box<[Instruction]>,
}

pub struct CircuitBuilder<'id> {
    pub(crate) qubit_count: u32,
    pub(crate) bit_count: u32,
    pub(crate) parameter_count: u32,
    instructions: Vec<Instruction>,
    id: Id<'id>,
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum QuantumCircuitError {

}

impl QuantumCircuit {
    pub fn new<F>(init: F) -> Result<Self, QuantumCircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), QuantumCircuitError>
    {
        let mut builder = CircuitBuilder {
            id: Id::default(),
            qubit_count: 0,
            bit_count: 0,
            parameter_count: 0,
            instructions: vec![],
        };
        
        init(&mut builder)?;

        Ok(QuantumCircuit {
            qubit_count: builder.qubit_count, 
            bit_count: builder.bit_count, 
            parameter_count: builder.parameter_count, 
            instructions: builder.instructions.into_boxed_slice(), 
        })
    }
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub fn alloc<T: CircuitSymbol<'id>>(&mut self) -> T {
        let count = T::count(self);
        let res = T::new(*count);
        *count += 1;
        res
    }

    #[inline]
    pub fn alloc_n<T: CircuitSymbol<'id>, const N: usize>(&mut self) -> [T; N] {
        [0; N].map(|_| self.alloc())
    }

    #[inline]
    pub fn alloc_list<T: CircuitSymbol<'id>>(&mut self, len: u32) -> List<T> {
        let count = T::count(self);
        let start = *count;
        *count += len;
        T::new_list(start..*count)
    }

    #[inline]
    pub fn bit_count(&self) -> u32 {
        self.bit_count
    }

    #[inline]
    pub fn qubit_count(&self) -> u32 {
        self.qubit_count
    }

    #[inline]
    pub fn parameter_count(&self) -> u32 {
        self.parameter_count
    }

    #[inline]
    pub fn h(&mut self, target: Qubit<'id>) -> &mut Self {
        todo!()
    }

    #[inline]
    pub fn cx(&mut self, control: Qubit<'id>, target: Qubit<'id>) -> &mut Self {
        todo!()
    }

    #[inline]
    pub fn rx<P>(&mut self, target: Qubit<'id>, angle: P) -> &mut Self
    where
        P: Into<Parameter<'id>>,
    {
        todo!()
    }

    #[inline]
    pub fn measure(&mut self, target: Qubit<'id>, result: Bit<'id>) -> &mut Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bell() {
        let circ = QuantumCircuit::new(|circ| {
            let [q1, q2] = circ.alloc_n();

            circ.h(q1)
                .cx(q1, q2);

            Ok(())
        });
    }
}