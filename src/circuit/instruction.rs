use std::io::{Read, Write};
use std::ops::Deref;
use std::rc::Rc;

use crate::genericity::Id;

use super::{QuantumCircuit, parameter};
use super::parameter::Parameter;
use super::symbol::{Qubit, Bit};

macro_rules! operations {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident {
                qubits: $qubits: expr,
                parameters: $parameters: expr,
                bits: $bits: expr,
                unitary: $unitary: literal,
                label: $label: literal,
            },
        )* 
    } => {
        #[repr(u32)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum OperationKind {
            $(
                $(#[doc$($args)*])*
                $name,
            )*
        }

        use OperationKind::*;
        
        impl OperationKind {
            pub const ALL_OPERATIONS: &'static [Self] = &[$($name,)*];

            #[inline]
            pub fn qubit_count(self) -> Option<usize> {
                Some(match self {
                    $($name => $qubits,)*
                })
            }

            #[inline]
            pub fn parameter_count(self) -> Option<usize> {
                Some(match self {
                    $($name => $parameters,)*
                })
            }

            #[inline]
            pub fn bit_count(self) -> Option<usize> {
                Some(match self {
                    $($name => $bits,)*
                })
            }

            #[inline]
            pub fn is_unitary(self) -> bool {
                match self {
                    $($name => $unitary,)*
                }
            }

            #[inline]
            pub fn label(self) -> &'static str {
                match self {
                    $($name => $label,)*
                }
            }
        }

    }
}

macro_rules! variadic {
    () => { None? }
}

operations! {
    /// No-operation, or nop.
    Nop {
        qubits: 0,
        parameters: 0,
        bits: 0,
        unitary: false,
        label: "nop",
    },
    /// Hadamard gate.
    H {
        qubits: 1,
        parameters: 0,
        bits: 0,
        unitary: true,
        label: "h",
    },
    /// Pauli-X gate, or NOT gate.
    X {
        qubits: 1,
        parameters: 0,
        bits: 0,
        unitary: true,
        label: "x",
    },
    /// Rotation about the X-axis
    Rx {
        qubits: 1,
        parameters: 1,
        bits: 0,
        unitary: true,
        label: "rx",
    },
    /// Barrier, a non-unitary operator, variadic over qubits. 
    /// Prevents optimizations between it's left and right sides.
    Barrier {
        qubits: variadic!(),
        parameters: 0,
        bits: 0,
        unitary: false,
        label: "barrier",
    },
    Measure {
        qubits: 1,
        parameters: 0,
        bits: 1,
        unitary: false,
        label: "measure",
    },
}

impl Default for OperationKind {
    #[inline]
    fn default() -> Self {
        Nop
    }
}

pub enum InstructionKind<'id> {
    Operation {
        kind: OperationKind,
        controls: Option<&'id [Qubit<'id>]>,
    },
    Composite(Rc<QuantumCircuit>),
}

impl Default for InstructionKind<'_> {
    #[inline]
    fn default() -> Self {
        Self::Operation { kind: Nop, controls: None }
    }
}

pub enum Modifier {
    Conditional,
    WhileLoop,
    ForLoop,
    Repeat,
}

#[derive(Default)]
pub struct Instruction<'id> {
    kind: InstructionKind<'id>,
    modifier: Option<Modifier>,
    targets: &'id [Qubit<'id>],
    parameters: &'id [Parameter<'id>],
    bits: &'id [Bit<'id>],
}

impl<'id> Instruction<'id> {
    #[inline]
    pub fn is_operation(&self) -> bool {
        matches!(self.kind, InstructionKind::Operation { .. })
    }

    #[inline]
    pub fn into_operation(&self) -> Option<OperationKind> {
        match self.kind {
            InstructionKind::Operation { kind, .. } => Some(kind),
            _ => None,
        }
    }

    #[inline]
    pub fn is_composite(&self) -> bool {
        matches!(self.kind, InstructionKind::Composite(_))
    }

    #[inline]
    pub fn into_composite(&self) -> Option<&QuantumCircuit> {
        match self.kind {
            InstructionKind::Composite(ref circ) => Some(circ),
            _ => None,
        }
    }

    #[inline]
    pub fn targets(&self) -> &[Qubit<'id>] {
        self.targets
    }

    #[inline]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    #[inline]
    pub fn has_controls(&self) -> bool {
        matches!(self.kind, InstructionKind::Operation { controls: Some(_), .. })
    }

    #[inline]
    pub fn controls(&self) -> Option<&[Qubit<'id>]> {
        match self.kind {
            InstructionKind::Operation { controls, .. } => controls,
            _ => None,
        }
    }

    #[inline]
    pub fn parameters(&self) -> &[Parameter<'id>] {
        self.parameters
    }

    #[inline]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    #[inline]
    pub fn bits(&self) -> &[Bit<'id>] {
        self.bits
    }

    #[inline]
    pub fn bit_count(&self) -> usize {
        self.bits.len()
    }

    #[inline]
    pub fn has_modifier(&self) -> bool {
        self.modifier.is_some()
    }

    #[inline]
    pub fn modifier(&self) -> Option<&Modifier> {
        self.modifier.as_ref()
    }
}

pub struct InstructionRope;

pub(crate) struct RopeToken(usize);

impl InstructionRope {
    #[inline]
    pub(crate) fn push_composite(&mut self, circ: Rc<QuantumCircuit>, targets: &[Qubit], parameters: &[Qubit], bits: &[Bit]) -> RopeToken {
        todo!()
    }

    #[inline]
    pub(crate) fn push_controls(&mut self, token: RopeToken, controls: &[Qubit]) -> RopeToken {
        todo!()
    }

    #[inline]
    pub(crate) fn push_modifier(&mut self, token: RopeToken, modifier: Modifier) {
        todo!()
    }

    #[inline]
    pub(crate) fn iter<'any>(&mut self) -> InstructionIter<'any> {
        todo!()
    }
}

pub struct InstructionIter<'id> {
    _id: Id<'id>
}

impl<'id> Iterator for InstructionIter<'id> {
    type Item = &'id Instruction<'id>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}