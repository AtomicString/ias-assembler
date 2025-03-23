use common::rtn::Amount;

use crate::analysis::{MedRepr, MedReprSingle};

pub(super) fn decode_load(instruction: i64) -> MedRepr {
    use crate::analysis::ComplexExpr as CE;
    use crate::analysis::ComplexTerm as CTerm;
    use crate::analysis::ComplexUnary as CUnary;
    use crate::analysis::ComplexUnaryWithSize as CUnarySize;

    let opcode = instruction >> 12;
    let mem = instruction & 0xFFF;
    match opcode {
        0b00001010 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::MQ,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00001001 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::MQ,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00100001 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00000001 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00000010 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: true,
                        is_abs: false,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00000011 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: false,
                        is_abs: true,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        0b00000100 => (
            MedReprSingle {
                left: CUnarySize {
                    unary: CUnary {
                        signless: CTerm::AC,
                        is_neg: false,
                        is_abs: false,
                    },
                    size: Amount::Full,
                },
                right: CE::Unary(CUnarySize {
                    unary: CUnary {
                        signless: CTerm::M(mem as u16),
                        is_neg: true,
                        is_abs: true,
                    },
                    size: Amount::Full,
                }),
                condition: None,
            },
            None,
        ),
        _ => unreachable!(),
    }
}
