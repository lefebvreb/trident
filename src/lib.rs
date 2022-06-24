#![doc(html_logo_url = "https://raw.githubusercontent.com/L-Benjamin/trident/master/logo.svg")]

#![allow(unused)]

mod bitset;
mod circuit;
mod genericity;
mod linalg;

pub mod prelude {
    pub use crate::circuit::QuantumCircuit;
}