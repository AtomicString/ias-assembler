mod rtn {
    use std::ops::RangeInclusive;
    enum Register {
        AC,
        MQ,
        MBR,
        IBR,
        IR,
        PC,
        MAR
    }

    enum Addressing {
        Register(Register),
        Memory,
        Constant(u32)
    }

    enum Amount {
        Full,
        Range(RangeInclusive<usize>)
    }

    struct Operand {
        pub operand_type: Addressing,
        pub amount: Amount
    }

    struct RegisterTransfer {
        pub from: Operand,
        pub to: Operand
    }
}

#[cfg(test)]
mod tests {
    
    pub fn 

}
