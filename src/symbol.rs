use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;
use crate::circuit::{CircuitBuilder, CircuitError};
use crate::prelude::QuantumCircuit;

pub trait Symbol<'id>: Copy + Eq + Ord + Sized + 'id {
    /// The maximum number of symbols of type `Self` that may be allocated in a single quantum circuit.
    const MAX: u32 = u32::MAX - 1;

    fn new_unchecked(n: u32) -> Self;
    
    fn id(self) -> u32;

    fn num<'circ>(circ: &'circ mut CircuitBuilder<'id>) -> &'circ mut u32;

    #[inline]
    #[doc(hidden)]
    fn checked_incr(a: &mut u32, n: usize) -> Result<(), CircuitError> {
        (n < (Self::MAX - *a) as usize)
            .then(|| *a += n as u32)
            .ok_or(CircuitError::AllocOverflow)
    }

    #[inline]
    fn alloc(circ: &mut CircuitBuilder<'id>) -> Result<Self, CircuitError> {
        let count = Self::num(circ);
        let res = Self::new_unchecked(*count);
        Self::checked_incr(count, 1).map(|_| res)
    }

    #[inline]
    fn alloc_n<const N: usize>(circ: &mut CircuitBuilder<'id>) -> Result<[Self; N], CircuitError> {
        let count = Self::num(circ);
        let mut start = *count;
        Self::checked_incr(count, N).map(|_|
            [0; N].map(|_| {
                let res = Self::new_unchecked(start);
                start += 1;
                res
            })
        )
    }
}

/// Used to define the different symbols types.
macro_rules! symbols {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident {
                num: $num: ident,
                $(max: $max: expr,)?
            }
        )*
    } => {
        $(
            $(#[doc$($args)*])*
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            pub struct $name<'id> {
                n: u32,
                id: Id<'id>,
            }
    
            impl<'id> Symbol<'id> for $name<'id> {
                $(const MAX: u32 = $max;)?

                #[inline]
                fn new_unchecked(n: u32) -> Self {
                    Self { n, id: Id::default() }
                }

                #[inline]
                fn id(self) -> u32 {
                    self.n
                }
                
                #[inline]
                fn num<'b>(circ: &'b mut CircuitBuilder) -> &'b mut u32 {
                    circ.$num()
                }
            }
        )*
    }
}

symbols! {
    /// TODO: Doc
    Qubit {
        num: num_qubits_mut,
    }

    /// TODO: Doc
    FormalParameter {
        num: num_params_mut,
    }

    /// TODO: Doc
    Bit {
        num: num_bits_mut,
        max: 1 << f32::MANTISSA_DIGITS,
    }
}

pub trait SymbolTuple<'id>: Sized {
    fn alloc(b: &mut CircuitBuilder<'id>) -> Result<Self, CircuitError>;
}

macro_rules! peel {
    { $name: ident, $($rest: ident,)* } => { tuple!{ $($rest,)* } }
}

macro_rules! tuple {
    {} => {};
    { $($name: ident,)+ } => {
        impl<'id, $($name,)+> SymbolTuple<'id> for ($($name,)+) where $($name: Symbol<'id>,)* {
            #[inline]
            fn alloc(b: &mut CircuitBuilder<'id>) -> Result<Self, CircuitError> {
                Ok(($(b.alloc::<$name>()?,)+))
            }
        }

        peel! { $($name,)+ }
    }
}

tuple! { A, B, C, D, E, F, G, H, I, J, K, L, }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ancillas<'id> {
    _id: Id<'id>,
    range: Range<u32>,
}

impl<'id> Ancillas<'id> {
    /// Returns an ancillas list from a circuit's parameters.
    #[inline]
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

    #[inline]
    pub fn count(&self) -> usize {
        self.range.len()
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<Qubit<'id>> {
        (n < self.count()).then(|| Qubit::new_unchecked(n as u32 + self.range.start))
    }

    #[inline]
    pub fn contains(&self, qubit: Qubit<'id>) -> bool {
        self.range.contains(&qubit.id())
    }

    #[inline]
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.ancillas.range.next().map(Qubit::new_unchecked)
    }
}

impl<'id> IntoIterator for Ancillas<'id> {
    type Item = Qubit<'id>;

    type IntoIter = AncillasIter<'id>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}