#![cfg(test)]

use crate::description::Instruction;
use crate::DecisionNode;
use crate::Insn;
use serde::Deserialize;
use std::rc::Rc;

#[test]
fn it_works() {
    let instructions: [Insn; 3] = [
        Insn {
            opcode: [0x00, 0x00, 0x00, 0x1a].into(),
            mask: [0x00, 0xfc, 0xe0, 0x7f].into(),
            mnemonic: "adc",
        },
        Insn {
            opcode: [0x00, 0x00, 0x00, 0x5a].into(),
            mask: [0x00, 0xfc, 0xe0, 0x7f].into(),
            mnemonic: "sbc",
        },
        Insn {
            opcode: [0x00, 0x00, 0x40, 0x29].into(),
            mask: [0x00, 0x00, 0xc0, 0x7e].into(),
            mnemonic: "ldp",
        },
    ];

    // TODO: Separate out instructions with masks that have only 1's, i.e. no don't care bits.
    // TODO: Separate out instructions with masks that have only 0's, i.e. no bits to match, warn and ignore.
    // TODO: What other checks can be done to optimize and validate the decision tree?

    let decision_tree = Rc::new(DecisionNode::Leafs(instructions.to_vec()));

    if let DecisionNode::Leafs(instructions) = &*decision_tree {
        for insn in instructions {
            let dont_care = !insn.mask;

            println!(
                "mask: {:?}, opcode: {:?}, mnemonic: {}, don't care: {:?}",
                insn.mask, insn.opcode, insn.mnemonic, dont_care
            );

            println!(
                "opcode leading zeroes: {}, opcode leading ones: {}",
                insn.opcode.leading_zeros(),
                insn.opcode.leading_ones()
            );
            println!(
                "mask leading zeroes: {}, mask leading ones: {}",
                insn.mask.leading_zeros(),
                insn.mask.leading_ones()
            );
            println!(
                "dont_care leading zeroes: {}, dont_care leading ones: {}",
                dont_care.leading_zeros(),
                dont_care.leading_ones()
            );
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Instructions(pub Vec<Instruction>);

#[test]
fn parse() -> std::io::Result<()> {
    let data = std::fs::read_to_string("./aarch64.json")?;
    let data = serde_json::from_str::<Instructions>(&data)?;

    print!("{:#x?}", data.0[0]);

    Ok(())
}
