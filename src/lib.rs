#![doc(html_logo_url = "https://raw.githubusercontent.com/L-Benjamin/trident/master/doc/logo.svg")]

#![allow(unused)] // temporary

mod genericity;

pub mod bitset;
pub mod circuit;

pub mod prelude {
    pub use crate::circuit::QuantumCircuit;
}