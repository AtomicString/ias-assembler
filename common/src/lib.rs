pub mod rtn {
    use std::{fmt::Debug, fmt::Formatter};

    use serde::{Deserialize, Serialize};
    use tsify_next::Tsify;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[derive(Tsify, Serialize, Deserialize, Clone, Copy)]
    pub enum Register {
        AC,
        MQ,
        MBR,
        IBR,
        IR,
        PC,
        MAR,
    }

    impl Debug for Register {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::AC => f.write_fmt(format_args!("AC")),
                Self::MQ => f.write_fmt(format_args!("MQ")),
                Self::MBR => f.write_fmt(format_args!("MBR")),
                Self::IBR => f.write_fmt(format_args!("IBR")),
                Self::IR => f.write_fmt(format_args!("IR")),
                Self::PC => f.write_fmt(format_args!("PC")),
                Self::MAR => f.write_fmt(format_args!("MAR")),
            }
        }
    }

    #[derive(Tsify, Serialize, Deserialize, Clone, Copy)]
    pub enum Addressing {
        Register(Register),
        Unary(Register, UnaryOperation),
        MixedReg(Register, Register, BinaryOperation),
        MixedConst(Register, u16, BinaryOperation),
        Memory,
        Constant(u16),
    }

    impl Debug for Addressing {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Register(reg) => f.write_fmt(format_args!("{:?}", reg)),
                Self::Unary(reg, unary) => f.write_fmt(format_args!("{:?} {:?}", reg, unary)),
                Self::MixedReg(reg1, reg2, binary) => {
                    f.write_fmt(format_args!("{:?} {:?} {:?}", reg1, binary, reg2))
                }
                Self::MixedConst(reg, constant, binary) => {
                    f.write_fmt(format_args!("{:?} {:?} {:?}", reg, binary, constant))
                }
                Self::Memory => f.write_fmt(format_args!("M(MAR)")),
                Self::Constant(constant) => f.write_fmt(format_args!("{:?}", constant)),
            }
        }
    }

    #[derive(Tsify, Serialize, Deserialize, Clone, Copy)]
    pub enum UnaryOperation {
        BitFlip,
        LeftShift,
        RightShift,
    }

    impl Debug for UnaryOperation {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::BitFlip => f.write_fmt(format_args!("~")),
                Self::LeftShift => f.write_fmt(format_args!("<<")),
                Self::RightShift => f.write_fmt(format_args!(">>")),
            }
        }
    }

    #[derive(Tsify, Serialize, Deserialize, Clone, Copy)]
    pub enum BinaryOperation {
        Addition,
        Subtraction,
        Multiplication,
        Remainder,
        Division,
    }

    impl Debug for BinaryOperation {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Addition => f.write_fmt(format_args!("+")),
                Self::Subtraction => f.write_fmt(format_args!("-")),
                Self::Multiplication => f.write_fmt(format_args!("*")),
                Self::Remainder => f.write_fmt(format_args!("%")),
                Self::Division => f.write_fmt(format_args!("/")),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Tsify, Serialize, Deserialize)]
    pub enum Amount {
        Full,
        Range { start: usize, end: usize },
    }

    impl Debug for Amount {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Full => f.write_str(""),
                Self::Range { start, end } => f.write_fmt(format_args!("<{:?}..{:?}>", start, end)),
            }
        }
    }

    #[derive(Tsify, Serialize, Deserialize, Clone, Copy)]
    pub struct Operand {
        pub operand_type: Addressing,
        pub amount: Amount,
    }

    impl Debug for Operand {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}{:?}", self.operand_type, self.amount))
        }
    }

    #[derive(Tsify, Serialize, Deserialize)]
    pub struct RegisterTransfer {
        pub from: Operand,
        pub to: Operand,
    }

    impl Debug for RegisterTransfer {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?} <- {:?}", self.to, self.from))
        }
    }
}

//#[cfg(test)]
//mod tests {
//
//    pub fn
//
//}
