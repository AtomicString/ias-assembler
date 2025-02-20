use common::rtn::Amount;
use pest::iterators::Pair;

use super::{
    ComplexExpr, ComplexTerm, ComplexUnary, ComplexUnarySignless, ComplexUnaryWithSize, MedRepr,
    MedReprSingle, Rule,
};

pub fn handle_stor(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    let mut op_rules = operands.into_inner();
    let op1 = op_rules.next().unwrap();
    if op_rules.next().is_some() {
        panic!("No second operand for STOR");
    }
    let mut op1_pairs = op1.into_inner();
    let op1_first = op1_pairs.next().unwrap();

    if Rule::signless != op1_first.as_rule() {
        panic!("STOR doesn't support negative or nonnegative");
    }

    let op1_second = op1_first.into_inner().next().unwrap();

    if Rule::memory != op1_second.as_rule() {
        panic!("STOR doesn't support non-memory operands");
    }

    let mut address_field = op1_second.into_inner().next().unwrap().into_inner();

    let address_num = address_field.next().unwrap().as_str().parse().unwrap();
    let slice_maybe = address_field.next();

    if let Some(slice) = slice_maybe {
        stor_mx_slice(address_num, slice)
    } else {
        stor_mx(address_num)
    }
}

fn stor_mx(address_num: u16) -> MedRepr {
    let ac_operand = ComplexExpr::Unary(super::ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Full,
    });
    let mem_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::M(address_num))),
        size: Amount::Full,
    };
    (
        MedReprSingle {
            left: mem_operand,
            right: ac_operand,
            condition: None,
        },
        None,
        None,
    )
}

fn stor_mx_slice(address_num: u16, slice: Pair<'_, Rule>) -> MedRepr {
    let mut slice_pairs = slice.into_inner();
    let first = slice_pairs.next().unwrap().as_str().parse().unwrap();
    let second = slice_pairs.next().unwrap().as_str().parse().unwrap();

    if !(first == 8 && second == 19 || first == 28 && second == 39) {
        panic!("Unknown slice for STOR operation");
    }

    let ac_operand = ComplexExpr::Unary(super::ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::AC)),
        size: Amount::Range(28..=39),
    });
    let mem_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::Signless(ComplexUnarySignless::Term(ComplexTerm::M(address_num))),
        size: Amount::Range(first..=second),
    };
    (
        MedReprSingle {
            left: mem_operand,
            right: ac_operand,
            condition: None,
        },
        None,
        None,
    )
}
