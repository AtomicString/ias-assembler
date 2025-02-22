use common::rtn::{Addressing, Amount, Operand, Register, RegisterTransfer};

use crate::{
    analysis::{ComplexExpr, ComplexTerm, ComplexUnary, ComplexUnaryWithSize},
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
        _ => unreachable!(),
    }
}
