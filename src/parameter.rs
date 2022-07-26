use thiserror::Error;

use crate::genericity::Id;
use crate::linalg::c64;

use super::symbol::{FormalParameter, Symbol};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, Debug)]
pub struct Parameter<'id> {
    _id: Id<'id>,
    bits: u32,
}

impl<'id> Parameter<'id> {
    /// The precision with which to compare two parameters. Quantum computers
    /// can't reach this level of precision nowadays.
    pub const PRECISION: f32 = 1E-5;

    /// Returns a new `Parameter` from it's bits.
    fn new(bits: u32) -> Self {
        Self { bits, _id: Id::default() }
    }

    pub fn is_value(self) -> bool {
        f32::from_bits(self.bits).is_finite()
    }

    pub fn is_formal(self) -> bool {
        !self.is_value()
    }

    pub fn as_value(self) -> Option<f32> {
        self.try_into().ok()
    }

    pub fn as_formal(self) -> Option<FormalParameter<'id>> {
        self.try_into().ok()
    }
}

impl<'id> From<f32> for Parameter<'id> {
    fn from(mut value: f32) -> Self {
        if !value.is_finite() {
            value = 0.0;
        }

        Self::new(value.to_bits())
    }
}

impl<'id> From<FormalParameter<'id>> for Parameter<'id> {
    fn from(formal: FormalParameter<'id>) -> Self {
        Self::new(u32::from(formal.id()) | f32::INFINITY.to_bits())
    }
}

impl PartialEq for Parameter<'_> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self.as_value(), rhs.as_value()) {
            (Some(x), Some(y)) => (x - y).abs() < Self::PRECISION,
            (None, None) => self.bits == rhs.bits,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
#[error("parameter is not a value")]
pub struct NotValue;

impl TryFrom<Parameter<'_>> for f32 {
    type Error = NotValue;

    fn try_from(param: Parameter) -> Result<Self, Self::Error> {
        param.is_value().then(|| f32::from_bits(param.bits)).ok_or(NotValue)
    }
}

impl TryFrom<Parameter<'_>> for c64 {
    type Error = NotValue;

    fn try_from(param: Parameter) -> Result<Self, Self::Error> {
        f32::try_from(param).map(c64::from)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
#[error("parameter is not formal")]
pub struct NotFormal;

impl<'id> TryFrom<Parameter<'id>> for FormalParameter<'id> {
    type Error = NotFormal;

    fn try_from(param: Parameter) -> Result<Self, Self::Error> {
        const MANTISSA_MASK: u32 = (1 << f32::MANTISSA_DIGITS) - 1;
        param.is_formal().then(|| Self::new_unchecked(param.bits & MANTISSA_MASK)).ok_or(NotFormal)
    }
}