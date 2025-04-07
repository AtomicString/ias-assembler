extern crate common;

mod analysis;
mod decoding;
mod encoding;
mod synthesis;

use analysis::{analysis, ComplexTerm, ComplexUnary, MedRepr, MedReprSingle};
use common::rtn::RegisterTransfer;
use encoding::{encode_ac, encode_mem, encode_mq, encode_pc};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use synthesis::{init_pc, syn_next_instr};
use tsify_next::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Tsify)]
pub struct MachineState {
    #[serde(with = "BigArray")]
    memory: [i64; 1024],
    reg_stack: RegisterStack,
    handling_right: bool,
    right_fetch: bool,
}

#[wasm_bindgen]
impl MachineState {
    #[wasm_bindgen(getter)]
    pub fn memory(&self) -> Vec<i64> {
        self.memory.to_vec()
    }

    #[wasm_bindgen(setter)]
    pub fn set_memory(&mut self, mem: Vec<i64>) {
        self.memory = mem.try_into().unwrap();
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Tsify)]
pub struct RegisterTransferList {
    transfer: Vec<RegisterTransfer>,
}

#[wasm_bindgen]
impl RegisterTransferList {
    #[wasm_bindgen(getter)]
    pub fn get_transfer(&self) -> Vec<RegisterTransfer> {
        self.transfer.clone()
    }
}

#[wasm_bindgen]
impl MachineState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MachineState {
        Self::default()
    }

    #[wasm_bindgen(getter)]
    pub fn get_reg_stack(&self) -> RegisterStack {
        self.reg_stack
    }
}

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            memory: [0; 1024],
            reg_stack: RegisterStack::default(),
            handling_right: false,
            right_fetch: false,
        }
    }
}

#[wasm_bindgen]
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RegisterStack {
    pub ac: i64,
    pub mq: i64,
    pub pc: i64,
}

#[wasm_bindgen]
impl RegisterStack {
    #[wasm_bindgen(getter)]
    pub fn get_ac(&self) -> i64 {
        self.ac
    }

    #[wasm_bindgen(getter)]
    pub fn get_mq(&self) -> i64 {
        self.mq
    }

    #[wasm_bindgen(getter)]
    pub fn get_pc(&self) -> i64 {
        self.pc
    }
}

#[wasm_bindgen]
pub fn gen_encoding(code: String, state: &mut MachineState) {
    let semantics: Vec<MedRepr> = analysis(code).expect("Unsuccessful parse");
    encode_instr(&semantics, &mut state.memory);
}

#[wasm_bindgen]
pub fn step(state: &mut MachineState) -> Vec<RegisterTransfer> {
    let mut reg_transfers = vec![];
    if state.reg_stack.pc == 0 {
        state.reg_stack.pc = 1;
        reg_transfers.push(init_pc());
    }

    reg_transfers.extend(syn_next_instr(state));

    reg_transfers
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
            curr_line = 0;
            counter += 1;
        }

        is_right = !is_right;
    }
    if curr_line != 0 && is_right {
        mem[counter] = curr_line;
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

        let analyzed = analysis(raw.to_string()).unwrap();
        let correct = [
            "(AC <- MQ, None)",
            "(MQ <- M(100), None)",
            "(M(100) <- AC, None)",
            "(AC <- M(100), None)",
            "(AC <- -M(100), None)",
            "(AC <- |M(100)|, None)",
            "(AC <- -|M(100)|, None)",
            "(PC <- 100, None)",
            "(PC <- 100, Some(PC <- SkipLeftInstruction))",
            "(PC <- 100*, None)",
            "(PC <- 100*, Some(PC <- SkipLeftInstruction*))",
            "(AC <- AC+M(100), None)",
            "(AC <- AC+|M(100)|, None)",
            "(AC <- AC-M(100), None)",
            "(AC <- AC-|M(100)|, None)",
            "(AC <- MQ*M(100)<40..79>, Some(MQ <- MQ*M(100)<0..39>))",
            "(AC <- AC%M(100), Some(MQ <- AC/M(100)))",
            "(AC <- AC*2, None)",
            "(AC <- AC/2, None)",
            "(M(100)<8..19> <- AC<28..39>, None)",
            "(M(100)<28..39> <- AC<28..39>, None)",
        ];

        for (line, compare) in analyzed.into_iter().zip(correct) {
            assert_eq!(format!("{:?}", line), compare);
        }
    }

    #[test]
    fn test_program_1() {
        let program = "LOAD M(500)\nADD M(501)\nSTOR M(502)\n";
        let mut state = MachineState::new();
        state.memory[500] = 2;
        state.memory[501] = 3;
        gen_encoding(program.to_string(), &mut state);
        //println!("{:?}", state.memory);
        println!("{:?}", step(&mut state));
        assert_eq!(state.reg_stack.ac, 2);
        println!("{:?}", step(&mut state));
        assert_eq!(state.reg_stack.ac, 5);
        println!("{:?}", step(&mut state));
        assert_eq!(state.memory[502], 5);
    }

    #[test]
    fn test_program_2() {
        let program = "
            LOAD M(501)
            SUB M(502)
            DIV M(503)
            STOR M(505)
            LOAD MQ
            STOR M(504)
        ";
    }
}
