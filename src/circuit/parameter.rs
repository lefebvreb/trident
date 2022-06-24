use std::ops::Range;

use crate::genericity::Id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Parameter<'id> {
    bits: u64,
    id: Id<'id>,
}

impl<'id> Parameter<'id> {
    #[inline]
    pub(crate) fn new(bits: u64) -> Self {
        Self { bits, id: Id::default() }
    }

    #[inline]
    pub fn is_value(self) -> bool {
        f64::from_bits(self.bits).is_finite()
    }

    #[inline]
    pub fn is_formal(self) -> bool {
        !self.is_value()
    }

    #[inline]
    pub fn as_value(self) -> Option<f64> {
        self.is_value().then(|| f64::from_bits(self.bits))
    }

    #[inline]
    pub fn as_formal(self) -> Option<FormalParameter<'id>> {
        self.is_formal().then(|| FormalParameter::new((self.bits & 0xFFFFFFFF) as u32))
    }
}

impl<'id> From<f64> for Parameter<'id> {
    #[inline]
    fn from(mut value: f64) -> Self {
        if value.is_normal() {
            value = 0.0;
        }

        Self::new(value.to_bits())
    }
}

impl<'id> From<FormalParameter<'id>> for Parameter<'id> {
    #[inline]
    fn from(formal: FormalParameter<'id>) -> Self {
        Self::new(u64::from(formal.n) | f64::INFINITY.to_bits())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FormalParameter<'id> {
    n: u32,
    id: Id<'id>,
}

impl<'id> FormalParameter<'id> {
    #[inline]
    pub(crate) fn new(n: u32) -> FormalParameter<'id> {
        Self { n, id: Id::default() }
    } 

    #[inline]
    pub fn id(self) -> u32 {
        self.n
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FormalParameterList<'id> {
    range: Range<u32>,
    id: Id<'id>,
}

impl<'id> FormalParameterList<'id> {
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
    pub fn get(&self, id: u32) -> Option<FormalParameter<'id>> {
        (id < self.len()).then(|| FormalParameter::new(id + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: FormalParameter<'id>) -> bool {
        self.range.contains(&parameter.n)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = FormalParameter<'id>> {
        self.range.clone().map(FormalParameter::new)
    }
}