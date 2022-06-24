use std::marker::PhantomData;
use std::ops::Range;

use crate::genericity::Id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bit<'id> {
    n: u32,
    id: Id<'id>,
}

impl<'id> Bit<'id> {
    #[inline]
    pub(crate) fn new(n: u32) -> Bit<'id> {
        Self { n, id: Id::default() }
    } 

    #[inline]
    pub fn id(self) -> u32 {
        self.n
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BitList<'id> {
    range: Range<u32>,
    id: Id<'id>,
}

impl<'id> BitList<'id> {
    // Creates a new BitList from a range.
    #[inline]
    pub(crate) fn new(range: Range<u32>) -> Self {
        Self { range, id: Id::default() }
    }

    #[inline]
    pub fn range(&self) -> Range<u32> {
        self.range.clone()
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.range.end - self.range.start
    }

    #[inline]
    pub fn get(&self, id: u32) -> Option<Bit<'id>> {
        (id < self.len()).then(|| Bit::new(id + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: Bit<'id>) -> bool {
        self.range.contains(&parameter.n)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Bit<'id>> {
        self.range.clone().map(Bit::new)
    }
}
