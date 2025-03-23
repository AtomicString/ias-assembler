mod addr;
mod arithmetic;
mod jump;
mod load;

use addr::decode_addr_modify;
use arithmetic::decode_arithmetic;
use jump::{decode_con_jmp, decode_unc_jmp};
use load::decode_load;

use crate::analysis::MedRepr;

pub fn decode(instruction: i64) -> MedRepr {
    let opcode = instruction >> 12;
    println!("{:08b}", opcode);
    match opcode {
        0b00001010 | 0b00001001 | 0b00100001 | 0b00000001 | 0b00000010 | 0b00000011
        | 0b00000100 => decode_load(instruction),
        0b00001101 | 0b00001110 => decode_unc_jmp(instruction),
        0b00001111 | 0b00010000 => decode_con_jmp(instruction),
        0b00000101 | 0b00000111 | 0b00000110 | 0b00001000 | 0b00001011 | 0b00001100
        | 0b00010100 | 0b00010101 => decode_arithmetic(instruction),
        0b00010010 | 0b00010011 => decode_addr_modify(instruction),
        _ => unreachable!(),
    }
}
