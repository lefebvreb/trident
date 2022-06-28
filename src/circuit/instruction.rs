macro_rules! instr_impl {
    {
        $(
            $(#[doc$($args :tt)*])*
            $name: ident {
                qubits:     $qubits: literal,
                params:     $params: literal,
                bits:       $bits: literal,
                unitary:    $unitary: literal,
                label:      $label: literal,
            },
        )*
    } => {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
        pub enum Instr {
            $(
                $(#[doc$($args)*])*
                $name,
            )*
        }

        use Instr::*;

        impl Instr {
            #[inline]
            pub const fn qubit_count(self) -> usize {
                match self {
                    $($name => $qubits,)*
                }
            }

            #[inline]
            pub const fn parameter_count(self) -> usize {
                match self {
                    $($name => $params,)*
                }
            }

            #[inline]
            pub const fn bit_count(self) -> usize {
                match self {
                    $($name => $bits,)*
                }
            }

            #[inline]
            pub const fn is_unitary(self) -> bool {
                match self {
                    $($name => $unitary,)*
                }
            }

            #[inline]
            pub const fn label(self) -> &'static str {
                match self {
                    $($name => $label,)*
                }
            }
        }
    }
}

instr_impl! {
    /// The identity gate.
    I {
        qubits: 1,
        params: 0,
        bits: 0,
        unitary: true,
        label: "I",
    },
    H {
        qubits: 1,
        params: 0,
        bits: 0,
        unitary: true,
        label: "H",
    },
    Cx {
        qubits: 2,
        params: 0,
        bits: 0,
        unitary: true,
        label: "CX",
    },
    Rx {
        qubits: 1,
        params: 1,
        bits: 0,
        unitary: true,
        label: "Rx",
    },
    Measure {
        qubits: 1,
        params: 0,
        bits: 1,
        unitary: false,
        label: "measure",
    },
    Reset {
        qubits: 1,
        params: 0,
        bits: 0,
        unitary: false,
        label: "reset",
    },
    Barrier {
        qubits: 0,
        params: 0,
        bits: 0,
        unitary: false,
        label: "barrier",
    },
}