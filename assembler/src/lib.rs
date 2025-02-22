extern crate common;

mod analysis;
mod synthesis;

use analysis::{analysis, MedRepr};
use common::rtn::RegisterTransfer;
use synthesis::synthesis;

#[derive(Default)]
pub struct RegisterStack {
    pub ac: i64,
    pub mq: i64,
}

pub fn assemble(code: String, mem: [i64; 1024]) -> Vec<RegisterTransfer> {
    let semantics: Vec<MedRepr> = analysis(code).expect("Unsuccessful parse");
    let reg_stack: RegisterStack = RegisterStack::default();
    let final_rtn: Vec<RegisterTransfer> = synthesis(semantics, mem, reg_stack);
    final_rtn
}

#[cfg(test)]
mod tests {
    use analysis::analysis;

    use super::*;

    #[test]
    fn intermediate_representaion() {
        let code = "
            LOAD MQ, M(801) ; Dummy comment

            MUL M(802)
            STOR M(803)
            LOAD MQ
            STOR M(804)
        ";

        assert!(analysis(code.to_string()).is_ok());
    }
}
