use core::panic;

use common::rtn::Amount;
use pest::iterators::Pair;

use super::{
    ComplexBinary, ComplexExpr, ComplexOperation, ComplexTerm, ComplexUnary, ComplexUnaryWithSize,
    MedRepr, MedReprSingle, Rule,
};

pub fn handle_lsh(operands: Option<Pair<'_, Rule>>) -> MedRepr {
    if operands.is_some() {
        panic!("LSH doesn't support operands");
    }

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::AC),
        size: Amount::Full,
    };

    let const_2 = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::Constant(2)),
        size: Amount::Full,
    };

    let ac_mul_2 = ComplexExpr::Binary(ComplexBinary {
        op1: ac_operand.clone(),
        op2: const_2,
        op: ComplexOperation::Multiply,
        size: Amount::Full,
    });

    (
        MedReprSingle {
            left: ac_operand,
            right: ac_mul_2,
            condition: None,
        },
        None,
    )
}

pub fn handle_rsh(operands: Option<Pair<'_, Rule>>) -> MedRepr {
    if operands.is_some() {
        panic!("LSH doesn't support operands");
    }

    let ac_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::AC),
        size: Amount::Full,
    };

    let const_2 = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::Constant(2)),
        size: Amount::Full,
    };

    let ac_div_2 = ComplexExpr::Binary(ComplexBinary {
        op1: ac_operand.clone(),
        op2: const_2,
        op: ComplexOperation::Division,
        size: Amount::Full,
    });

    (
        MedReprSingle {
            left: ac_operand,
            right: ac_div_2,
            condition: None,
        },
        None,
    )
}
