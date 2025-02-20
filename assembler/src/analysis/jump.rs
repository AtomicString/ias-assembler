use pest::iterators::Pair;

use super::{MedRepr, Rule};

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

    match op1_first.as_rule() {
        Rule::signless => {}
        Rule::nonneg_term => {
            let op1_second = op1_first.into_inner().next().unwrap();
            if Rule::term != op1_second.as_rule() {
                panic!("JUMP doesn't support absolute operands")
            }
        }
        _ => unreachable!(),
    }

    todo!()
}
