use common::rtn::Amount;
use pest::iterators::Pair;

use super::{
    ComplexBinary, ComplexExpr, ComplexOperation, ComplexTerm, ComplexUnary, ComplexUnaryWithSize,
    MedRepr, MedReprSingle, Rule,
};

pub fn handle_div(operands: Pair<'_, Rule>) -> MedRepr {
    let mut op_rules = operands.into_inner();
    let op1 = op_rules.next().unwrap();
    if op_rules.next().is_some() {
        panic!("No second operand for MUL");
    }
    let mut op1_pairs = op1.into_inner();
    let op1_first = op1_pairs.next().unwrap();

    if Rule::signless == op1_first.as_rule() {
        panic!("MUL doesn't support negative or non negative");
    }

    let op1_second = op1_first.into_inner().next().unwrap();

    if Rule::memory != op1_second.as_rule() {
        panic!("MUL doesn't support non-memory operands");
    }

    let mut address_field = op1_second.into_inner().next().unwrap().into_inner();

    let address_num = address_field.next().unwrap().as_str().parse().unwrap();

    if address_field.next().is_some() {
        panic!("MUL doesn't support slicing");
    }

    let mq_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::MQ),
        size: Amount::Full,
    };

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::AC),
        size: Amount::Full,
    };

    let mx_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::M(address_num)),
        size: Amount::Full,
    };

    let ac_div_mx_operand = ComplexExpr::Binary(ComplexBinary {
        op1: ac_operand.clone(),
        op2: mx_operand.clone(),
        op: ComplexOperation::Division,
        size: Amount::Range(40..=79),
    });

    let ac_rem_mx_operand = ComplexExpr::Binary(ComplexBinary {
        op1: ac_operand.clone(),
        op2: mx_operand,
        op: ComplexOperation::Remainder,
        size: Amount::Range(0..=39),
    });

    (
        MedReprSingle {
            left: mq_operand,
            right: ac_div_mx_operand,
            condition: None,
        },
        Some(MedReprSingle {
            left: ac_operand,
            right: ac_rem_mx_operand,
            condition: None,
        }),
    )
}
