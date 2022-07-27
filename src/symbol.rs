use std::ops::Range;

use crate::genericity::Id;
use crate::circuit::{CircuitBuilder, CircuitError, CircuitAllocOverflow};
use crate::prelude::QuantumCircuit;

pub trait Symbol<'id>: Sized {
    fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitAllocOverflow>;
}

/// Used to define the different symbols types.
macro_rules! symbols {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident {
                max: $max: expr,
                alloc: $alloc: ident,
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

            impl $name<'_> {
                /// The maximum number of symbols of type `Self` that may be allocated in a single quantum circuit.
                pub const MAX: u32 = $max;

                pub fn new_unchecked(n: u32) -> Self {
                    Self { n, _id: Id::default() }
                }

                pub fn id(self) -> u32 {
                    self.n
                }
            }
    
            impl<'id> Symbol<'id> for $name<'id> {
                fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitAllocOverflow> {
                    builder.$alloc()
                }
            }
        )*
    }
}

symbols! {
    Qubit {
        max: u32::MAX - 1,
        alloc: qubit,
    }

    FormalParameter {
        max: u32::MAX - 1,
        alloc: formal,
    }

    Bit {
        max: 1 << f32::MANTISSA_DIGITS,
        alloc: bit,
    }
}

pub trait SymbolTuple<'id>: Sized {
    fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitAllocOverflow>;
}

macro_rules! peel {
    { $name: ident, $($rest: ident,)* } => { tuple!{ $($rest,)* } }
}

macro_rules! tuple {
    {} => {};
    { $($name: ident,)+ } => {
        impl<'id, $($name,)+> SymbolTuple<'id> for ($($name,)+) where $($name: Symbol<'id>,)* {
            fn alloc(builder: &mut CircuitBuilder<'id>) -> Result<Self, CircuitAllocOverflow> {
                Ok(($($name::alloc(builder)?,)+))
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