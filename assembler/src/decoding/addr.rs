use common::rtn::Amount;

use crate::analysis::{MedRepr, MedReprSingle};

pub(super) fn decode_addr_modify(instruction: i64) -> MedRepr {
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let opcode = instruction >> 12;
    let mem = instruction & 0xFFF;
    match opcode {
        0b00010010 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Range { start: 8, end: 19 },
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Range { start: 28, end: 39 },
                }),
                condition: None,
            },
            None,
        ),
        0b00010011 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Range { start: 28, end: 39 },
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Range { start: 28, end: 39 },
                }),
                condition: None,
            },
            None,
        ),
        _ => unreachable!(),
    }
}
