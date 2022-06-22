use std::ops::Range;

use crate::genericity::Id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Parameter<'id> {
    Value(f64),
    Formal(FormalParameter<'id>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FormalParameter<'id> {
    i: u32,
    id: Id<'id>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FormalParameterList<'id> {
    range: Range<u32>,
    id: Id<'id>,
}

impl<'id> From<f64> for Parameter<'id> {
    #[inline]
    fn from(x: f64) -> Self {
        Parameter::Value(x)
    }
}

impl<'id> From<FormalParameter<'id>> for Parameter<'id> {
    #[inline]
    fn from(formal: FormalParameter<'id>) -> Self {
        Parameter::Formal(formal)
    }
}

impl<'id> FormalParameter<'id> {
    #[inline]
    pub(crate) fn new(i: u32) -> FormalParameter<'id> {
        FormalParameter { i, id: Id::default() }
    } 
}

impl<'id> FormalParameterList<'id> {
    #[inline]
    pub(crate) fn new(range: Range<u32>) -> Self {
        Self { range, id: Id::default() }
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.range.end - self.range.start
    }

    #[inline]
    pub fn get(&self, i: u32) -> Option<FormalParameter<'id>> {
        (i < self.len()).then(|| FormalParameter::new(i + self.range.start))
    }

    #[inline]
    pub fn contains(&self, parameter: FormalParameter<'id>) -> bool {
        self.range.contains(&parameter.i)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = FormalParameter<'id>> {
        self.range.clone().map(|i| FormalParameter { i, id: Id::default() })
    }
}

#[test]
fn quick() {
    eprintln!("{}", std::mem::size_of::<Option<f64>>());
}