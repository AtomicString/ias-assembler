use common::rtn::Amount;

use crate::analysis::ComplexBinary;

use super::{
    ComplexExpr, ComplexOperation, ComplexTerm, ComplexUnary, ComplexUnaryWithSize, MedRepr,
    MedReprSingle, Rule,
};

pub fn handle_sub(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    let mut op_rules = operands.into_inner();
    let op1 = op_rules.next().unwrap();
    if op_rules.next().is_some() {
        panic!("No second operand for SUB");
    }
    let mut op1_pairs = op1.into_inner();
    let op1_first = op1_pairs.next().unwrap().into_inner().next().unwrap();

    if Rule::neg_term == op1_first.as_rule() {
        panic!("SUB doesn't support negative");
    }

    let op1_second = op1_first.into_inner().next().unwrap();

    let is_abs = op1_second.as_rule() == Rule::abs_term;

    let op1_third = op1_second.into_inner().next().unwrap();

    if Rule::memory != op1_third.as_rule() {
        panic!("SUB doesn't support non-memory operands");
    }

    let mut address_field = op1_third.into_inner().next().unwrap().into_inner();

    let address_num = address_field.next().unwrap().as_str().parse().unwrap();

    if address_field.next().is_some() {
        panic!("SUB doesn't support slicing");
    }

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::AC),
        size: Amount::Full,
    };

    let mx_operand = ComplexUnaryWithSize {
        unary: ComplexUnary {
            signless: ComplexTerm::M(address_num),
            is_neg: false,
            is_abs,
        },
        size: Amount::Full,
    };

    let ac_sub_mx_operand = ComplexExpr::Binary(ComplexBinary {
        op1: ac_operand.clone(),
        op2: mx_operand,
        op: ComplexOperation::Subtraction,
        size: Amount::Full,
    });

    (
        MedReprSingle {
            left: ac_operand,
            right: ac_sub_mx_operand,
            condition: None,
        },
        None,
    )
}
