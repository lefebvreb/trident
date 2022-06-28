use thiserror::Error;

use crate::genericity::Id;

use super::instruction::Instr;
use super::parameter::Parameter;
use super::symbol::{SymbolTuple, Symbol, Qubit, Bit, List};

pub struct QuantumCircuit {
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
    instructions: Box<[Instr]>,
}

pub struct CircuitBuilder<'id> {
    _id: Id<'id>,
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
    instructions: Vec<Instr>,
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum QuantumCircuitError {
    #[error("quantum allocator overflow")]
    AllocOverflow,
}

impl QuantumCircuit {
    #[inline]
    pub fn new<F>(init: F) -> Result<Self, QuantumCircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), QuantumCircuitError>
    {
        let mut builder = CircuitBuilder {
            _id: Id::default(),
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

    #[inline]
    pub fn bit_count(&self) -> usize {
        self.bit_count as usize
    }

    #[inline]
    pub fn qubit_count(&self) -> usize {
        self.qubit_count as usize
    }

    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.parameter_count as usize
    }
}

impl<'id> CircuitBuilder<'id> {
    // Returns a mutable reference to the qubit count.
    #[inline]
    pub(crate) fn qubit_count_mut(&mut self) -> &mut u32 {
        &mut self.qubit_count
    }

    // Returns a mutable reference to the parameter count.
    #[inline]
    pub(crate) fn parameter_count_mut(&mut self) -> &mut u32 {
        &mut self.parameter_count
    }

    // Returns a mutable reference to the bit count.
    #[inline]
    pub(crate) fn bit_count_mut(&mut self) -> &mut u32 {
        &mut self.bit_count
    }

    #[inline]
    pub fn qubit_count(&self) -> usize {
        self.qubit_count as usize
    }

    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.parameter_count as usize
    }

    #[inline]
    pub fn bit_count(&self) -> usize {
        self.bit_count as usize
    }

    #[inline]
    pub fn alloc<T: Symbol<'id>>(&mut self) -> Result<T, QuantumCircuitError> {
        T::alloc(self)
    }

    #[inline]
    pub fn alloc_n<T: Symbol<'id>, const N: usize>(&mut self) -> Result<[T; N], QuantumCircuitError> {
        T::alloc_n(self)
    }

    #[inline]
    pub fn alloc_list<T: Symbol<'id>>(&mut self, len: usize) -> Result<List<T>, QuantumCircuitError> {
        T::alloc_list(self, len)
    }

    #[inline]
    pub fn alloc_tuple<T: SymbolTuple<'id>>(&mut self) -> Result<T, QuantumCircuitError> {
        T::alloc(self)
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
            let [q1, q2] = circ.alloc_n()?;

            circ.h(q1)
                .cx(q1, q2);

            Ok(())
        });

        assert!(circ.is_ok());
    }
}