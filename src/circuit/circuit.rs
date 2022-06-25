use thiserror::Error;

use crate::genericity::Id;

use super::{FormalParameter, Instruction, Parameter, Qubit, List, CircuitSymbol};
use super::symbol::CircuitSymbolPrivate;

pub struct QuantumCircuit {
    qubit_count: u32,
    bit_count: u32,
    parameter_count: u32,
    instructions: Box<[Instruction]>,
}

pub struct CircuitBuilder<'id> {
    id: Id<'id>,
    pub(crate) qubit_count: u32,
    pub(crate) bit_count: u32,
    pub(crate) parameter_count: u32,
    instructions: Vec<Instruction>,
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

pub trait CircuitAllocatable<'id>: Sized {
    fn alloc(b: &mut CircuitBuilder) -> Self;
}

impl<'id, T: CircuitSymbol<'id>> CircuitAllocatable<'id> for T {
    #[inline]
    fn alloc(b: &mut CircuitBuilder) -> Self {
        let count = T::count(b);
        let res = T::new(*count);
        *count += 1;
        res
    }
}

impl<'id, const N: usize, T: CircuitSymbol<'id>> CircuitAllocatable<'id> for [T; N] {
    #[inline]
    fn alloc(b: &mut CircuitBuilder) -> Self {
        [0; N].map(|_| T::alloc(b))
    }
}

impl<'id> CircuitBuilder<'id> {
    #[inline]
    pub fn alloc<T: CircuitAllocatable<'id>>(&mut self) -> T {
        T::alloc(self)
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
    pub fn cx(&mut self, target: Qubit<'id>, control: Qubit<'id>) -> &mut Self {
        todo!()
    }

    #[inline]
    pub fn rx<P>(&mut self, target: Qubit<'id>, angle: P) -> &mut Self
    where
        P: Into<Parameter<'id>>,
    {
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
        let circ = QuantumCircuit::new(|circ| {
            let [q1, q2] = circ.alloc::<[Qubit; 2]>();

            circ.h(q1)
                .cx(q2, q1);

            Ok(())
        });
    }
}