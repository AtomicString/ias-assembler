use common::rtn;
use pest::iterators::Pair;

use crate::analysis::ComplexExpr;

use super::{
    ComplexMachineOp, ComplexTerm, ComplexUnary, ComplexUnaryWithSize, Condition, MedRepr,
    MedReprSingle, Rule,
};

pub fn handle_jump(operands: Pair<'_, Rule>) -> MedRepr {
    let mut op_rules = operands.into_inner();
    let op1 = op_rules.next().unwrap();
    if op_rules.next().is_some() {
        panic!("No second operand for STOR");
    }
    let mut op1_pairs = op1.into_inner();
    let op1_first = op1_pairs.next().unwrap();

    if Rule::neg_term == op1_first.as_rule() {
        panic!("JUMP doesn't allow negative terms");
    }

    let non_neg: bool = Rule::nonneg_term == op1_first.as_rule();

    let op1_second = if non_neg {
        op1_first
            .into_inner()
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
    } else {
        op1_first.into_inner().next().unwrap()
    };

    if Rule::abs_term == op1_second.as_rule() {
        panic!("JUMP doesn't allow absolute terms")
    }

    let op1_third = op1_second.into_inner().next().unwrap();

    if Rule::mul_quo == op1_third.as_rule() {
        panic!("JUMP operand cannot be MQ")
    }

    let address = op1_third.into_inner().next().unwrap();
    let mut address_fields = address.into_inner();

    let address_num: u16 = address_fields.next().unwrap().as_str().parse().unwrap();
    let slice = address_fields
        .next()
        .expect("No slice found, left or right instruction?");

    let mut slice_nums = slice.into_inner();
    let first: u16 = slice_nums.next().unwrap().as_str().parse().unwrap();
    let second: u16 = slice_nums.next().unwrap().as_str().parse().unwrap();

    if !(first == 0 && second == 19 || first == 20 && second == 39) {
        panic!("Unknown slice for JUMP operand")
    }

    let pc_operand = ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::PC),
        size: rtn::Amount::Full,
    };

    let const_operand = ComplexExpr::Unary(ComplexUnaryWithSize {
        unary: ComplexUnary::basic(ComplexTerm::Constant(address_num)),
        size: rtn::Amount::Full,
    });

    let skip_operand = ComplexExpr::ComplexMachineOp(ComplexMachineOp::SkipLeftInstruction);

    let condition = if non_neg {
        Some(Condition::ACNonNegative)
    } else {
        None
    };

    (
        MedReprSingle {
            left: pc_operand.clone(),
            right: const_operand,
            condition,
        },
        if first == 20 && second == 39 {
            Some(MedReprSingle {
                left: pc_operand,
                right: skip_operand,
                condition,
            })
        } else {
            None
        },
    )
}
