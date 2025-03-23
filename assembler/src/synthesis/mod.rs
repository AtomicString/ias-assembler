mod ac;
mod mem;
mod mq;
mod pc;

use crate::analysis::{ComplexUnary, ComplexUnaryWithSize, MedReprSingle};
use crate::decoding::decode;
use crate::synthesis::{ac::handle_ac, mq::handle_mq, pc::handle_pc};
use crate::MachineState;
use common::rtn::{
    Addressing, Amount, BinaryOperation, Operand, Register, RegisterTransfer, UnaryOperation,
};
use mem::handle_mem;

use crate::analysis::ComplexTerm;

pub fn syn_next_instr(state: &mut MachineState) -> Vec<RegisterTransfer> {
    let mut final_list: Vec<RegisterTransfer> = Vec::new();
    if !state.handling_right {
        final_list.extend([pc_to_mar(), m_to_mbr(false), mbr_to_ibr(), mbr_to_ir()]);
    } else {
        if state.right_fetch {
            final_list.extend([pc_to_mar(), m_to_mbr(false), mbr_to_ibr()]);
            state.right_fetch = false;
        }
        final_list.extend([ibr_to_ir()]);
    }

    let instr_string = if !state.handling_right {
        state.memory[state.reg_stack.pc as usize - 1] & 0xFFFFF
    } else {
        state.memory[state.reg_stack.pc as usize - 1] >> 20
    };

    let line = decode(instr_string);

    let first_line = line.0;
    let MedReprSingle {
        left:
            ComplexUnaryWithSize {
                unary:
                    ComplexUnary {
                        signless: left_op,
                        is_abs: false,
                        is_neg: false,
                    },
                size: left_size,
            },
        condition,
        ..
    } = first_line
    else {
        panic!("Left operand cannot be negative or absolute");
    };
    let right_op = first_line.right;

    let mut normal_pc = true;

    match left_op {
        ComplexTerm::AC => final_list.extend(handle_ac(right_op, state)),
        ComplexTerm::MQ => final_list.extend(handle_mq(right_op, state)),
        ComplexTerm::PC => {
            final_list.extend(handle_pc(right_op, condition, state, line.1));
            normal_pc = false;
        }
        ComplexTerm::M(mem_addr) => {
            final_list.extend(handle_mem(mem_addr, left_size, right_op, state))
        }
        _ => unreachable!(),
    }

    if normal_pc {
        state.handling_right = !state.handling_right;
        if !state.handling_right {
            state.reg_stack.pc += 1;
            final_list.push(inc_pc());
        }
    }

    final_list
}

//pub fn synthesis(
//    basic: Vec<MedRepr>,
//    mut mem: [i64; 1024],
//    mut reg_stack: RegisterStack,
//) -> Vec<RegisterTransfer> {
//    let mut final_list: Vec<RegisterTransfer> = Vec::new();
//    let mut is_right_instr: bool = false;
//
//    final_list.push(init_pc());
//    for line in basic {
//        if !is_right_instr {
//            final_list
//                .extend([pc_to_mar(), m_to_mbr(false), mbr_to_ibr(), mbr_to_ir()].into_iter());
//        } else {
//            final_list.extend([ibr_to_ir()].into_iter());
//        }
//        let first_line = line.0;
//        let ComplexUnaryWithSize {
//            unary:
//                ComplexUnary {
//                    signless: left_op,
//                    is_abs: false,
//                    is_neg: false,
//                },
//            size: left_size,
//        } = first_line.left
//        else {
//            panic!("Left operand cannot be negative or absolute");
//        };
//        let right_op = first_line.right;
//
//        match left_op {
//            ComplexTerm::AC => final_list
//                .extend(handle_ac(right_op, &mut mem, &mut reg_stack, is_right_instr).into_iter()),
//            ComplexTerm::MQ => final_list
//                .extend(handle_mq(right_op, &mut mem, &mut reg_stack, is_right_instr).into_iter()),
//            ComplexTerm::PC => final_list.extend(
//                handle_pc(right_op, &mut mem, &mut reg_stack, is_right_instr, line.1).into_iter(),
//            ),
//            ComplexTerm::M(mem_addr) => final_list.extend(
//                handle_mem(
//                    mem_addr,
//                    left_size,
//                    right_op,
//                    &mut mem,
//                    &mut reg_stack,
//                    is_right_instr,
//                )
//                .into_iter(),
//            ),
//            _ => unreachable!(),
//        }
//
//        is_right_instr = !is_right_instr;
//        final_list.push(inc_pc())
//    }
//    final_list
//}

pub(super) const fn flip_ac() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Unary(Register::AC, UnaryOperation::BitFlip),
            amount: Amount::Range { start: 1, end: 1 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Range { start: 1, end: 1 },
        },
    }
}

pub(super) const fn reg_to_mar(reg: Register) -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(reg),
            amount: Amount::Range { start: 8, end: 19 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MAR),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn init_pc() -> RegisterTransfer {
    RegisterTransfer {
        to: Operand {
            operand_type: Addressing::Register(Register::PC),
            amount: Amount::Full,
        },
        from: Operand {
            operand_type: Addressing::Constant(1),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn m_to_mbr(abs: bool) -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Memory,
            amount: if abs {
                Amount::Range { start: 1, end: 39 }
            } else {
                Amount::Full
            },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn pc_to_mar() -> RegisterTransfer {
    RegisterTransfer {
        to: Operand {
            operand_type: Addressing::Register(Register::MAR),
            amount: Amount::Full,
        },
        from: Operand {
            operand_type: Addressing::Register(Register::PC),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn mbr_to_ibr() -> RegisterTransfer {
    RegisterTransfer {
        to: Operand {
            operand_type: Addressing::Register(Register::IBR),
            amount: Amount::Full,
        },
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Range { start: 20, end: 39 },
        },
    }
}

pub(super) const fn mbr_to_ir() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Range { start: 0, end: 7 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::IR),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn ibr_to_ir() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::IBR),
            amount: Amount::Range { start: 0, end: 7 },
        },
        to: Operand {
            operand_type: Addressing::Register(Register::IR),
            amount: Amount::Full,
        },
    }
}

pub(super) const fn inc_pc() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::MixedConst(Register::PC, 1, BinaryOperation::Addition),
            amount: Amount::Full,
        },
        to: Operand {
            operand_type: Addressing::Register(Register::PC),
            amount: Amount::Full,
        },
    }
}
