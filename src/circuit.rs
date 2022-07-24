use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::genericity::Id;
use crate::instruction::{Instr, InstrIter, InstrVec};
use crate::provider::Provider;
use crate::symbol::{SymbolTuple, Symbol, Qubit, Ancillas};

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum CircuitError {
    #[error("quantum allocator overflow")]
    AllocOverflow,
}

#[derive(Clone, Default, Debug)]
pub struct QuantumCircuit {
    num_qubits: u32,
    num_ancillas: u32,
    num_bits: u32,
    num_params: u32,
    data: Vec<u32>,
}

impl QuantumCircuit {
    #[inline]
    pub fn new<F>(init: F) -> Result<Self, CircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), CircuitError>
    {
        Self::default().edit(init)
    }

    #[inline]
    pub fn edit<F>(self, edit: F) -> Result<Self, CircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), CircuitError>
    {
        let mut builder = CircuitBuilder::from_circ(self);
        edit(&mut builder).map(|_| builder.into_circ())
    }

    #[inline]
    pub fn num_qubit(&self) -> usize {
        self.num_qubits as usize
    }

    #[inline]
    pub fn num_ancillas(&self) -> usize {
        self.num_ancillas as usize
    }

    #[inline]
    pub fn num_parameters(&self) -> usize {
        self.num_params as usize
    }

    #[inline]
    pub fn num_bits(&self) -> usize {
        self.num_bits as usize
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.num_qubit() + self.num_ancillas()
    }

    #[inline]
    pub fn is_concrete(&self) -> bool {
        self.num_parameters() == 0
    }

    #[inline]
    pub fn bind(self, parameters: &[f32]) -> Option<ConcreteCircuit> {
        (parameters.len() == self.num_parameters()).then(|| {
            todo!()
        })
    }

    #[inline]
    pub fn bind_copy(&self, parameters: &[f32]) -> Option<ConcreteCircuit> {
        self.clone().bind(parameters)
    }
}

#[derive(Debug)]
pub struct CircuitBuilder<'id> {
    num_qubits: u32,
    num_ancillas: u32,
    num_bits: u32,
    num_params: u32,
    data: InstrVec<'id>,
}

impl<'id> Deref for CircuitBuilder<'id> {
    type Target = InstrVec<'id>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for CircuitBuilder<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'id> CircuitBuilder<'id> {
    /// Turns the quantum circuit into a circuit builder.
    #[inline]
    pub(crate) fn from_circ(circ: QuantumCircuit) -> Self {
        Self {
            num_qubits: circ.num_qubits,
            num_ancillas: circ.num_ancillas,
            num_bits: circ.num_bits,
            num_params: circ.num_params,
            data: InstrVec::new(circ.data),
        }
    }

    /// Turns the circuit builder into a quantum circuit.
    #[inline]
    pub(crate) fn into_circ(self) -> QuantumCircuit {
        QuantumCircuit {
            num_qubits: self.num_qubits,
            num_ancillas: self.num_ancillas,
            num_bits: self.num_bits,
            num_params: self.num_params,
            data: self.data.take(),
        }
    }

    /// Returns a mutable reference to the qubit count.
    #[inline]
    pub(crate) fn num_qubits_mut(&mut self) -> &mut u32 {
        &mut self.num_qubits
    }

    /// Returns a mutable reference to the parameter count.
    #[inline]
    pub(crate) fn num_params_mut(&mut self) -> &mut u32 {
        &mut self.num_params
    }

    /// Returns a mutable reference to the bit count.
    #[inline]
    pub(crate) fn num_bits_mut(&mut self) -> &mut u32 {
        &mut self.num_bits
    }

    #[inline]
    pub fn qubit_count(&self) -> usize {
        self.num_qubits as usize
    }

    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.num_params as usize
    }

    #[inline]
    pub fn bit_count(&self) -> usize {
        self.num_bits as usize
    }

    #[inline]
    pub fn ancilla_count(&self) -> usize {
        self.num_ancillas as usize
    }

    // #[inline]
    // pub fn set_ancilla_count(&mut self, value: usize) -> Result<(), CircuitError> {
    //     self.num_ancillas = value;
    // }

    #[inline]
    pub fn width(&self) -> usize {
        self.qubit_count() + self.ancilla_count()
    }

    #[inline]
    pub fn alloc<T: Symbol<'id>>(&mut self) -> Result<T, CircuitError> {
        T::alloc(self)
    }

    #[inline]
    pub fn alloc_n<T: Symbol<'id>, const N: usize>(&mut self) -> Result<[T; N], CircuitError> {
        T::alloc_n(self)
    }

    #[inline]
    pub fn alloc_tuple<T: SymbolTuple<'id>>(&mut self) -> Result<T, CircuitError> {
        T::alloc(self)
    }

    #[inline]
    pub fn instructions(&self) -> &InstrVec<'id> {
        &self.data
    }

    #[inline]
    pub fn instructions_mut(&mut self) -> &mut InstrVec<'id> {
        &mut self.data
    }
}

#[derive(Clone, Default, Debug)]
pub struct ConcreteCircuit {
    circ: QuantumCircuit,
}

impl ConcreteCircuit {
    /// Wraps the circuit in a concrete circuit type, to signify it is
    /// a concrete circuit. Sould obviously only be used after
    /// binding the input circuit's parameters.
    #[inline]
    fn new(circ: QuantumCircuit) -> Self {
        Self { circ }
    }

    #[inline]
    pub fn transpile<T: Provider>(self, config: &T::TranspileConfig) -> Result<TranspiledCircuit<T>, T::TranspileError> {
        let mut circ = self.take();
        let ancillas = Ancillas::new(&circ);
        circ.data = T::transpile(InstrVec::new(circ.data), ancillas, config)?.take();
        Ok(TranspiledCircuit::new(circ))
    }

    #[inline]
    pub fn transpile_copy<T: Provider>(&self, config: &T::TranspileConfig) -> Result<TranspiledCircuit<T>, T::TranspileError> {
        self.clone().transpile(config)
    }

    #[inline]
    pub fn take(self) -> QuantumCircuit {
        self.circ
    }
}

impl Deref for ConcreteCircuit {
    type Target = QuantumCircuit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

#[derive(Clone, Default, Debug)]
pub struct TranspiledCircuit<T: Provider> {
    _phantom: PhantomData<T>,
    circ: QuantumCircuit,
}

impl<T: Provider> Deref for TranspiledCircuit<T> {
    type Target = QuantumCircuit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

impl<T: Provider> TranspiledCircuit<T> {
    /// Wraps the circuit in a transpiled circuit type, to signify it has
    /// been transpiled for the provider T. Sould obviously only be used after
    /// transpiling the input circuit.
    #[inline]
    fn new(circ: QuantumCircuit) -> Self {
        Self { _phantom: PhantomData, circ }
    }

    #[inline]
    pub fn take(self) -> QuantumCircuit {
        self.circ
    }
}