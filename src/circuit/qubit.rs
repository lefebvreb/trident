use std::ops::Range;

use crate::genericity::Id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Qubit<'id> {
    n: u32,
    id: Id<'id>,
}

impl<'id> Qubit<'id> {
    #[inline]
    pub(crate) fn new(n: u32) -> Qubit<'id> {
        Self { n, id: Id::default() }
    } 

    #[inline]
    pub fn id(self) -> u32 {
        self.n
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct QubitList<'id> {
    range: Range<u32>,
    id: Id<'id>,
}

impl<'id> QubitList<'id> {
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
    pub fn get(&self, id: u32) -> Option<Qubit<'id>> {
        (id < self.len()).then(|| Qubit::new(id + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: Qubit<'id>) -> bool {
        self.range.contains(&parameter.n)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Qubit<'id>> {
        self.range.clone().map(Qubit::new)
    }
}