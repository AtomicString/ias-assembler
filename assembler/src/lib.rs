extern crate common;

mod analysis;

use analysis::{analysis, MedRepr};
use common::rtn::RegisterTransfer;

pub fn assemble(code: String) -> Vec<RegisterTransfer> {
    let semantics: Vec<MedRepr> = analysis(code).expect("Unsuccessful parse");
    //let final_rtn: Vec<RegisterTransfer> = synthesis_phase(semantics);
    return vec![];
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
