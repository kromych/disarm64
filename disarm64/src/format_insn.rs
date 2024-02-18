use crate::decoder::Opcode;
use disarm64_defn::defn::InsnOpcode;
use disarm64_defn::InsnOperandKind;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

fn get_int_reg_name(is_64: bool, reg: u8, is_sp: bool) -> &'static str {
    debug_assert!(reg < 32);
    const INT_REG: [[[&str; 32]; 2]; 2] = [
        [
            [
                "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "w10", "w11", "w12",
                "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22", "w23", "w24",
                "w25", "w26", "w27", "w28", "w29", "w30", "wsp",
            ],
            [
                "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12",
                "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
                "x25", "x26", "x27", "x28", "x29", "x30", "sp",
            ],
        ],
        [
            [
                "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "w10", "w11", "w12",
                "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22", "w23", "w24",
                "w25", "w26", "w27", "w28", "w29", "w30", "wzr",
            ],
            [
                "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12",
                "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
                "x25", "x26", "x27", "x28", "x29", "x30", "xzr",
            ],
        ],
    ];

    let is_sp = is_sp as usize;
    let is_64 = is_64 as usize;

    INT_REG[is_sp][is_64][reg as usize]
}

fn get_sve_reg_name(is_64: bool, reg: u8) -> &'static str {
    const SVE_REG: [[&str; 32]; 2] = [
        [
            "z0.s", "z1.s", "z2.s", "z3.s", "z4.s", "z5.s", "z6.s", "z7.s", "z8.s", "z9.s",
            "z10.s", "z11.s", "z12.s", "z13.s", "z14.s", "z15.s", "z16.s", "z17.s", "z18.s",
            "z19.s", "z20.s", "z21.s", "z22.s", "z23.s", "z24.s", "z25.s", "z26.s", "z27.s",
            "z28.s", "z29.s", "z30.s", "z31.s",
        ],
        [
            "z0.d", "z1.d", "z2.d", "z3.d", "z4.d", "z5.d", "z6.d", "z7.d", "z8.d", "z9.d",
            "z10.d", "z11.d", "z12.d", "z13.d", "z14.d", "z15.d", "z16.d", "z17.d", "z18.d",
            "z19.d", "z20.d", "z21.d", "z22.d", "z23.d", "z24.d", "z25.d", "z26.d", "z27.d",
            "z28.d", "z29.d", "z30.d", "z31.d",
        ],
    ];

    debug_assert!(reg < 32);
    let is_64 = is_64 as usize;

    SVE_REG[is_64][reg as usize]
}

/// Format an instruction to a string
pub fn format_insn(f: &mut impl Write, opcode: &Opcode) -> Result {
    let definition = opcode.definition();
    let bits = opcode.bits();

    write!(f, "{}\t", definition.mnemonic)?;
    let op_count = definition.operands.len();
    for (i, operand) in definition.operands.iter().enumerate() {
        let op_kind = operand.kind;
        match op_kind {
            InsnOperandKind::Rd
            | InsnOperandKind::Rn
            | InsnOperandKind::Rm
            | InsnOperandKind::Rt
            | InsnOperandKind::Rt2
            | InsnOperandKind::Rs
            | InsnOperandKind::Ra
            | InsnOperandKind::Rt_LS64
            | InsnOperandKind::Rt_SYS
            | InsnOperandKind::PAIRREG
            | InsnOperandKind::PAIRREG_OR_XZR
            | InsnOperandKind::SVE_Rm
            | InsnOperandKind::LSE128_Rt
            | InsnOperandKind::LSE128_Rt2 => {
                write!(f, "{op_kind:?}")?;
            }
            _ => write!(f, "op{i}")?,
        };

        if i + 1 < op_count {
            write!(f, ", ")?;
        }
    }

    Ok(())
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        format_insn(f, self)
    }
}
