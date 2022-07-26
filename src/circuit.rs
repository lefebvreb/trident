use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::instruction::InstrVec;
use crate::provider::Architecture;
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
    pub fn new<F>(init: F) -> Result<Self, CircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), CircuitError>
    {
        Self::default().edit(init)
    }

    pub fn edit<F>(self, edit: F) -> Result<Self, CircuitError>
    where
        F: for<'id> FnOnce(&mut CircuitBuilder<'id>) -> Result<(), CircuitError>
    {
        let mut builder = CircuitBuilder::from_circ(self);
        edit(&mut builder).map(|_| builder.into_circ())
    }

    pub fn num_qubit(&self) -> usize {
        self.num_qubits as usize
    }

    pub fn num_ancillas(&self) -> usize {
        self.num_ancillas as usize
    }

    pub fn num_parameters(&self) -> usize {
        self.num_params as usize
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits as usize
    }

    pub fn width(&self) -> usize {
        self.num_qubit() + self.num_ancillas()
    }

    pub fn is_concrete(&self) -> bool {
        self.num_parameters() == 0
    }

    pub fn bind(self, parameters: &[f32]) -> Option<ConcreteCircuit> {
        (parameters.len() == self.num_parameters()).then(|| {
            todo!() // TODO: implement this somehow.
        })
    }

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

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for CircuitBuilder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'id> CircuitBuilder<'id> {
    /// Turns the quantum circuit into a circuit builder.
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
    pub(crate) fn num_qubits_mut(&mut self) -> &mut u32 {
        &mut self.num_qubits
    }

    /// Returns a mutable reference to the parameter count.
    pub(crate) fn num_params_mut(&mut self) -> &mut u32 {
        &mut self.num_params
    }

    /// Returns a mutable reference to the bit count.
    pub(crate) fn num_bits_mut(&mut self) -> &mut u32 {
        &mut self.num_bits
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits as usize
    }

    pub fn num_ancillas(&self) -> usize {
        self.num_ancillas as usize
    }

    pub fn num_parameters(&self) -> usize {
        self.num_params as usize
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits as usize
    }

    pub fn width(&self) -> usize {
        self.num_qubits() + self.num_ancillas()
    }

    pub fn set_num_ancillas(&mut self, n: usize) -> Result<(), CircuitError> {
        (n < Qubit::MAX as usize - self.width())
            .then(|| self.num_ancillas += n as u32)
            .ok_or(CircuitError::AllocOverflow)
    }

    pub fn alloc<T: Symbol<'id>>(&mut self) -> Result<T, CircuitError> {
        T::reserve(self, 1).map(T::new_unchecked)
    }

    pub fn alloc_n<T: Symbol<'id>, const N: usize>(&mut self) -> Result<[T; N], CircuitError> {
        T::reserve(self, N).map(|mut n| {
            [(); N].map(|_| {
                let sym = T::new_unchecked(n);
                n += 1;
                sym
            })
        })
    }

    pub fn alloc_tuple<T: SymbolTuple<'id>>(&mut self) -> Result<T, CircuitError> {
        T::alloc(self)
    }

    pub fn instructions(&self) -> &InstrVec<'id> {
        &self.data
    }

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
    fn new(circ: QuantumCircuit) -> Self {
        Self { circ }
    }

    pub fn transpile<'arch, T: Architecture>(self, backend: &T) -> Result<TranspiledCircuit<T>, T::TranspileError> {
        let mut circ = self.take();
        let ancillas = Ancillas::new(&circ);
        circ.data = backend.transpile(InstrVec::new(circ.data), ancillas)?.take();
        Ok(TranspiledCircuit::new(circ))
    }

    pub fn transpile_copy<T: Architecture>(&self, backend: &T) -> Result<TranspiledCircuit<T>, T::TranspileError> {
        self.clone().transpile(backend)
    }

    pub fn take(self) -> QuantumCircuit {
        self.circ
    }
}

impl Deref for ConcreteCircuit {
    type Target = QuantumCircuit;

    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

#[derive(Clone, Debug)]
pub struct TranspiledCircuit<T: Architecture> {
    _phantom: PhantomData<T>,
    circ: QuantumCircuit,
}

impl<T: Architecture> Deref for TranspiledCircuit<T> {
    type Target = QuantumCircuit;

    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

impl<'arch, T: Architecture> TranspiledCircuit<T> {
    /// Wraps the circuit in a transpiled circuit type, to signify it has
    /// been transpiled for the provider T. Sould obviously only be used after
    /// transpiling the input circuit.
    fn new(circ: QuantumCircuit) -> Self {
        Self { _phantom: PhantomData, circ }
    }

    pub fn take(self) -> QuantumCircuit {
        self.circ
    }
}