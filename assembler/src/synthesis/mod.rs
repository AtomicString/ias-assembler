mod ac;
mod mq;
use crate::synthesis::mq::handle_mq;
use crate::{analysis::ComplexUnary, synthesis::ac::handle_ac};
use common::rtn::{
    Addressing, Amount, BinaryOperation, Operand, Register, RegisterTransfer, UnaryOperation,
};

use crate::{
    analysis::{ComplexExpr, ComplexTerm, MedRepr},
    RegisterStack,
};

pub fn synthesis(
    basic: Vec<MedRepr>,
    mut mem: [i64; 1024],
    mut reg_stack: RegisterStack,
) -> Vec<RegisterTransfer> {
    let mut final_list: Vec<RegisterTransfer> = Vec::new();
    let mut is_right_instr: bool = false;
    final_list.push(init_pc());
    for line in basic {
        if !is_right_instr {
            final_list
                .extend([pc_to_mar(), m_to_mbr(false), mbr_to_ibr(), mbr_to_ir()].into_iter());
        } else {
            final_list.extend([ibr_to_ir()].into_iter());
        }
        let first_line = line.0;
        let ComplexUnary {
            signless: left_op,
            is_abs: false,
            is_neg: false,
        } = first_line.left.unary
        else {
            panic!("Left operand cannot be negative or absolute");
        };
        let right_op = first_line.right;

        match left_op {
            ComplexTerm::AC => final_list
                .extend(handle_ac(right_op, &mut mem, &mut reg_stack, is_right_instr).into_iter()),
            ComplexTerm::MQ => final_list
                .extend(handle_mq(right_op, &mut mem, &mut reg_stack, is_right_instr).into_iter()),
            ComplexTerm::PC => final_list
                .extend(handle_pc(right_op, &mut mem, &mut reg_stack, is_right_instr).into_iter()),
            ComplexTerm::M(mem_addr) => final_list.extend(
                handle_mem(mem_addr, right_op, &mut mem, &mut reg_stack, is_right_instr)
                    .into_iter(),
            ),
            _ => unreachable!(),
        }

        is_right_instr = !is_right_instr;
        final_list.push(inc_pc())
    }
    final_list
}

fn handle_mem(
    mem_addr: u16,
    right_op: ComplexExpr,
    mem: &mut [i64; 1024],
    reg_stack: &mut RegisterStack,
    is_right_instr: bool,
) -> Vec<RegisterTransfer> {
    todo!()
}

fn handle_pc(
    right_op: ComplexExpr,
    mem: &mut [i64; 1024],
    reg_stack: &mut RegisterStack,
    is_right_instr: bool,
) -> Vec<RegisterTransfer> {
    todo!()
}

pub(super) const fn flip_ac() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Unary(Register::AC, UnaryOperation::BitFlip),
            amount: Amount::Range(1..=1),
        },
        to: Operand {
            operand_type: Addressing::Register(Register::AC),
            amount: Amount::Range(1..=1),
        },
    }
}

pub(super) const fn reg_to_mar(reg: Register) -> RegisterTransfer {
    RegisterTransfer {
        to: Operand {
            operand_type: Addressing::Register(reg),
            amount: Amount::Range(8..=19),
        },
        from: Operand {
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
                Amount::Range(1..=39)
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
            amount: Amount::Range(20..=39),
        },
    }
}

pub(super) const fn mbr_to_ir() -> RegisterTransfer {
    RegisterTransfer {
        from: Operand {
            operand_type: Addressing::Register(Register::MBR),
            amount: Amount::Range(0..=7),
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
            amount: Amount::Range(0..=7),
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
