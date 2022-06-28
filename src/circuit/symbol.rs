use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;

use super::{CircuitBuilder, QuantumCircuitError};

pub(crate) mod private {
    use crate::circuit::CircuitBuilder;

    #[doc(hidden)]
    pub trait SymbolPrivate<'id> {
        fn new(n: u32) -> Self;

        fn count<'a>(circ: &'a mut CircuitBuilder<'id>) -> &'a mut u32;
    }
}

#[inline(always)]
fn checked_incr(a: &mut u32, n: usize) -> Result<(), QuantumCircuitError> {
    n.try_into().ok()
        .and_then(|n| a.checked_add(n))
        .map(|n| *a = n)
        .ok_or(QuantumCircuitError::AllocOverflow)
}

pub trait Symbol<'id>: Sized + private::SymbolPrivate<'id> + 'id {
    fn id(self) -> u32;

    #[inline]
    fn alloc(b: &mut CircuitBuilder<'id>) -> Result<Self, QuantumCircuitError> {
        let count = Self::count(b);
        let res = Self::new(*count);
        checked_incr(count, 1).map(|_| res)
    }

    #[inline]
    fn alloc_n<const N: usize>(b: &mut CircuitBuilder<'id>) -> Result<[Self; N], QuantumCircuitError> {
        let count = Self::count(b);
        let mut start = *count;
        checked_incr(count, N).map(|_|
            [0; N].map(|_| {
                let res = Self::new(start);
                start += 1;
                res
            })
        )
    }

    #[inline]
    fn alloc_list(b: &mut CircuitBuilder<'id>, len: usize) -> Result<List<Self>, QuantumCircuitError> {
        let count = Self::count(b);
        let start = *count;
        checked_incr(count, len).map(|_| List::new(start..*count))
    }
}

macro_rules! circuit_symbol_impl {
    { 
        $(#[doc$($args :tt)*])* 
        $name: ident,
        count: $count: ident
    } => {
        $(#[doc$($args)*])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name<'id> {
            n: u32,
            id: Id<'id>,
        }

        impl<'id> private::SymbolPrivate<'id> for $name<'id> {
            #[inline]
            fn new(n: u32) -> Self {
                Self { n, id: Id::default() }
            }
            
            #[inline]
            fn count<'b>(circ: &'b mut CircuitBuilder) -> &'b mut u32 {
                circ.$count()
            }
        }

        impl<'id> Symbol<'id> for $name<'id> {
            #[inline]
            fn id(self) -> u32 {
                self.n
            }
        }
    }
}

circuit_symbol_impl! {
    /// TODO: Doc
    Qubit,
    count: qubit_count_mut
}

circuit_symbol_impl! {
    /// TODO: Doc
    FormalParameter,
    count: parameter_count_mut
}

circuit_symbol_impl! {
    /// TODO: Doc
    Bit,
    count: bit_count_mut
}

pub struct List<T> {
    range: Range<u32>,
    _phantom: PhantomData<T>,
}

impl<'id, T: Symbol<'id> + 'id> List<T> {
    #[inline]
    fn new(range: Range<u32>) -> Self {
        List { range, _phantom: PhantomData }
    }

    #[inline]
    pub fn range(&self) -> Range<u32> {
        self.range.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }

    #[inline]
    pub fn get(&self, id: usize) -> Option<T> {
        (id < self.len()).then(|| T::new(id as u32 + self.range.start))
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

pub trait SymbolTuple<'id>: Sized {
    fn alloc(b: &mut CircuitBuilder<'id>) -> Result<Self, QuantumCircuitError>;
}

macro_rules! peel {
    { $name: ident, $($rest: ident,)* } => { tuple!{ $($rest,)* } }
}

macro_rules! tuple {
    {} => {};
    { $($name: ident,)+ } => {
        impl<'id, $($name,)+> SymbolTuple<'id> for ($($name,)+) where $($name: Symbol<'id>,)* {
            #[inline]
            fn alloc(b: &mut CircuitBuilder<'id>) -> Result<Self, QuantumCircuitError> {
                Ok(($(b.alloc::<$name>()?,)+))
            }
        }

        peel! { $($name,)+ }
    }
}

tuple! { A, B, C, D, E, F, G, H, I, J, K, L, }