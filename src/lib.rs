#![doc(html_logo_url = "https://raw.githubusercontent.com/L-Benjamin/trident/master/logo.svg")]

#![allow(unused)]

mod genericity;

pub mod bitset;
pub mod circuit;
pub mod linalg;

pub mod prelude {
    pub use crate::circuit::QuantumCircuit;
}