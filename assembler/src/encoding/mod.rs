use common::rtn::Amount;

use crate::analysis::{
    ComplexBinary, ComplexExpr, ComplexMachineOp, ComplexOperation, ComplexTerm, ComplexUnary,
    ComplexUnaryWithSize, Condition, MedReprSingle,
};

pub fn encode_mem(mem_addr: u16, size: Amount) -> i64 {
    (match size {
        Amount::Full => 0b100001 << 12,
        Amount::Range(range) if range == (8..=19) => 0b10010 << 12,
        Amount::Range(range) if range == (28..=39) => 0b10011 << 12,
        _ => unreachable!(),
    } | mem_addr as i64)
}

pub fn encode_pc(
    right_op: ComplexExpr,
    condition: Option<Condition>,
    line2: Option<MedReprSingle>,
) -> i64 {
    use ComplexExpr as CE;
    use ComplexTerm as CTerm;
    use ComplexUnary as CUnary;
    use ComplexUnaryWithSize as CUnarySize;

    match right_op {
        CE::Unary(CUnarySize {
            unary:
                CUnary {
                    signless: CTerm::Constant(constant),
                    is_neg: false,
                    is_abs: false,
                },
            size: Amount::Full,
        }) => {
            let line2_is_skip = matches!(
                line2,
                Some(MedReprSingle {
                    left: ComplexUnaryWithSize {
                        unary: CUnary {
                            signless: CTerm::PC,
                            is_neg: false,
                            is_abs: false
                        },
                        size: Amount::Full
                    },
                    right: CE::ComplexMachineOp(ComplexMachineOp::SkipLeftInstruction),
                    ..
                })
            );
            (match (condition, line2_is_skip) {
                (None, false) => 0b1101 << 12,
                (None, true) => 0b1110 << 12,
                (Some(Condition::ACNonNegative), false) => 0b1111 << 12,
                (Some(Condition::ACNonNegative), true) => 0b10000 << 12,
            } | constant as i64)
        }
        _ => unreachable!(),
    }
}

pub fn encode_mq(right_op: ComplexExpr) -> i64 {
    use ComplexExpr as CE;
    use ComplexTerm as CTerm;
    use ComplexUnary as CUnary;
    use ComplexUnaryWithSize as CUnarySize;

    match right_op {
        CE::Unary(CUnarySize {
            unary:
                CUnary {
                    signless: CTerm::M(mem_addr),
                    is_neg: false,
                    is_abs: false,
                },
            size: Amount::Full,
        }) => 0b1001 << 12 | mem_addr as i64,
        _ => unreachable!(),
    }
}

pub fn encode_ac(right_op: ComplexExpr) -> i64 {
    use ComplexBinary as CBinary;
    use ComplexExpr as CE;
    use ComplexOperation as COp;
    use ComplexTerm as CTerm;
    use ComplexUnary as CUnary;
    use ComplexUnaryWithSize as CUnarySize;

    match right_op {
        CE::Unary(CUnarySize {
            unary:
                CUnary {
                    signless: CTerm::MQ,
                    is_neg: false,
                    is_abs: false,
                },
            size: Amount::Full,
        }) => 0b1010 << 12,
        CE::Unary(CUnarySize {
            unary:
                CUnary {
                    signless: CTerm::M(mem_addr),
                    is_neg,
                    is_abs,
                },
            size: Amount::Full,
        }) => {
            (match (is_abs, is_neg) {
                (false, false) => 0b1 << 12,
                (false, true) => 0b10 << 12,
                (true, false) => 0b11 << 12,
                (true, true) => 0b100 << 12,
            } | mem_addr as i64)
        }
        CE::Binary(CBinary {
            op1:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::M(mem_addr),
                            is_neg: false,
                            is_abs,
                        },
                    size: Amount::Full,
                },
            op,
            size: Amount::Full,
        }) if op == COp::Addition || op == COp::Subtraction => {
            (match (op, is_abs) {
                (COp::Addition, false) => 0b101 << 12,
                (COp::Addition, true) => 0b111 << 12,
                (COp::Subtraction, false) => 0b110 << 12,
                (COp::Subtraction, true) => 0b1000 << 12,
                _ => unreachable!(),
            } | mem_addr as i64)
        }
        CE::Binary(CBinary {
            op1:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::M(mem_addr),
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op: COp::Remainder,
            size: Amount::Full,
        }) => 0b1100 << 12 | mem_addr as i64,
        CE::Binary(CBinary {
            op1:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::MQ,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::M(mem_addr),
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op: COp::Multiply,
            size: Amount::Range(range),
        }) if range == (40..=79) => 0b1011 << 12 | mem_addr as i64,
        CE::Binary(CBinary {
            op1:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                CUnarySize {
                    unary:
                        CUnary {
                            signless: CTerm::Constant(2),
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op,
            size: Amount::Full,
        }) if op == COp::Multiply || op == COp::Division => match op {
            COp::Multiply => 0b10100 << 12,
            COp::Division => 0b10101 << 12,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
