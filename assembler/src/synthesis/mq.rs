use common::rtn::{Amount, RegisterTransfer};

use crate::{
    analysis::{ComplexExpr, ComplexTerm, ComplexUnary, ComplexUnaryWithSize},
    RegisterStack,
};

pub fn handle_mq(
    right_op: ComplexExpr,
    mem: &mut [i64; 1024],
    reg_stack: &mut RegisterStack,
    is_right_instr: bool,
) -> Vec<RegisterTransfer> {
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
            vec![]
        }
        _ => unreachable!(),
    }
}
