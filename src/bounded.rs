use std::ops::Range;

use crate::genericity::Id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Parameter<'id> {
    Value(f64),
    Formal(FormalParameter<'id>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FormalParameter<'id> {
    n: u32,
    id: Id<'id>,
}

impl<'id> From<f64> for Parameter<'id> {
    #[inline]
    fn from(x: f64) -> Self {
        Parameter::Value(x)
    }
}

impl<'id> Parameter<'id> {
    #[inline]
    pub(crate) fn formal(n: u32) -> Self {
        Parameter::Formal(FormalParameter { n, id: Id::default() })
    }
}

pub struct FormalParameterList<'id> {
    range: Range<u32>,
    id: Id<'id>,
}

impl<'id> FormalParameterList<'id> {
    
}

#[test]
fn quick() {
    eprintln!("{}", std::mem::size_of::<Parameter>());
}