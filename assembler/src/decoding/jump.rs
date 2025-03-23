use common::rtn::Amount;

use crate::analysis::{Condition, MedRepr, MedReprSingle};

pub(super) fn decode_con_jmp(instruction: i64) -> MedRepr {
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexMachineOp as CMachineOp;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let opcode = instruction >> 12;
    let mem = instruction & 0xFFF;

    match opcode {
        0b00001111 => {
            let pc_operand = CUnarySize {
                unary: CUnary::basic(CTerm::PC),
                size: Amount::Full,
            };

            let const_operand = CE::Unary(CUnarySize {
                unary: CUnary::basic(CTerm::Constant(mem as u16)),
                size: Amount::Full,
            });

            let condition = Some(Condition::ACNonNegative);

            (
                MedReprSingle {
                    left: pc_operand.clone(),
                    right: const_operand,
                    condition,
                },
                None,
            )
        }
        0b00010000 => {
            let pc_operand = CUnarySize {
                unary: CUnary::basic(CTerm::PC),
                size: Amount::Full,
            };

            let const_operand = CE::Unary(CUnarySize {
                unary: CUnary::basic(CTerm::Constant(mem as u16)),
                size: Amount::Full,
            });

            let skip_operand = CE::ComplexMachineOp(CMachineOp::SkipLeftInstruction);

            let condition = Some(Condition::ACNonNegative);

            (
                MedReprSingle {
                    left: pc_operand.clone(),
                    right: const_operand,
                    condition,
                },
                Some(MedReprSingle {
                    left: pc_operand,
                    right: skip_operand,
                    condition,
                }),
            )
        }
        _ => unreachable!(),
    }
}

pub(super) fn decode_unc_jmp(instruction: i64) -> MedRepr {
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexMachineOp as CMachineOp;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let opcode = instruction >> 12;
    let mem = instruction & 0xFFF;

    match opcode {
        0b00001101 => {
            let pc_operand = CUnarySize {
                unary: CUnary::basic(CTerm::PC),
                size: Amount::Full,
            };

            let const_operand = CE::Unary(CUnarySize {
                unary: CUnary::basic(CTerm::Constant(mem as u16)),
                size: Amount::Full,
            });

            let condition = None;

            (
                MedReprSingle {
                    left: pc_operand.clone(),
                    right: const_operand,
                    condition,
                },
                None,
            )
        }
        0b00001110 => {
            let pc_operand = CUnarySize {
                unary: CUnary::basic(CTerm::PC),
                size: Amount::Full,
            };

            let const_operand = CE::Unary(CUnarySize {
                unary: CUnary::basic(CTerm::Constant(mem as u16)),
                size: Amount::Full,
            });

            let skip_operand = CE::ComplexMachineOp(CMachineOp::SkipLeftInstruction);

            let condition = None;

            (
                MedReprSingle {
                    left: pc_operand.clone(),
                    right: const_operand,
                    condition,
                },
                Some(MedReprSingle {
                    left: pc_operand,
                    right: skip_operand,
                    condition,
                }),
            )
        }
        _ => unreachable!(),
    }
}
