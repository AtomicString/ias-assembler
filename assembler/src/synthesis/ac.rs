use common::rtn::{
    Addressing, Amount, BinaryOperation, Operand, Register, RegisterTransfer, UnaryOperation,
};

use crate::{
    analysis::{
        ComplexBinary, ComplexExpr, ComplexOperation, ComplexTerm, ComplexUnary,
        ComplexUnaryWithSize,
    },
    synthesis::{flip_ac, m_to_mbr, reg_to_mar},
    RegisterStack,
};

pub fn handle_ac(
    right_op: ComplexExpr,
    mem: &mut [i64; 1024],
    reg_stack: &mut RegisterStack,
    is_right_instr: bool,
) -> Vec<RegisterTransfer> {
    match right_op {
        ComplexExpr::Unary(ComplexUnaryWithSize {
            unary:
                ComplexUnary {
                    signless: ComplexTerm::MQ,
                    is_neg: false,
                    is_abs: false,
                },
            ..
        }) => {
            reg_stack.ac = reg_stack.mq;
            vec![RegisterTransfer {
                from: Operand {
                    operand_type: Addressing::Register(Register::MQ),
                    amount: Amount::Full,
                },
                to: Operand {
                    operand_type: Addressing::Register(Register::AC),
                    amount: Amount::Full,
                },
            }]
        }
        ComplexExpr::Unary(ComplexUnaryWithSize {
            unary:
                ComplexUnary {
                    signless: ComplexTerm::M(mem_addr),
                    is_neg,
                    is_abs,
                },
            ..
        }) => {
            let value = mem[mem_addr as usize];
            let value = if is_abs { value.abs() } else { value };
            let value = if is_neg { -value } else { value };
            reg_stack.ac = value;
            if is_neg {
                vec![
                    reg_to_mar(if is_right_instr {
                        Register::IBR
                    } else {
                        Register::MBR
                    }),
                    m_to_mbr(is_abs),
                    flip_ac(),
                ]
            } else {
                vec![
                    reg_to_mar(if is_right_instr {
                        Register::IBR
                    } else {
                        Register::MBR
                    }),
                    m_to_mbr(is_abs),
                ]
            }
        }
        ComplexExpr::Binary(ComplexBinary {
            op1:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::M(mem_addr),
                            is_neg: false,
                            is_abs,
                        },
                    size: Amount::Full,
                },
            op,
            size: Amount::Full,
        }) if op == ComplexOperation::Addition || op == ComplexOperation::Subtraction => {
            let value = mem[mem_addr as usize];
            let value = if is_abs { value.abs() } else { value };
            match op {
                ComplexOperation::Addition => {
                    reg_stack.ac =
                        (reg_stack.ac + value).clamp(-2i64.pow(39) + 1, 2i64.pow(39) - 1);
                    vec![
                        reg_to_mar(if is_right_instr {
                            Register::IBR
                        } else {
                            Register::MBR
                        }),
                        m_to_mbr(is_abs),
                        ac_add_mbr(),
                    ]
                }
                ComplexOperation::Subtraction => {
                    reg_stack.ac =
                        (reg_stack.ac - value).clamp(-2i64.pow(39) + 1, 2i64.pow(39) - 1);
                    vec![
                        reg_to_mar(if is_right_instr {
                            Register::IBR
                        } else {
                            Register::MBR
                        }),
                        m_to_mbr(is_abs),
                        ac_sub_mbr(),
                    ]
                }
                _ => unreachable!(),
            }
        }
        ComplexExpr::Binary(ComplexBinary {
            op1:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::Constant(2),
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op,
            size: Amount::Full,
        }) if op == ComplexOperation::Multiply || op == ComplexOperation::Division => match op {
            ComplexOperation::Multiply => {
                reg_stack.ac = (reg_stack.ac * 2).clamp(-2i64.pow(39) + 1, 2i64.pow(39) - 1);
                vec![ac_lsh()]
            }
            ComplexOperation::Division => {
                reg_stack.ac /= 2;
                vec![ac_rsh()]
            }
            _ => unreachable!(),
        },
        ComplexExpr::Binary(ComplexBinary {
            op1:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::AC,
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op2:
                ComplexUnaryWithSize {
                    unary:
                        ComplexUnary {
                            signless: ComplexTerm::M(mem_addr),
                            is_neg: false,
                            is_abs: false,
                        },
                    size: Amount::Full,
                },
            op: ComplexOperation::Remainder,
            size: Amount::Full,
        }) => {
            reg_stack.ac %= mem[mem_addr as usize];
            vec![
                reg_to_mar(if is_right_instr {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                ac_remainder(),
            ]
        }
        _ => unreachable!(),
    }
}

pub(super) const fn ac_add_mbr() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::AC,
                Register::MBR,
                BinaryOperation::Addition,
            ),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_sub_mbr() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::AC,
                Register::MBR,
                BinaryOperation::Subtraction,
            ),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_lsh() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Unary(Register::AC, UnaryOperation::LeftShift),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_rsh() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Unary(Register::AC, UnaryOperation::RightShift),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_remainder() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::AC,
                Register::MBR,
                BinaryOperation::Remainder,
            ),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}
