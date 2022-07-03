use std::cell::Cell;
use std::marker::PhantomData;

// This is a type with an invariant lifetime. It is used to
// emulate genericity. It enables trusted indexing.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub(crate) struct Id<'id> {
    _phantom: PhantomData<Cell<&'id ()>>
}