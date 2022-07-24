#![doc(html_logo_url = "https://raw.githubusercontent.com/L-Benjamin/trident/master/logo.svg")]

#![allow(unused)] // TODO: remove once not needed anymore

mod genericity;
mod storage;

pub mod bitset;
pub mod circuit;
pub mod instruction;
pub mod operation;
pub mod parameter;
pub mod symbol;
pub mod transpile;

pub mod prelude {
    //! `use trident::prelude::*;` to import the most common types, traits and functions.

    pub use crate::circuit::QuantumCircuit;
}