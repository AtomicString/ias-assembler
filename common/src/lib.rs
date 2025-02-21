pub mod rtn {
    use std::{fmt::Debug, fmt::Formatter, ops::RangeInclusive};
    pub enum Register {
        AC,
        MQ,
        MBR,
        IBR,
        IR,
        PC,
        MAR,
    }

    pub enum Addressing {
        Register(Register),
        Memory,
        Constant(u32),
    }

    #[derive(Clone)]
    pub enum Amount {
        Full,
        Range(RangeInclusive<usize>),
    }

    impl Debug for Amount {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Full => f.write_str(""),
                Self::Range(range) => {
                    f.write_fmt(format_args!("<{:?}..{:?}>", range.start(), range.end()))
                }
            }
        }
    }

    pub struct Operand {
        pub operand_type: Addressing,
        pub amount: Amount,
    }

    pub struct RegisterTransfer {
        pub from: Operand,
        pub to: Operand,
    }
}

//#[cfg(test)]
//mod tests {
//
//    pub fn
//
//}
