use common::rtn::Amount;
use pest::iterators::Pair;

use super::{
    ComplexExpr, ComplexTerm, ComplexUnary, ComplexUnarySignless, ComplexUnaryWithSize, MedRepr,
    MedReprSingle, Rule,
};

pub fn handle_load(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    let mut op_rules = operands.into_inner();
    let op1 = op_rules.next().unwrap();
    let op2_maybe = op_rules.next();
    let op1 = op1.into_inner().next().unwrap();

    let mut op1_pairs = op1.into_inner();

    let op1_first = op1_pairs.next().unwrap();

    match op1_first.as_rule() {
        Rule::signless => {
            let op1_second = op1_first.into_inner().next().unwrap();
            match op1_second.as_rule() {
                Rule::term => {
                    let op1_third = op1_second.into_inner().next().unwrap();
                    match op1_third.as_rule() {
                        Rule::mul_quo => {
                            if let Some(op2) = op2_maybe {
                                load_mq_mx(op2)
                            } else {
                                if op2_maybe.is_some() {
                                    panic!("Second operand not expected");
                                }
                                load_mq()
                            }
                        }
                        Rule::memory => {
                            if op2_maybe.is_some() {
                                panic!("Second operand not expected");
                            }
                            load_mx(op1_third)
                        }
                        _ => unreachable!(),
                    }
                }
                Rule::abs_term => {
                    if Rule::memory != op1_second.as_rule() {
                        panic!("Unknown abs term");
                    }
                    load_abs_mx(op1_second.into_inner().next().unwrap())
                }
                _ => unreachable!(),
            }
        }
        Rule::neg_term => {
            let op1_second = op1_pairs.next().unwrap();
            match op1_second.as_rule() {
                Rule::abs_term => {
                    if Rule::memory != op1_second.as_rule() {
                        panic!("Unknown abs term");
                    }
                    load_neg_abs_mx(op1_second.into_inner().next().unwrap())
                }
                Rule::term => {
                    if Rule::memory != op1_second.as_rule() {
                        panic!("Unknown abs term");
                    }
                    load_neg_mx(op1_second.into_inner().next().unwrap())
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn load_mq_mx(op2: Pair<'_, Rule>) -> MedRepr {
    let mq_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::MQ)),
        size: Amount::Full,
    };

    let op2 = op2.into_inner().next().unwrap();
    if Rule::memory == op2.as_rule() {
        let address = op2.into_inner().next().unwrap();
        let memory_num: u16 = address
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let memory_operand = ComplexExpr::Unary(ComplexUnaryWithSize {
            unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::M(memory_num))),
            size: Amount::Full,
        });
        return (
            MedReprSingle {
                left: mq_operand,
                right: memory_operand,
                condition: None,
            },
            None,
        );
    }
    panic!("Second operand is not valid!")
}

fn load_mq() -> MedRepr {
    let mq_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::MQ)),
        size: Amount::Full,
    };

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    };
    (
        MedReprSingle {
            left: ac_operand,
            right: ComplexExpr::Unary(mq_operand),
            condition: None,
        },
        None,
    )
}

fn load_mx(op1: Pair<'_, Rule>) -> MedRepr {
    let memory_num: u16 = op1.into_inner().next().unwrap().as_str().parse().unwrap();

    let memory_operand = ComplexExpr::Unary(ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::M(memory_num))),
        size: Amount::Full,
    });

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    };

    (
        MedReprSingle {
            left: ac_operand,
            right: memory_operand,
            condition: None,
        },
        None,
    )
}

fn load_abs_mx(op1: Pair<'_, Rule>) -> MedRepr {
    let memory_num: u16 = op1.as_str().parse().unwrap();

    let memory_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Absolute(ComplexTerm::M(memory_num))),
        size: Amount::Full,
    };

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    };

    (
        MedReprSingle {
            left: ac_operand,
            right: ComplexExpr::Unary(memory_operand),
            condition: None,
        },
        None,
    )
}

fn load_neg_abs_mx(op1: Pair<'_, Rule>) -> MedRepr {
    let memory_num: u16 = op1.as_str().parse().unwrap();

    let memory_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Negative(ComplexUnarySignless::Absolute(ComplexTerm::M(memory_num))),
        size: Amount::Full,
    };

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    };

    (
        MedReprSingle {
            left: ac_operand,
            right: ComplexExpr::Unary(memory_operand),
            condition: None,
        },
        None,
    )
}

fn load_neg_mx(op1: Pair<'_, Rule>) -> MedRepr {
    let memory_num: u16 = op1.as_str().parse().unwrap();

    let memory_operand = ComplexExpr::Unary(ComplexUnaryWithSize {
        unary: ComplexUnary::Negative(ComplexUnarySignless::Absolute(ComplexTerm::M(memory_num))),
        size: Amount::Full,
    });

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    };

    (
        MedReprSingle {
            left: ac_operand,
            right: memory_operand,
            condition: None,
        },
        None,
    )
}
