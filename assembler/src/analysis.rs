use common::rtn::Amount;
use pest::Parser;
use pest_derive::Parser;
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
pub enum ComplexTerm {
    MQ,
    AC,
    PC,
    M(u16),
    Constant(u16)
}

#[allow(dead_code)]
pub enum ComplexExpr {
    Unary(ComplexUnaryWithSize),
    Binary(ComplexBinary)
}

#[allow(dead_code)]
pub struct ComplexBinary {
    pub op1: ComplexUnaryWithSize,
    pub op2: ComplexUnaryWithSize,
    pub op: ComplexOperation,
    pub size: Amount
}

#[allow(dead_code)]
pub struct ComplexUnaryWithSize {
    pub unary: ComplexUnary,
    pub size: Amount
}

#[allow(dead_code)]
pub enum ComplexOperation {
    Addition,
    Division,
    Remainder,
    Multiply,
}

#[allow(dead_code)]
pub enum ComplexUnary {
    Negative(ComplexUnarySignless),
    Signless(ComplexUnarySignless)
}

#[allow(dead_code)]
pub enum ComplexUnarySignless {
    Absolute(ComplexTerm),
    Term(ComplexTerm)
}

#[allow(dead_code)]
pub struct MedRepr {
    pub left: ComplexUnaryWithSize,
    pub right: ComplexBinary,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IASParser;

pub fn analysis(code: String) -> Result<Vec<MedRepr>, pest::error::Error<Rule>> {
    let doc = IASParser::parse(Rule::doc, &code)?.next().expect("Empty").into_inner();
    let med_repr: Vec<MedRepr> = vec![];
    for line in doc {
        if line.as_rule() == Rule::EOI {
            continue
        }
        
        let mut line_rules = line.into_inner();

        let mnenomic = line_rules.next().unwrap();
        let operands = line_rules.next().unwrap();
        let mnenomic_type = mnenomic.clone().into_inner().next().unwrap();
        println!("{:?} {:?}", mnenomic_type.as_rule(), operands.as_str());
    }
    Ok(vec![])
}
