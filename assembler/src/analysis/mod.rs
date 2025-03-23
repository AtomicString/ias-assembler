mod add;
mod div;
mod jump;
mod load;
mod mul;
mod sh;
mod stor;
mod sub;

use std::fmt::{Debug, Formatter};

use add::handle_add;
use common::rtn::Amount;
use div::handle_div;
use jump::handle_jump;
use load::handle_load;
use mul::handle_mul;
use pest::Parser;
use pest_derive::Parser;
use sh::{handle_lsh, handle_rsh};
use stor::handle_stor;
use sub::handle_sub;
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ComplexTerm {
    MQ,
    AC,
    PC,
    M(u16),
    Constant(u16),
}

impl Debug for ComplexTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexTerm::MQ => f.write_fmt(format_args!("MQ")),
            ComplexTerm::AC => f.write_fmt(format_args!("AC")),
            ComplexTerm::PC => f.write_fmt(format_args!("PC")),
            ComplexTerm::M(num) => f.write_fmt(format_args!("M({})", num)),
            ComplexTerm::Constant(num) => f.write_fmt(format_args!("{}", num)),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ComplexExpr {
    Unary(ComplexUnaryWithSize),
    Binary(ComplexBinary),
    ComplexMachineOp(ComplexMachineOp),
}

impl Debug for ComplexExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unary(unary) => f.write_fmt(format_args!("{:?}", unary)),
            Self::Binary(binary) => f.write_fmt(format_args!("{:?}", binary)),
            Self::ComplexMachineOp(op) => f.write_fmt(format_args!("{:?}", op)),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ComplexMachineOp {
    SkipLeftInstruction,
}

impl Debug for ComplexMachineOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SkipLeftInstruction => f.write_str("SkipLeftInstruction"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ComplexBinary {
    pub op1: ComplexUnaryWithSize,
    pub op2: ComplexUnaryWithSize,
    pub op: ComplexOperation,
    pub size: Amount,
}

impl Debug for ComplexBinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}{}{:?}{:?}",
            self.op1,
            match self.op {
                ComplexOperation::Addition => "+",
                ComplexOperation::Division => "/",
                ComplexOperation::Multiply => "*",
                ComplexOperation::Remainder => "%",
                ComplexOperation::Subtraction => "-",
            },
            self.op2,
            self.size,
        ))
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ComplexUnaryWithSize {
    pub unary: ComplexUnary,
    pub size: Amount,
}

impl Debug for ComplexUnaryWithSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}{:?}", self.unary, self.size))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexOperation {
    Addition,
    Subtraction,
    Division,
    Remainder,
    Multiply,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ComplexUnary {
    pub signless: ComplexTerm,
    pub is_neg: bool,
    pub is_abs: bool,
}

impl ComplexUnary {
    pub fn basic(signless: ComplexTerm) -> Self {
        Self {
            signless,
            is_neg: false,
            is_abs: false,
        }
    }
}

impl Debug for ComplexUnary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_abs {
            f.write_fmt(format_args!(
                "{}|{:?}|",
                if self.is_neg { "-" } else { "" },
                self.signless
            ))
        } else {
            f.write_fmt(format_args!(
                "{}{:?}",
                if self.is_neg { "-" } else { "" },
                self.signless
            ))
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MedReprSingle {
    pub left: ComplexUnaryWithSize,
    pub right: ComplexExpr,
    pub condition: Option<Condition>,
}

impl Debug for MedReprSingle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.condition.is_some() {
            f.write_fmt(format_args!("{:?} <- {:?}*", self.left, self.right,))
        } else {
            f.write_fmt(format_args!("{:?} <- {:?}", self.left, self.right,))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    ACNonNegative,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IASParser;

pub type MedRepr = (MedReprSingle, Option<MedReprSingle>);

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
        let operands = line_rules.next();
        let mnenomic_type = mnenomic.clone().into_inner().next().unwrap();
        let repr = match mnenomic_type.as_rule() {
            Rule::load => handle_load(operands.expect("LOAD needs operands")),
            Rule::stor => handle_stor(operands.expect("STOR needs operands")),
            Rule::jump => handle_jump(operands.expect("JUMP needs operands")),
            Rule::add => handle_add(operands.expect("ADD needs operands")),
            Rule::sub => handle_sub(operands.expect("SUB needs operands")),
            Rule::mul => handle_mul(operands.expect("MUL needs operands")),
            Rule::div => handle_div(operands.expect("DIV needs operands")),
            Rule::lsh => handle_lsh(operands),
            Rule::rsh => handle_rsh(operands),
            _ => unreachable!(),
        };
        //println!("{:?}", repr);
        med_repr.push(repr);
    }
    Ok(med_repr)
}
