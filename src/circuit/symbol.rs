use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;

use super::{QuantumCircuitError, CircuitBuilder};

#[doc(hidden)]
pub trait CircuitSymbolPrivate<'id>: Sized {
    fn new(n: u32) -> Self;

    #[inline]
    fn new_list(range: Range<u32>) -> List<Self> {
        List { range, _phantom: PhantomData }
    }

    fn count<'b>(b: &'b mut CircuitBuilder) -> &'b mut u32;
}

pub trait CircuitSymbol<'id>: CircuitSymbolPrivate<'id> {
    fn id(self) -> u32;
}

macro_rules! circuit_symbol_impl {
    { $(#[doc $($args :tt)*])* $name: ident $count: ident} => {
        $(#[doc $($args)*])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name<'id> {
            n: u32,
            id: Id<'id>,
        }

        impl<'id> CircuitSymbolPrivate<'id> for $name<'id> {
            #[inline]
            fn new(n: u32) -> Self {
                Self { n, id: Id::default() }
            }

            #[inline]
            fn count<'b>(b: &'b mut CircuitBuilder) -> &'b mut u32 {
                &mut b.$count
            }
        }

        impl<'id> CircuitSymbol<'id> for $name<'id> {
            #[inline]
            fn id(self) -> u32 {
                self.n
            }
        }
    }
}

circuit_symbol_impl! {
    /// TODO: Doc
    Bit 
    bit_count
}

circuit_symbol_impl! {
    /// TODO: Doc
    Qubit 
    qubit_count
}

circuit_symbol_impl! {
    /// TODO: Doc
    FormalParameter 
    parameter_count
}

pub struct List<T> {
    range: Range<u32>,
    _phantom: PhantomData<T>,
}

impl<'id, T: CircuitSymbol<'id> + 'id> List<T> {
    #[inline]
    pub fn range(&self) -> Range<u32> {
        self.range.clone()
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.range.end - self.range.start
    }

    #[inline]
    pub fn get(&self, id: u32) -> Option<T> {
        (id < self.len()).then(|| T::new(id + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: T) -> bool {
        self.range.contains(&parameter.id())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + 'id {
        self.range.clone().map(T::new)
    }
}