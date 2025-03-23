use common::rtn::Amount;

use crate::analysis::{MedRepr, MedReprSingle};

pub(super) fn decode_arithmetic(instruction: i64) -> MedRepr {
    use crate::analysis::ComplexBinary as CBinary;
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexOperation as COp;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let opcode = instruction >> 12;
    let mem = instruction & 0xFFF;

    let ac_op_mx = |op: COp, mem: u16, is_abs: bool| CBinary {
        op1: CUnarySize {
            unary: CUnary {
                signless: CTerm::AC,
                is_neg: false,
                is_abs: false,
            },
            size: Amount::Full,
        },
        op2: CUnarySize {
            unary: CUnary {
                signless: CTerm::M(mem),
                is_neg: false,
                is_abs,
            },
            size: Amount::Full,
        },
        op,
        size: Amount::Full,
    };

    match opcode {
        0b00000101 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Binary(ac_op_mx(COp::Addition, mem as u16, false)),
                condition: None,
            },
            None,
        ),
        0b00000111 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Binary(ac_op_mx(COp::Addition, mem as u16, true)),
                condition: None,
            },
            None,
        ),
        0b00000110 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Binary(ac_op_mx(COp::Subtraction, mem as u16, false)),
                condition: None,
            },
            None,
        ),
        0b00001000 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Binary(ac_op_mx(COp::Subtraction, mem as u16, true)),
                condition: None,
            },
            None,
        ),
        0b00001011 => {
            let mq_operand = CUnarySize {
                unary: CUnary::basic(CTerm::MQ),
                size: Amount::Full,
            };

            let ac_operand = CUnarySize {
                unary: CUnary::basic(CTerm::AC),
                size: Amount::Full,
            };

            let mx_operand = CUnarySize {
                unary: CUnary::basic(CTerm::M(mem as u16)),
                size: Amount::Full,
            };

            let mq_mul_mx_high_operand = CE::Binary(CBinary {
                op1: mq_operand.clone(),
                op2: mx_operand.clone(),
                op: COp::Multiply,
                size: Amount::Range { start: 40, end: 79 },
            });

            let mq_mul_mx_low_operand = CE::Binary(CBinary {
                op1: mq_operand.clone(),
                op2: mx_operand,
                op: COp::Multiply,
                size: Amount::Range { start: 0, end: 39 },
            });

            (
                MedReprSingle {
                    left: ac_operand,
                    right: mq_mul_mx_high_operand,
                    condition: None,
                },
                Some(MedReprSingle {
                    left: mq_operand,
                    right: mq_mul_mx_low_operand,
                    condition: None,
                }),
            )
        }
        0b00001100 => {
            let mq_operand = CUnarySize {
                unary: CUnary::basic(CTerm::MQ),
                size: Amount::Full,
            };

            let ac_operand = CUnarySize {
                unary: CUnary::basic(CTerm::AC),
                size: Amount::Full,
            };

            let mx_operand = CUnarySize {
                unary: CUnary::basic(CTerm::M(mem as u16)),
                size: Amount::Full,
            };

            let ac_div_mx_operand = CE::Binary(CBinary {
                op1: ac_operand.clone(),
                op2: mx_operand.clone(),
                op: COp::Division,
                size: Amount::Full,
            });

            let ac_rem_mx_operand = CE::Binary(CBinary {
                op1: ac_operand.clone(),
                op2: mx_operand,
                op: COp::Remainder,
                size: Amount::Full,
            });

            (
                MedReprSingle {
                    left: ac_operand,
                    right: ac_rem_mx_operand,
                    condition: None,
                },
                Some(MedReprSingle {
                    left: mq_operand,
                    right: ac_div_mx_operand,
                    condition: None,
                }),
            )
        }
        0b00010100 => {
            let ac_operand = CUnarySize {
                unary: CUnary::basic(CTerm::AC),
                size: Amount::Full,
            };

            let const_2 = CUnarySize {
                unary: CUnary::basic(CTerm::Constant(2)),
                size: Amount::Full,
            };

            let ac_mul_2 = CE::Binary(CBinary {
                op1: ac_operand.clone(),
                op2: const_2,
                op: COp::Multiply,
                size: Amount::Full,
            });

            (
                MedReprSingle {
                    left: ac_operand,
                    right: ac_mul_2,
                    condition: None,
                },
                None,
            )
        }
        0b00010101 => {
            let ac_operand = CUnarySize {
                unary: CUnary::basic(CTerm::AC),
                size: Amount::Full,
            };

            let const_2 = CUnarySize {
                unary: CUnary::basic(CTerm::Constant(2)),
                size: Amount::Full,
            };

            let ac_div_2 = CE::Binary(CBinary {
                op1: ac_operand.clone(),
                op2: const_2,
                op: COp::Division,
                size: Amount::Full,
            });

            (
                MedReprSingle {
                    left: ac_operand,
                    right: ac_div_2,
                    condition: None,
                },
                None,
            )
        }
        _ => unreachable!(),
    }
}
