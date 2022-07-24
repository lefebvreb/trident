use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;
use crate::circuit::{CircuitBuilder, QuantumCircuitError};

pub trait Symbol<'id>: Copy + Eq + Ord + Sized + 'id {
    /// The maximum number of symbols of type `Self` that may be allocated in a single quantum circuit.
    const MAX: u32 = u32::MAX - 1;

    fn new_unchecked(n: u32) -> Self;
    
    fn id(self) -> u32;

    fn count<'circ>(circ: &'circ mut CircuitBuilder<'id>) -> &'circ mut u32;

    /// Increases the borrowed integer `a` with the value `n` if `*a + n < Self::MAX`.
    /// Else, returns a `Err(QuantumCircuitError::AllocOverflow)`.
    /// This is used when allocating symbols of type `Self`.
    #[inline]
    #[doc(hidden)]
    fn checked_incr(a: &mut u32, n: usize) -> Result<(), QuantumCircuitError> {
        (n < (Self::MAX - *a) as usize)
            .then(|| *a += n as u32)
            .ok_or(QuantumCircuitError::AllocOverflow)
    }

    #[inline]
    fn alloc(circ: &mut CircuitBuilder<'id>) -> Result<Self, QuantumCircuitError> {
        let count = Self::count(circ);
        let res = Self::new_unchecked(*count);
        Self::checked_incr(count, 1).map(|_| res)
    }

    #[inline]
    fn alloc_n<const N: usize>(circ: &mut CircuitBuilder<'id>) -> Result<[Self; N], QuantumCircuitError> {
        let count = Self::count(circ);
        let mut start = *count;
        Self::checked_incr(count, N).map(|_|
            [0; N].map(|_| {
                let res = Self::new_unchecked(start);
                start += 1;
                res
            })
        )
    }

    #[inline]
    fn alloc_list(circ: &mut CircuitBuilder<'id>, len: usize) -> Result<List<Self>, QuantumCircuitError> {
        let count = Self::count(circ);
        let start = *count;
        Self::checked_incr(count, len).map(|_| List::from_range(start..*count))
    }
}

/// Used to define the different symbols types.
macro_rules! symbols {
    {
        $(
            $(#[doc$($args: tt)*])* 
            $name: ident {
                count: $count: ident,
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
                fn count<'b>(circ: &'b mut CircuitBuilder) -> &'b mut u32 {
                    circ.$count()
                }
            }
        )*
    }
}

symbols! {
    /// TODO: Doc
    Qubit {
        count: qubit_count_mut,
    }

    /// TODO: Doc
    FormalParameter {
        count: parameter_count_mut,
    }

    /// TODO: Doc
    Bit {
        count: bit_count_mut,
        max: 1 << f32::MANTISSA_DIGITS,
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct List<T> {
    range: Range<u32>,
    _phantom: PhantomData<T>,
}

impl<'id, T: Symbol<'id> + 'id> List<T> {
    #[inline]
    fn from_range(range: Range<u32>) -> Self {
        List { range, _phantom: PhantomData }
    }

    #[inline]
    pub fn from_range_unchecked(range: Range<u32>) -> Self {
        List::from_range(range)
    }

    #[inline]
    pub fn new(start: T, end: T) -> Option<Self> {
        (start <= end).then(|| List::from_range(start.id()..end.id() + 1))
    }

    #[inline]
    pub fn as_range(&self) -> Range<u32> {
        self.range.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<T> {
        (n < self.len()).then(|| T::new_unchecked(n as u32 + self.range.start))
    }

    #[inline]
    pub fn contains(&self, symbol: T) -> bool {
        self.range.contains(&symbol.id())
    }

    #[inline]
    pub fn iter(&self) -> SymbolIter<T> {
        SymbolIter { inner: self.clone() }
    }
}

impl<'id, T: Symbol<'id> + 'id> From<T> for List<T> {
    #[inline]
    fn from(sym: T) -> Self {
        Self::from_range(sym.id()..sym.id() + 1)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SymbolIter<T> {
    inner: List<T>,
}

impl<'id, T: Symbol<'id> + 'id> Iterator for SymbolIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.range.next().map(T::new_unchecked)
    }
}

impl<'id, T: Symbol<'id> + 'id> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = SymbolIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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