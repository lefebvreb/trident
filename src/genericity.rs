use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug)]
pub(crate) struct Id<'id> {
    _phantom: PhantomData<Cell<&'id ()>>
}