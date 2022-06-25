macro_rules! instr_impl {
    { $($name: ident => $qubit: literal $parameter: literal $bit: literal $unitary: literal $str: literal),*, } => {
        #[repr(u32)]
        #[non_exhaustive]
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
        pub enum Instr {
            $($name),*
        }

        use Instr::*;

        impl Instr {
            #[inline]
            pub const fn qubit_count(self) -> usize {
                match self {
                    $($name => $qubit),*
                }
            }

            #[inline]
            pub const fn parameter_count(self) -> usize {
                match self {
                    $($name => $parameter),*
                }
            }

            #[inline]
            pub const fn bit_count(self) -> usize {
                match self {
                    $($name => $bit),*
                }
            }

            #[inline]
            pub const fn is_unitary(self) -> bool {
                match self {
                    $($name => $unitary),*
                }
            }

            #[inline]
            pub const fn name(self) -> &'static str {
                match self {
                    $($name => $str),*
                }
            }
        }
    }
}

instr_impl! {
    H => 1 0 0 true "h",
    Cx => 2 0 0 true "cx",
    Rx => 1 1 0 true "rx",
}