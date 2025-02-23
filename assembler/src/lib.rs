extern crate common;

mod analysis;
mod encoding;
mod synthesis;

use analysis::{analysis, ComplexTerm, ComplexUnary, MedRepr, MedReprSingle};
use common::rtn::RegisterTransfer;
use encoding::{encode_ac, encode_mem, encode_mq, encode_pc};
use synthesis::synthesis;

#[derive(Default)]
pub struct RegisterStack {
    pub ac: i64,
    pub mq: i64,
}

pub fn assemble(code: String, mut mem: [i64; 1024]) -> Vec<RegisterTransfer> {
    let semantics: Vec<MedRepr> = analysis(code).expect("Unsuccessful parse");
    encode_instr(&semantics, &mut mem);
    let reg_stack: RegisterStack = RegisterStack::default();
    let final_rtn: Vec<RegisterTransfer> = synthesis(semantics, mem, reg_stack);
    final_rtn
}

fn encode_instr(semantics: &[(MedReprSingle, Option<MedReprSingle>)], mem: &mut [i64; 1024]) {
    let mut counter = 0;
    let mut curr_line: i64 = 0;
    let mut is_right = false;
    for line in semantics {
        let first_line = line.0.clone();
        let ComplexUnary {
            signless: left_op,
            is_abs: false,
            is_neg: false,
        } = first_line.left.unary
        else {
            panic!("Left operand cannot be negative or absolute");
        };
        let right_op = first_line.right;

        let enc_simple = match left_op {
            ComplexTerm::AC => encode_ac(right_op),
            ComplexTerm::MQ => encode_mq(right_op),
            ComplexTerm::PC => encode_pc(right_op, first_line.condition, line.1.clone()),
            ComplexTerm::M(mem_addr) => encode_mem(mem_addr, first_line.left.size),
            _ => unreachable!(),
        };

        if !is_right {
            curr_line |= enc_simple;
        } else {
            curr_line |= enc_simple << 20;
            mem[counter] = curr_line;
            counter += 1;
        }

        is_right = !is_right;
    }
}

#[cfg(test)]
mod tests {
    use analysis::analysis;

    use super::*;

    #[test]
    fn all_med_repr() {
        let raw = "
            LOAD MQ
            LOAD MQ,M(100)
            STOR M(100)
            LOAD M(100)
            LOAD -M(100)
            LOAD |M(100)|
            LOAD -|M(100)|
            JUMP M(100,0:19)
            JUMP M(100,20:39)
            JUMP +M(100,0:19)
            JUMP +M(100,20:39)
            ADD M(100)
            ADD |M(100)|
            SUB M(100)
            SUB |M(100)|
            MUL M(100)
            DIV M(100)
            LSH
            RSH
            STOR M(100, 8:19)
            STOR M(100, 28:39)
        ";

        assert!(analysis(raw.to_string()).is_ok());
    }

    #[test]
    fn all_encode() {
        let raw = "
            LOAD MQ
            LOAD MQ,M(100)
            STOR M(100)
            LOAD M(100)
            LOAD -M(100)
            LOAD |M(100)|
            LOAD -|M(100)|
            JUMP M(100,0:19)
            JUMP M(100,20:39)
            JUMP +M(100,0:19)
            JUMP +M(100,20:39)
            ADD M(100)
            ADD |M(100)|
            SUB M(100)
            SUB |M(100)|
            MUL M(100)
            DIV M(100)
            LSH
            RSH
            STOR M(100, 8:19)
            STOR M(100, 28:39)
        ";
        let mut mem: [i64; 1024] = [0; 1024];
        let semantics: Vec<MedRepr> = analysis(raw.to_string()).expect("Unsuccessful parse");
        encode_instr(&semantics, &mut mem);
    }

    #[test]
    fn all_med_repr_correct() {
        let raw = "
            LOAD MQ
            LOAD MQ,M(X)
            STOR M(X)
            LOAD M(X)
            LOAD -M(X)
            LOAD |M(X)|
            LOAD -|M(X)|
            JUMP M(X,0:19)
            JUMP M(X,20:39)
            JUMP +M(X,0:19)
            JUMP +M(X,20:39)
            ADD M(X)
            ADD |M(X)|
            SUB M(X)
            SUB |M(X)|
            MUL M(X)
            DIV M(X)
            LSH
            RSH
            STOR M(X, 8:19)
            STOR M(X, 28:39)
        ";
    }
}
