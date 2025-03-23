use common::rtn::{
    Addressing, Amount, BinaryOperation, Operand, Register, RegisterTransfer, UnaryOperation,
};

use crate::{
    analysis::{
        ComplexBinary, ComplexExpr, ComplexOperation, ComplexTerm, ComplexUnary,
        ComplexUnaryWithSize,
    },
    synthesis::{flip_ac, m_to_mbr, reg_to_mar},
    MachineState,
};

pub fn handle_ac(right_op: ComplexExpr, state: &mut MachineState) -> Vec<RegisterTransfer> {
    use ComplexBinary as CBinary;
    use ComplexExpr as CE;
    use ComplexUnary as CUnary;
    use ComplexUnaryWithSize as CUnarySize;

    match right_op {
        ComplexExpr::Unary(CUnarySize { unary, size }) => handle_ac_unary(unary, size, state),
        ComplexExpr::Binary(CBinary { op1, op2, op, size }) => {
            handle_ac_binary(op1, op2, op, size, state)
        }
        _ => unreachable!(),
    }
}

fn handle_ac_binary(
    op1: ComplexUnaryWithSize,
    op2: ComplexUnaryWithSize,
    op: ComplexOperation,
    size: Amount,
    state: &mut MachineState,
) -> Vec<RegisterTransfer> {
    use ComplexTerm as CTerm;
    if op1.unary.signless != CTerm::AC
        && size == (Amount::Range { start: 40, end: 7 })
        && op == ComplexOperation::Multiply
    {
        if let CTerm::M(mem_addr) = op2.unary.signless {
            let res: i128 = state.reg_stack.mq as i128 * state.memory[mem_addr as usize] as i128;

            state.reg_stack.ac = ((res >> 40) & 0x7FFFFFFFFF) as i64;
            state.reg_stack.mq = (res & 0x7FFFFFFFFF) as i64;

            state.reg_stack.ac *= if res < 0 { -1 } else { 1 };
            state.reg_stack.mq *= if res < 0 { -1 } else { 1 };

            return vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                m_to_mbr(false),
                mq_times_mbr_to_ac(),
                mq_times_mbr_to_mq(),
            ];
        } else {
            panic!()
        }
    }

    let op2_unary = op2.unary;

    match (op, op2_unary) {
        (
            ComplexOperation::Addition,
            ComplexUnary {
                signless: CTerm::M(mem_addr),
                is_neg: false,
                is_abs,
            },
        ) => {
            let value = state.memory[mem_addr as usize];
            let value = if is_abs { value.abs() } else { value };
            state.reg_stack.ac =
                (state.reg_stack.ac + value).clamp(-2i64.pow(39) + 1, 2i64.pow(39) - 1);
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                m_to_mbr(is_abs),
                ac_add_mbr(),
            ]
        }
        (
            ComplexOperation::Subtraction,
            ComplexUnary {
                signless: CTerm::M(mem_addr),
                is_neg: false,
                is_abs,
            },
        ) => {
            let value = state.memory[mem_addr as usize];
            let value = if is_abs { value.abs() } else { value };
            state.reg_stack.ac =
                (state.reg_stack.ac + value).clamp(-2i64.pow(39) + 1, 2i64.pow(39) - 1);
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                m_to_mbr(is_abs),
                ac_sub_mbr(),
            ]
        }
        (
            ComplexOperation::Remainder,
            ComplexUnary {
                signless: CTerm::M(mem_addr),
                is_neg: false,
                is_abs: false,
            },
        ) => {
            let quo = state.reg_stack.ac / state.memory[mem_addr as usize];
            let rem = state.reg_stack.ac % state.memory[mem_addr as usize];
            state.reg_stack.ac = rem;
            state.reg_stack.mq = quo;
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                m_to_mbr(false),
                ac_div_m_to_mq(),
                ac_rem_m_to_ac(),
            ]
        } //TODO: Finish AC <- AC*2 andd AC <- AC/2
        (
            ComplexOperation::Multiply,
            ComplexUnary {
                signless: CTerm::Constant(2),
                is_neg: false,
                is_abs: false,
            },
        ) => {
            state.reg_stack.ac <<= 1;
            vec![ac_lsh()]
        }
        (
            ComplexOperation::Division,
            ComplexUnary {
                signless: CTerm::Constant(2),
                is_neg: false,
                is_abs: false,
            },
        ) => {
            state.reg_stack.ac = (state.reg_stack.ac >> 1).clamp(2_i64.pow(39), -2_i64.pow(39));
            vec![ac_rsh()]
        }
        _ => unreachable!(),
    }
}

fn handle_ac_unary(
    unary: ComplexUnary,
    size: Amount,
    state: &mut MachineState,
) -> Vec<RegisterTransfer> {
    use ComplexTerm as CTerm;
    use ComplexUnary as CUnary;

    if size != Amount::Full {
        panic!("Size must be full for unary ac operations")
    }
    match unary {
        CUnary {
            signless: CTerm::MQ,
            is_neg: false,
            is_abs: false,
        } => {
            state.reg_stack.ac = state.reg_stack.mq;
            vec![mq_to_ac()]
        }
        CUnary {
            signless: CTerm::M(mem_addr),
            is_neg,
            is_abs,
        } => {
            let value = state.memory[mem_addr as usize];
            let value = if is_abs { value.abs() } else { value };
            let value = if is_neg { -value } else { value };
            state.reg_stack.ac = value;
            if is_neg {
                vec![
                    reg_to_mar(if state.handling_right {
                        Register::IBR
                    } else {
                        Register::MBR
                    }),
                    m_to_mbr(is_abs),
                    flip_ac(),
                ]
            } else {
                vec![
                    reg_to_mar(if state.handling_right {
                        Register::IBR
                    } else {
                        Register::MBR
                    }),
                    m_to_mbr(is_abs),
                    mbr_to_ac(),
                ]
            }
        }
        _ => unreachable!(),
    }
}

pub(super) const fn mbr_to_ac() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_div_m_to_mq() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::AC,
                Register::MBR,
                BinaryOperation::Division,
            ),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MQ),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ac_rem_m_to_ac() -> RegisterTransfer {
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

pub(super) const fn mq_times_mbr_to_ac() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::MQ,
                Register::MBR,
                BinaryOperation::Multiplication,
            ),
            amount: Amount::Range { start: 40, end: 79 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn mq_times_mbr_to_mq() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedReg(
                Register::MQ,
                Register::MBR,
                BinaryOperation::Multiplication,
            ),
            amount: Amount::Range { start: 0, end: 39 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MQ),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn mq_to_ac() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MQ),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
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
