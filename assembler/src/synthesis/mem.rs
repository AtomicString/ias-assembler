use std::ops::RangeInclusive;

use common::rtn::{Addressing, Amount, Operand, Register, RegisterTransfer};

use crate::{analysis::ComplexExpr, synthesis::reg_to_mar, MachineState};

pub fn handle_mem(
    mem_addr: u16,
    left_size: Amount,
    right_op: ComplexExpr,
    state: &mut MachineState,
) -> Vec<RegisterTransfer> {
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let CE::Unary(CUnarySize {
        unary:
            CUnary {
                signless: CTerm::AC,
                is_neg: false,
                is_abs: false,
            },
        size: right_size,
    }) = right_op
    else {
        panic!("right operand not valid in RTN")
    };

    match (left_size, right_size) {
        (Amount::Full, Amount::Full) => {
            state.memory[mem_addr as usize] = state.reg_stack.ac;
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                ac_to_mbr(),
                mbr_to_mem(),
            ]
        }
        (
            Amount::Range {
                start: range_start1,
                end: range_end1,
            },
            Amount::Range {
                start: range_start2,
                end: range_end2,
            },
        ) if ((range_start1 == 8 && range_end1 == 19)
            || (range_start1 == 28 && range_end1 == 39))
            && (range_start2 == 28 && range_end2 == 39) =>
        {
            if (range_start1 == 8 && range_end1 == 19) {
                state.memory[mem_addr as usize] &= 0xFF000FFFFF;
                state.memory[mem_addr as usize] |= (state.reg_stack.ac & 0xFFF) << 20;
            } else {
                state.memory[mem_addr as usize] &= 0xFFFFFFF000;
                state.memory[mem_addr as usize] |= state.reg_stack.ac & 0xFFF;
            }
            vec![
                reg_to_mar(if state.handling_right {
                    Register::IBR
                } else {
                    Register::MBR
                }),
                ac_to_mbr(),
                mbr_to_mem_part(range_start1, range_end1, range_start2, range_end2),
            ]
        }
        _ => unreachable!(),
    }
}

pub(super) fn ac_to_mbr() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Full,
        },
    }
}

pub(super) fn mbr_to_mem() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Memory,
            amount: Amount::Full,
        },
    }
}

pub(super) fn mbr_to_mem_part(
    range_start1: usize,
    range_end1: usize,
    range_start2: usize,
    range_end2: usize,
) -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Range {
                start: range_start2,
                end: range_end2,
            },
        },
        to: Operand {
            operand_type: Addressing::Memory,
            amount: Amount::Range {
                start: range_start1,
                end: range_end1,
            },
        },
    }
}
