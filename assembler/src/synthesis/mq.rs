use common::rtn::{Addressing, Amount, Operand, Register, RegisterTransfer};

use crate::{
    analysis::{ComplexExpr, ComplexTerm, ComplexUnary, ComplexUnaryWithSize},
    synthesis::{m_to_mbr, reg_to_mar},
    MachineState,
};

pub fn handle_mq(right_op: ComplexExpr, state: &mut MachineState) -> Vec<RegisterTransfer> {
    match right_op {
        ComplexExpr::Unary(ComplexUnaryWithSize {
            unary:
                ComplexUnary {
                    signless: ComplexTerm::M(mem_addr),
                    is_abs: false,
                    is_neg: false,
                },
            size: Amount::Full,
        }) => {
            state.reg_stack.mq = state.memory[mem_addr as usize];
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                m_to_mbr(false),
            ]
        }
        _ => unreachable!(),
    }
}

pub(crate) fn mbr_to_mq() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MQ),
            amount: Amount::Full,
        },
    }
}
