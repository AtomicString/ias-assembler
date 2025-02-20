mod jump;
mod load;
mod stor;

use core::panic;

use common::rtn::Amount;
use load::handle_load;
use pest::Parser;
use pest_derive::Parser;
use stor::handle_stor;
//
//pub struct Doc {
//    pub lines: Vec<Operation>
//}
//
//type Operands = (Negativity, Option<Term>);
//
//pub struct Operation {
//    pub mnemonic: Mnenomic,
//    pub operands: Operands
//}
//
//pub enum Mnenomic {
//    LOAD,
//    STOR,
//    JUMP,
//    ADD,
//    SUB,
//    MUL,
//    DIV,
//    LSH,
//    RSH
//}
//
//pub enum Term {
//    MQ,
//    M(Amount)
//}
//
//pub enum Negativity {
//    Signless(Signless),
//    NonNegative(Signless),
//    Negative(Signless)
//}
//
//pub enum Signless {
//    Absolute(Term),
//    Term(Term)
//}
//

#[allow(dead_code)]
#[derive(Debug)]
pub enum ComplexTerm {
    MQ,
    AC,
    PC,
    M(u16),
    Constant(u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ComplexExpr {
    Unary(ComplexUnaryWithSize),
    Binary(ComplexBinary),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ComplexBinary {
    pub op1: ComplexUnaryWithSize,
    pub op2: ComplexUnaryWithSize,
    pub op: ComplexOperation,
    pub size: Amount,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ComplexUnaryWithSize {
    pub unary: ComplexUnary,
    pub size: Amount,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ComplexOperation {
    Addition,
    Division,
    Remainder,
    Multiply,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ComplexUnary {
    Negative(ComplexUnarySignless),
    NonNegative(ComplexUnarySignless),
    Signless(ComplexUnarySignless),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ComplexUnarySignless {
    Absolute(ComplexTerm),
    Term(ComplexTerm),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MedReprSingle {
    pub left: ComplexUnaryWithSize,
    pub right: ComplexExpr,
    pub condition: Option<Condition>,
}

#[derive(Debug)]
pub enum Condition {
    ACNonNegative,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IASParser;

pub type MedRepr = (MedReprSingle, Option<MedReprSingle>, Option<MedReprSingle>);

pub fn analysis(code: String) -> Result<Vec<MedRepr>, pest::error::Error<Rule>> {
    let doc = IASParser::parse(Rule::doc, &code)?
        .next()
        .expect("Empty")
        .into_inner();
    let mut med_repr: Vec<MedRepr> = vec![];
    for line in doc {
        if line.as_rule() == Rule::EOI {
            continue;
        }

        let mut line_rules = line.into_inner();

        let mnenomic = line_rules.next().unwrap();
        let operands = line_rules.next().unwrap();
        let mnenomic_type = mnenomic.clone().into_inner().next().unwrap();
        let repr = match mnenomic_type.as_rule() {
            Rule::load => handle_load(operands),
            Rule::stor => handle_stor(operands),
            Rule::jump => handle_jump(operands),
            Rule::add => handle_add(operands),
            Rule::sub => handle_sub(operands),
            Rule::mul => handle_mul(operands),
            Rule::div => handle_div(operands),
            Rule::lsh => handle_lsh(operands),
            Rule::rsh => handle_rsh(operands),
            _ => unreachable!(),
        };
        println!("{:?}", repr);
        med_repr.push(repr);
    }
    Ok(med_repr)
}

fn handle_rsh(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}

fn handle_lsh(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}

fn handle_div(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}

fn handle_mul(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}

fn handle_sub(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}

fn handle_add(operands: pest::iterators::Pair<'_, Rule>) -> MedRepr {
    todo!()
}
