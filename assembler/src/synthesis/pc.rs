use common::rtn::{Addressing, Amount, Operand, Register, RegisterTransfer};

use crate::{
    analysis::{ComplexExpr, ComplexMachineOp, ComplexUnaryWithSize, Condition, MedReprSingle},
    MachineState,
};

pub fn handle_pc(
    right_op: ComplexExpr,
    condition: Option<Condition>,
    state: &mut MachineState,
    line2: Option<MedReprSingle>,
) -> Vec<RegisterTransfer> {
    use crate::ComplexTerm as CTerm;
    use crate::ComplexUnary as CUnary;
    use ComplexExpr as CE;
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

            let ac_nonneg_check = matches!(condition, Some(Condition::ACNonNegative));
            if !ac_nonneg_check || state.reg_stack.ac >= 0 {
                if line2_is_skip {
                    state.reg_stack.pc = constant as i64;
                    state.handling_right = true;
                    state.right_fetch = true;
                    vec![const_to_pc(constant)]
                } else {
                    state.reg_stack.pc = constant as i64;
                    state.handling_right = false;
                    vec![const_to_pc(constant)]
                }
            } else {
                vec![]
            }
        }
        _ => unreachable!(),
    }
}

pub(super) fn const_to_pc(constant: u16) -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Constant(constant),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::PC),
            amount: Amount::Full,
        },
    }
}
