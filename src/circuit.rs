use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::instruction::InstrVec;
use crate::provider::Architecture;
use crate::symbol::{SymbolTuple, Symbol, Qubit, Ancillas, Bit, FormalParameter};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
pub enum CircuitError {
    #[error("quantum allocator overflow")]
    AllocOverflow,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
#[error("quantum allocator overflow")]
pub struct CircuitAllocOverflow;

#[derive(Clone, Default, Debug)]
pub struct QuantumCircuit {
    num_qubits: u32,
    num_bits: u32,
    num_formals: u32,
    num_ancillas: u32,
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

    pub fn num_formals(&self) -> usize {
        self.num_formals as usize
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits as usize
    }

    pub fn width(&self) -> usize {
        self.num_qubit() + self.num_ancillas()
    }

    pub fn num_ancillas(&self) -> usize {
        self.num_ancillas as usize
    }

    pub fn bind(self, parameters: &[f32]) -> Option<ConcreteCircuit> {
        (parameters.len() == self.num_formals()).then(|| {
            todo!() // TODO: implement this somehow.
        })
    }

    pub fn bind_copy(&self, parameters: &[f32]) -> Option<ConcreteCircuit> {
        self.clone().bind(parameters)
    }

    pub fn is_concrete(&self) -> bool {
        self.num_formals() == 0
    }

    pub fn as_concrete(self) -> Option<ConcreteCircuit> {
        self.try_into().ok()
    }
}

#[derive(Debug)]
pub struct CircuitBuilder<'id> {
    num_qubits: u32,
    num_bits: u32,
    num_formals: u32,
    num_ancillas: u32,
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

/// Increments the value at `val` by 1, and returns it's previous value
#[inline(always)]
fn incr(val: &mut u32) -> u32 {
    let prev = *val;
    *val += 1;
    prev
}

impl<'id> CircuitBuilder<'id> {
    /// Turns the quantum circuit into a circuit builder.
    pub(crate) fn from_circ(circ: QuantumCircuit) -> Self {
        Self {
            num_qubits: circ.num_qubits,
            num_ancillas: circ.num_ancillas,
            num_bits: circ.num_bits,
            num_formals: circ.num_formals,
            data: InstrVec::new(circ.data),
        }
    }

    /// Turns the circuit builder into a quantum circuit.
    pub(crate) fn into_circ(self) -> QuantumCircuit {
        QuantumCircuit {
            num_qubits: self.num_qubits,
            num_ancillas: self.num_ancillas,
            num_bits: self.num_bits,
            num_formals: self.num_formals,
            data: self.data.take(),
        }
    }

    /// Returns a mutable reference to the qubit count.
    pub(crate) fn num_qubits_mut(&mut self) -> &mut u32 {
        &mut self.num_qubits
    }

    /// Returns a mutable reference to the parameter count.
    pub(crate) fn num_params_mut(&mut self) -> &mut u32 {
        &mut self.num_formals
    }

    /// Returns a mutable reference to the bit count.
    pub(crate) fn num_bits_mut(&mut self) -> &mut u32 {
        &mut self.num_bits
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits as usize
    }

    pub fn qubit(&mut self) -> Result<Qubit<'id>, CircuitAllocOverflow> {
        (1 < Qubit::MAX as usize - self.width())
            .then(|| Qubit::new_unchecked(incr(&mut self.num_bits)))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn qubits<const N: usize>(&mut self) -> Result<[Qubit<'id>; N], CircuitAllocOverflow> {
        (N < Qubit::MAX as usize - self.num_bits())
            .then(|| [(); N].map(|_| Qubit::new_unchecked(incr(&mut self.num_bits))))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn num_formals(&self) -> usize {
        self.num_formals as usize
    }

    pub fn formal(&mut self) -> Result<FormalParameter<'id>, CircuitAllocOverflow> {
        (1 < FormalParameter::MAX as usize - self.num_formals())
            .then(|| FormalParameter::new_unchecked(incr(&mut self.num_bits)))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn formals<const N: usize>(&mut self) -> Result<[FormalParameter<'id>; N], CircuitAllocOverflow> {
        (N < FormalParameter::MAX as usize - self.num_formals())
            .then(|| [(); N].map(|_| FormalParameter::new_unchecked(incr(&mut self.num_bits))))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits as usize
    }

    pub fn bit(&mut self) -> Result<Bit<'id>, CircuitAllocOverflow> {
        (1 < Bit::MAX as usize - self.num_bits())
            .then(|| Bit::new_unchecked(incr(&mut self.num_bits)))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn bits<const N: usize>(&mut self) -> Result<[Bit<'id>; N], CircuitAllocOverflow> {
        (N < Bit::MAX as usize - self.num_bits())
            .then(|| [(); N].map(|_| Bit::new_unchecked(incr(&mut self.num_bits))))
            .ok_or(CircuitAllocOverflow)
    }

    pub fn num_ancillas(&self) -> usize {
        self.num_ancillas as usize
    }

    pub fn set_num_ancillas(&mut self, n: usize) -> Result<(), CircuitAllocOverflow> {
        (n < (Qubit::MAX - self.num_qubits) as usize)
            .then(|| self.num_ancillas = n as u32)
            .ok_or(CircuitAllocOverflow)
    }

    pub fn width(&self) -> usize {
        self.num_qubits() + self.num_ancillas()
    }

    pub fn alloc<T: SymbolTuple<'id>>(&mut self) -> Result<T, CircuitAllocOverflow> {
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
#[error("circuit is not concrete, it has at least one formal parameter")]
pub struct NotConcreteError;

impl TryFrom<QuantumCircuit> for ConcreteCircuit {
    type Error = NotConcreteError;

    fn try_from(circ: QuantumCircuit) -> Result<Self, Self::Error> {
        circ.is_concrete().then(|| ConcreteCircuit::new(circ)).ok_or(NotConcreteError)
    }
}

#[derive(Clone, Debug)]
pub struct TranspiledCircuit<T: Architecture> {
    _phantom: PhantomData<T>,
    circ: QuantumCircuit,
}

impl<T: Architecture> TranspiledCircuit<T> {
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

impl<T: Architecture> Deref for TranspiledCircuit<T> {
    type Target = QuantumCircuit;

    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}