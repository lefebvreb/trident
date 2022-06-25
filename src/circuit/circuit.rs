use thiserror::Error;

use crate::genericity::Id;

use super::{Instr, Parameter, Qubit, List, CircuitSymbol, Bit};

pub struct QuantumCircuit {
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
    instructions: Box<[Instr]>,
}

pub struct CircuitBuilder<'id> {
    pub(crate) qubit_count: u32,
    pub(crate) bit_count: u32,
    pub(crate) parameter_count: u32,
    instructions: Vec<Instr>,
    id: Id<'id>,
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

#[inline(always)]
fn checked_incr(a: &mut u32, n: usize) -> Result<(), QuantumCircuitError> {
    *a = a.checked_add(n.try_into().map_err(|_| QuantumCircuitError::AllocOverflow)?)
        .ok_or(QuantumCircuitError::AllocOverflow)?;
    Ok(())
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub fn alloc<T: CircuitSymbol<'id>>(&mut self) -> Result<T, QuantumCircuitError> {
        let count = T::count(self);
        let res = T::new(*count);
        checked_incr(count, 1).map(|_| res)
    }

    #[inline]
    pub fn alloc_n<T: CircuitSymbol<'id>, const N: usize>(&mut self) -> Result<[T; N], QuantumCircuitError> {
        let count = T::count(self);
        let mut start = *count;
        checked_incr(count, N).map(|_|
            [0; N].map(|_| {
                let res = T::new(start);
                start += 1;
                res
            })
        )
    }

    #[inline]
    pub fn alloc_list<T: CircuitSymbol<'id>>(&mut self, len: usize) -> Result<List<T>, QuantumCircuitError> {
        let count = T::count(self);
        let start = *count;
        checked_incr(count, len).map(|_| T::new_list(start..*count))
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