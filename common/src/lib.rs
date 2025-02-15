pub mod rtn {
    use std::ops::RangeInclusive;
    pub enum Register {
        AC,
        MQ,
        MBR,
        IBR,
        IR,
        PC,
        MAR
    }

    pub enum Addressing {
        Register(Register),
        Memory,
        Constant(u32)
    }

    pub enum Amount {
        Full,
        Range(RangeInclusive<usize>)
    }

    pub struct Operand {
        pub operand_type: Addressing,
        pub amount: Amount
    }

    pub struct RegisterTransfer {
        pub from: Operand,
        pub to: Operand
    }
}

//#[cfg(test)]
//mod tests {
//
//    pub fn 
//
//}
