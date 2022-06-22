use std::rc::Rc;

#[derive(Clone)]
pub enum Instruction {
    Gate(Gate),
    NonUnitary(NonUnitary),
    Composite(Rc<[Instruction]>),
}

#[derive(Clone)]
pub enum Gate {
    H { target: u32 },
    Rx { target: u32, angle: f64 },
    Cx { target: u32, control: u32 },
    // [...]
    Special(Rc<dyn SpecialGate>),
}  

pub trait SpecialGate {
    fn targets(&self) -> Option<&[u8]>;
}

#[derive(Clone)]
pub enum NonUnitary {
    Measure { target: u32, bit: u32 },
    Reset { target: u32 },
    Barrier,
}

#[test]
fn quick() {
    eprintln!("{}", std::mem::size_of::<Instruction>());
}