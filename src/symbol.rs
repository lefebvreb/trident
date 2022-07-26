use std::ops::Range;

use crate::genericity::Id;
use crate::circuit::{CircuitBuilder, CircuitError};
use crate::prelude::QuantumCircuit;

pub trait Symbol<'id> {
    /// The maximum number of symbols of type `Self` that may be allocated in a single quantum circuit.
    const MAX: u32 = u32::MAX - 1;

    fn new_unchecked(n: u32) -> Self;
    
    fn id(self) -> u32;

    fn reserve(builder: &mut CircuitBuilder<'id>, n: usize) -> Result<u32, CircuitError>;
}

/// Used to define the different symbols types.
/// 
/// # Implementaion details
/// 
/// + $num is the name of the `CircuitBuilder`'s method that returns the current count of allocated symbols:
/// for parameters and bits that's their direct count. For qubits that's the `width`: ancillas and primary qubits.
/// + $counter is the name of the `CircuitBuilder`'s method used to get a mutable reference to the number of allocated
/// symbols. For qubits, this does not include ancillas.
macro_rules! symbols {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident {
                num: $num: ident,
                counter: $counter: ident,
                $(max: $max: expr,)?
            }
        )*
    } => {
        $(
            $(#[doc$($args)*])*
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            pub struct $name<'id> {
                _id: Id<'id>,
                n: u32,
            }
    
            impl<'id> Symbol<'id> for $name<'id> {
                $(const MAX: u32 = $max;)?

                fn new_unchecked(n: u32) -> Self {
                    Self { n, _id: Id::default() }
                }

                fn id(self) -> u32 {
                    self.n
                }

                fn reserve(builder: &mut CircuitBuilder<'id>, n: usize) -> Result<u32, CircuitError> {
                    (n < Self::MAX as usize - builder.$num()).then(|| {
                        let val = *builder.$counter();
                        *builder.$counter() += n as u32;
                        val
                    })
                    .ok_or(CircuitError::AllocOverflow)
                }
            }
        )*
    }
}

symbols! {
    /// TODO: Doc
    Qubit {
        num: width,
        counter: num_qubits_mut,
    }

    /// TODO: Doc
    FormalParameter {
        num: num_parameters,
        counter: num_params_mut,
    }

    /// TODO: Doc
    Bit {
        num: num_bits,
        counter: num_bits_mut,
        max: 1 << f32::MANTISSA_DIGITS,
    }
}

pub trait SymbolTuple<'id>: Sized {
    fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitError>;
}

macro_rules! peel {
    { $name: ident, $($rest: ident,)* } => { tuple!{ $($rest,)* } }
}

macro_rules! tuple {
    {} => {};
    { $($name: ident,)+ } => {
        impl<'id, $($name,)+> SymbolTuple<'id> for ($($name,)+) where $($name: Symbol<'id>,)* {
            fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitError> {
                Ok(($(builder.alloc::<$name>()?,)+))
            }
        }

        peel! { $($name,)+ }
    };
}

tuple! { A, B, C, D, E, F, G, H, I, J, K, L, }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ancillas<'id> {
    _id: Id<'id>,
    range: Range<u32>,
}

impl<'id> Ancillas<'id> {
    /// Returns an ancillas list from a circuit's parameters.
    pub(crate) fn new(circ: &QuantumCircuit) -> Option<Self> {
        (circ.num_ancillas() > 0).then(|| {
            let start = circ.num_qubit() as u32;
            let end = start + circ.num_ancillas() as u32;
            Self {
                _id: Id::default(),
                range: start..end,
            }
        })
    }

    pub fn count(&self) -> usize {
        self.range.len()
    }

    pub fn get(&self, n: usize) -> Option<Qubit<'id>> {
        (n < self.count()).then(|| Qubit::new_unchecked(n as u32 + self.range.start))
    }

    pub fn contains(&self, qubit: Qubit<'id>) -> bool {
        self.range.contains(&qubit.id())
    }

    pub fn iter(&self) -> AncillasIter<'id> {
        AncillasIter { ancillas: self.clone() }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AncillasIter<'id> {
    ancillas: Ancillas<'id>,
}

impl<'id> Iterator for AncillasIter<'id> {
    type Item = Qubit<'id>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ancillas.range.next().map(Qubit::new_unchecked)
    }
}

impl<'id> IntoIterator for Ancillas<'id> {
    type Item = Qubit<'id>;

    type IntoIter = AncillasIter<'id>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}