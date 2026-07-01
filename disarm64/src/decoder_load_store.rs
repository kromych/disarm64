// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use disarm64_defn::defn::Insn;
use disarm64_defn::defn::InsnOpcode;
use disarm64_defn::defn::InsnOperand;
use disarm64_defn::BitfieldSpec;
use disarm64_defn::InsnBitField;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFeatureSet;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandClass;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;
#[doc = r" A decoded instruction: its raw bits and a reference to its definition."]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Opcode {
    bits: u32,
    def: &'static Insn,
}
impl core::fmt::Debug for Opcode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({:#010x})", self.def.mnemonic, self.bits)
    }
}
impl InsnOpcode for Opcode {
    fn definition(&self) -> &'static Insn {
        self.def
    }
    fn bits(&self) -> u32 {
        self.bits
    }
}
const BITFIELDS_0: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rd,
    lsb: 0,
    width: 5,
}];
const BITFIELDS_1: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rn,
    lsb: 5,
    width: 5,
}];
const BITFIELDS_2: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::N,
        lsb: 22,
        width: 1,
    },
    BitfieldSpec {
        bitfield: InsnBitField::immr,
        lsb: 16,
        width: 6,
    },
    BitfieldSpec {
        bitfield: InsnBitField::imms,
        lsb: 10,
        width: 6,
    },
];
const BITFIELDS_3: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rs,
    lsb: 16,
    width: 5,
}];
const BITFIELDS_4: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rt,
    lsb: 0,
    width: 5,
}];
const BITFIELDS_5: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::Rn,
        lsb: 5,
        width: 5,
    },
    BitfieldSpec {
        bitfield: InsnBitField::imm9,
        lsb: 12,
        width: 9,
    },
    BitfieldSpec {
        bitfield: InsnBitField::index,
        lsb: 11,
        width: 1,
    },
];
const BITFIELDS_6: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rt2,
    lsb: 10,
    width: 5,
}];
const BITFIELDS_7: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::LSE128_Rt,
    lsb: 0,
    width: 5,
}];
const BITFIELDS_8: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::LSE128_Rt2,
    lsb: 16,
    width: 5,
}];
const BITFIELDS_9: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::imm9,
        lsb: 12,
        width: 9,
    },
    BitfieldSpec {
        bitfield: InsnBitField::index,
        lsb: 11,
        width: 1,
    },
];
const BITFIELDS_10: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::imm7,
        lsb: 15,
        width: 7,
    },
    BitfieldSpec {
        bitfield: InsnBitField::index2,
        lsb: 24,
        width: 1,
    },
];
const BITFIELDS_11: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm19,
    lsb: 5,
    width: 19,
}];
const BITFIELDS_12: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::Rn,
        lsb: 5,
        width: 5,
    },
    BitfieldSpec {
        bitfield: InsnBitField::imm12,
        lsb: 10,
        width: 12,
    },
];
const BITFIELDS_13: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::Rn,
        lsb: 5,
        width: 5,
    },
    BitfieldSpec {
        bitfield: InsnBitField::S_imm10,
        lsb: 22,
        width: 1,
    },
    BitfieldSpec {
        bitfield: InsnBitField::imm9,
        lsb: 12,
        width: 9,
    },
    BitfieldSpec {
        bitfield: InsnBitField::index,
        lsb: 11,
        width: 1,
    },
];
const OPERANDS_0: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rd_SP,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::Rn,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_1,
    },
    InsnOperand {
        kind: InsnOperandKind::LIMM,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
];
const OPERANDS_1: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rd,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::Rn,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_1,
    },
    InsnOperand {
        kind: InsnOperandKind::Rm_SFT,
        class: InsnOperandClass::MODIFIED_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: &[],
    },
];
const OPERANDS_2: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rd,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::Rn,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_1,
    },
    InsnOperand {
        kind: InsnOperandKind::LIMM,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
];
const OPERANDS_3: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_4: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_5: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_6: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt_LS64,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_7: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_8: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_9: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_OFFSET,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_5,
    },
];
const OPERANDS_10: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_OFFSET,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_5,
    },
];
const OPERANDS_11: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_12: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::LSE128_Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::LSE128_Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_8,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_13: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM13,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::imm_tag],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_14: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_15: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM7,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_16: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Ft,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Ft2,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM7,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_17: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM7,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S],
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_18: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_PCREL19,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_19: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
        bit_fields: &[],
    },
];
const OPERANDS_20: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_21: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S, InsnOperandQualifier::S_D],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_22: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Ft,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_PCREL19,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_23: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Ft,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: &[],
    },
];
const OPERANDS_24: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Ft,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_25: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Ft,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[
            InsnOperandQualifier::S_B,
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
            InsnOperandQualifier::S_Q,
        ],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_26: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM10,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_27: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B],
        bit_fields: &[],
    },
];
const OPERANDS_28: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_29: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_30: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: &[],
    },
];
const OPERANDS_31: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_32: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_33: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
        bit_fields: &[],
    },
];
const OPERANDS_34: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_35: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_36: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
        bit_fields: &[],
    },
];
const OPERANDS_37: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_38: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_H, InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_39: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_PCREL19,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_40: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S],
        bit_fields: &[],
    },
];
const OPERANDS_41: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_42: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_S],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_43: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::PRFOP,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_PCREL19,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_44: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::PRFOP,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_45: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::PRFOP,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_UIMM12,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_46: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::PRFOP,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM9,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_47: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt_SP,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X, InsnOperandQualifier::SP],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM13,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::imm_tag, InsnOperandQualifier::imm_tag],
        bit_fields: BITFIELDS_9,
    },
];
const OPERANDS_48: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt_LS64,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_49: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM11,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::imm_tag],
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_50: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_51: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::W],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::W, InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
#[doc = r" The decoded instruction definitions referenced by the decoder."]
static INSNS: [Insn; 286] = [
    Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0x12000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "and",
        aliases: &[],
        opcode: 0xa000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x72000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_2,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "ands",
        aliases: &[],
        opcode: 0x6a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "bic",
        aliases: &[],
        opcode: 0xa200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "bics",
        aliases: &[],
        opcode: 0x6a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "cas",
        aliases: &[],
        opcode: 0x88a07c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "casa",
        aliases: &[],
        opcode: 0x88e07c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "casab",
        aliases: &[],
        opcode: 0x8e07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casah",
        aliases: &[],
        opcode: 0x48e07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casal",
        aliases: &[],
        opcode: 0x88e0fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "casalb",
        aliases: &[],
        opcode: 0x8e0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casalh",
        aliases: &[],
        opcode: 0x48e0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casb",
        aliases: &[],
        opcode: 0x8a07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "cash",
        aliases: &[],
        opcode: 0x48a07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casl",
        aliases: &[],
        opcode: 0x88a0fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caslb",
        aliases: &[],
        opcode: 0x8a0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "caslh",
        aliases: &[],
        opcode: 0x48a0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casp",
        aliases: &[],
        opcode: 0x8207c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_5,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspa",
        aliases: &[],
        opcode: 0x8607c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_5,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspal",
        aliases: &[],
        opcode: 0x860fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_5,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspl",
        aliases: &[],
        opcode: 0x820fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_5,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "eon",
        aliases: &[],
        opcode: 0x4a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "eor",
        aliases: &[],
        opcode: 0x52000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "eor",
        aliases: &[],
        opcode: 0x4a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ld64b",
        aliases: &[],
        opcode: 0xf83fd000,
        mask: 0xfffffc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_6,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldadd",
        aliases: &[],
        opcode: 0xb8200000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldadda",
        aliases: &[],
        opcode: 0xb8a00000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldaddab",
        aliases: &[],
        opcode: 0x38a00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaddah",
        aliases: &[],
        opcode: 0x78a00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaddal",
        aliases: &[],
        opcode: 0xb8e00000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldaddalb",
        aliases: &[],
        opcode: 0x38e00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaddalh",
        aliases: &[],
        opcode: 0x78e00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaddb",
        aliases: &[],
        opcode: 0x38200000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldaddh",
        aliases: &[],
        opcode: 0x78200000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldaddl",
        aliases: &[],
        opcode: 0xb8600000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldaddlb",
        aliases: &[],
        opcode: 0x38600000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldaddlh",
        aliases: &[],
        opcode: 0x78600000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldapr",
        aliases: &[],
        opcode: 0xb8bfc000,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaprb",
        aliases: &[],
        opcode: 0x38bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaprh",
        aliases: &[],
        opcode: 0x78bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0x99400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0xd9400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_10,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapurb",
        aliases: &[],
        opcode: 0x19400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapurh",
        aliases: &[],
        opcode: 0x59400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_10,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_10,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursw",
        aliases: &[],
        opcode: 0x99800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_10,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldar",
        aliases: &[],
        opcode: 0x88dffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldarb",
        aliases: &[],
        opcode: 0x8dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldarh",
        aliases: &[],
        opcode: 0x48dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaxp",
        aliases: &[],
        opcode: 0x887f8000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaxr",
        aliases: &[],
        opcode: 0x885ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaxrb",
        aliases: &[],
        opcode: 0x85ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaxrh",
        aliases: &[],
        opcode: 0x485ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclr",
        aliases: &[],
        opcode: 0xb8201000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldclra",
        aliases: &[],
        opcode: 0xb8a01000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldclrab",
        aliases: &[],
        opcode: 0x38a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrah",
        aliases: &[],
        opcode: 0x78a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclral",
        aliases: &[],
        opcode: 0xb8e01000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldclralb",
        aliases: &[],
        opcode: 0x38e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclralh",
        aliases: &[],
        opcode: 0x78e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrb",
        aliases: &[],
        opcode: 0x38201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldclrh",
        aliases: &[],
        opcode: 0x78201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldclrl",
        aliases: &[],
        opcode: 0xb8601000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldclrlb",
        aliases: &[],
        opcode: 0x38601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldclrlh",
        aliases: &[],
        opcode: 0x78601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldclrp",
        aliases: &[],
        opcode: 0x19201000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpa",
        aliases: &[],
        opcode: 0x19a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpal",
        aliases: &[],
        opcode: 0x19e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpl",
        aliases: &[],
        opcode: 0x19601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldeor",
        aliases: &[],
        opcode: 0xb8202000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldeora",
        aliases: &[],
        opcode: 0xb8a02000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldeorab",
        aliases: &[],
        opcode: 0x38a02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldeorah",
        aliases: &[],
        opcode: 0x78a02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldeoral",
        aliases: &[],
        opcode: 0xb8e02000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldeoralb",
        aliases: &[],
        opcode: 0x38e02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldeoralh",
        aliases: &[],
        opcode: 0x78e02000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldeorb",
        aliases: &[],
        opcode: 0x38202000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldeorh",
        aliases: &[],
        opcode: 0x78202000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldeorl",
        aliases: &[],
        opcode: 0xb8602000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldeorlb",
        aliases: &[],
        opcode: 0x38602000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldeorlh",
        aliases: &[],
        opcode: 0x78602000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldg",
        aliases: &[],
        opcode: 0xd9600000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldgm",
        aliases: &[],
        opcode: 0xd9e00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_14,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldlar",
        aliases: &[],
        opcode: 0x88df7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldlarb",
        aliases: &[],
        opcode: 0x8df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldlarh",
        aliases: &[],
        opcode: 0x48df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x28400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x2c400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x29400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x28c00000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2d400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2cc00000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x69400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_17,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x68c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_17,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x18000000,
        mask: 0xbf000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_18,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8600800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_19,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8400400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb9400000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x1c000000,
        mask: 0x3f000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_22,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c600800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_23,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c400400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3d400000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldraa",
        aliases: &[],
        opcode: 0xf8200400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
        operands: OPERANDS_26,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrab",
        aliases: &[],
        opcode: 0xf8a00400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAC,
        operands: OPERANDS_26,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_27,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x39400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_30,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x79400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_32,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_34,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x39800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_35,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_37,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x79800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_38,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0x98000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_39,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_40,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8800400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_41,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_42,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldset",
        aliases: &[],
        opcode: 0xb8203000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldseta",
        aliases: &[],
        opcode: 0xb8a03000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsetab",
        aliases: &[],
        opcode: 0x38a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetah",
        aliases: &[],
        opcode: 0x78a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetal",
        aliases: &[],
        opcode: 0xb8e03000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsetalb",
        aliases: &[],
        opcode: 0x38e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetalh",
        aliases: &[],
        opcode: 0x78e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetb",
        aliases: &[],
        opcode: 0x38203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldseth",
        aliases: &[],
        opcode: 0x78203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsetl",
        aliases: &[],
        opcode: 0xb8603000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldsetlb",
        aliases: &[],
        opcode: 0x38603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsetlh",
        aliases: &[],
        opcode: 0x78603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsetp",
        aliases: &[],
        opcode: 0x19203000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpa",
        aliases: &[],
        opcode: 0x19a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpal",
        aliases: &[],
        opcode: 0x19e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpl",
        aliases: &[],
        opcode: 0x19603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsmax",
        aliases: &[],
        opcode: 0xb8204000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldsmaxa",
        aliases: &[],
        opcode: 0xb8a04000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsmaxab",
        aliases: &[],
        opcode: 0x38a04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsmaxah",
        aliases: &[],
        opcode: 0x78a04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsmaxal",
        aliases: &[],
        opcode: 0xb8e04000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsmaxalb",
        aliases: &[],
        opcode: 0x38e04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsmaxalh",
        aliases: &[],
        opcode: 0x78e04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsmaxb",
        aliases: &[],
        opcode: 0x38204000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsmaxh",
        aliases: &[],
        opcode: 0x78204000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsmaxl",
        aliases: &[],
        opcode: 0xb8604000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldsmaxlb",
        aliases: &[],
        opcode: 0x38604000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsmaxlh",
        aliases: &[],
        opcode: 0x78604000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsmin",
        aliases: &[],
        opcode: 0xb8205000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldsmina",
        aliases: &[],
        opcode: 0xb8a05000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsminab",
        aliases: &[],
        opcode: 0x38a05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsminah",
        aliases: &[],
        opcode: 0x78a05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsminal",
        aliases: &[],
        opcode: 0xb8e05000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldsminalb",
        aliases: &[],
        opcode: 0x38e05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsminalh",
        aliases: &[],
        opcode: 0x78e05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsminb",
        aliases: &[],
        opcode: 0x38205000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsminh",
        aliases: &[],
        opcode: 0x78205000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsminl",
        aliases: &[],
        opcode: 0xb8605000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldsminlb",
        aliases: &[],
        opcode: 0x38605000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldsminlh",
        aliases: &[],
        opcode: 0x78605000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldtr",
        aliases: &[],
        opcode: 0xb8400800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldtrb",
        aliases: &[],
        opcode: 0x38400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtrh",
        aliases: &[],
        opcode: 0x78400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtrsb",
        aliases: &[],
        opcode: 0x38800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_34,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldtrsh",
        aliases: &[],
        opcode: 0x78800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_37,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldtrsw",
        aliases: &[],
        opcode: 0xb8800800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_41,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldumax",
        aliases: &[],
        opcode: 0xb8206000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldumaxa",
        aliases: &[],
        opcode: 0xb8a06000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldumaxab",
        aliases: &[],
        opcode: 0x38a06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldumaxah",
        aliases: &[],
        opcode: 0x78a06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldumaxal",
        aliases: &[],
        opcode: 0xb8e06000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldumaxalb",
        aliases: &[],
        opcode: 0x38e06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldumaxalh",
        aliases: &[],
        opcode: 0x78e06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldumaxb",
        aliases: &[],
        opcode: 0x38206000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldumaxh",
        aliases: &[],
        opcode: 0x78206000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldumaxl",
        aliases: &[],
        opcode: 0xb8606000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldumaxlb",
        aliases: &[],
        opcode: 0x38606000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldumaxlh",
        aliases: &[],
        opcode: 0x78606000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldumin",
        aliases: &[],
        opcode: 0xb8207000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldumina",
        aliases: &[],
        opcode: 0xb8a07000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "lduminab",
        aliases: &[],
        opcode: 0x38a07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "lduminah",
        aliases: &[],
        opcode: 0x78a07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "lduminal",
        aliases: &[],
        opcode: 0xb8e07000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "lduminalb",
        aliases: &[],
        opcode: 0x38e07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "lduminalh",
        aliases: &[],
        opcode: 0x78e07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "lduminb",
        aliases: &[],
        opcode: 0x38207000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "lduminh",
        aliases: &[],
        opcode: 0x78207000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "lduminl",
        aliases: &[],
        opcode: 0xb8607000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "lduminlb",
        aliases: &[],
        opcode: 0x38607000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "lduminlh",
        aliases: &[],
        opcode: 0x78607000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0xb8400000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0x3c400000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldurb",
        aliases: &[],
        opcode: 0x38400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldurh",
        aliases: &[],
        opcode: 0x78400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldursb",
        aliases: &[],
        opcode: 0x38800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_34,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldursh",
        aliases: &[],
        opcode: 0x78800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_37,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldursw",
        aliases: &[],
        opcode: 0xb8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_41,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldxp",
        aliases: &[],
        opcode: 0x887f0000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldxr",
        aliases: &[],
        opcode: 0x885f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldxrb",
        aliases: &[],
        opcode: 0x85f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldxrh",
        aliases: &[],
        opcode: 0x485f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "orn",
        aliases: &[],
        opcode: 0x2a200000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "orr",
        aliases: &[],
        opcode: 0x32000000,
        mask: 0x7f800000,
        class: InsnClass::LOG_IMM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "orr",
        aliases: &[],
        opcode: 0x2a000000,
        mask: 0x7f200000,
        class: InsnClass::LOG_SHIFT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::const_from_bits(131080u64),
    },
    Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xd8000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_43,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_44,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_45,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfum",
        aliases: &[],
        opcode: 0xf8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_46,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64b",
        aliases: &[],
        opcode: 0xf83f9000,
        mask: 0xfffffc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_6,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64bv",
        aliases: &[],
        opcode: 0xf820b000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64bv0",
        aliases: &[],
        opcode: 0xf820a000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgm",
        aliases: &[],
        opcode: 0xd9a00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_14,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x69000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x68800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stllr",
        aliases: &[],
        opcode: 0x889f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stllrb",
        aliases: &[],
        opcode: 0x89f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stllrh",
        aliases: &[],
        opcode: 0x489f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlr",
        aliases: &[],
        opcode: 0x889ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlrb",
        aliases: &[],
        opcode: 0x89ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlrh",
        aliases: &[],
        opcode: 0x489ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0x99000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0xd9000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_10,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlurb",
        aliases: &[],
        opcode: 0x19000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlurh",
        aliases: &[],
        opcode: 0x59000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlxp",
        aliases: &[],
        opcode: 0x88208000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_50,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlxr",
        aliases: &[],
        opcode: 0x8800fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_51,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlxrb",
        aliases: &[],
        opcode: 0x800fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlxrh",
        aliases: &[],
        opcode: 0x4800fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stnp",
        aliases: &[],
        opcode: 0x28000000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stnp",
        aliases: &[],
        opcode: 0x2c000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x29000000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x28800000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2d000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2c800000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8200800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_19,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8000400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb9000000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c200800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_23,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c000400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3d000000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_27,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x39000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_30,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x79000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_32,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttr",
        aliases: &[],
        opcode: 0xb8000800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "sttrb",
        aliases: &[],
        opcode: 0x38000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttrh",
        aliases: &[],
        opcode: 0x78000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0xb8000000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0x3c000000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sturb",
        aliases: &[],
        opcode: 0x38000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sturh",
        aliases: &[],
        opcode: 0x78000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stxp",
        aliases: &[],
        opcode: 0x88200000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_50,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stxr",
        aliases: &[],
        opcode: 0x88007c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_51,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stxrb",
        aliases: &[],
        opcode: 0x8007c00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stxrh",
        aliases: &[],
        opcode: 0x48007c00,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stz2g",
        aliases: &[],
        opcode: 0xd9e00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stz2g",
        aliases: &[],
        opcode: 0xd9e00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_47,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stzgm",
        aliases: &[],
        opcode: 0xd9200000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_14,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swp",
        aliases: &[],
        opcode: 0xb8208000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swpa",
        aliases: &[],
        opcode: 0xb8a08000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swpab",
        aliases: &[],
        opcode: 0x38a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpah",
        aliases: &[],
        opcode: 0x78a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpal",
        aliases: &[],
        opcode: 0xb8e08000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swpalb",
        aliases: &[],
        opcode: 0x38e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpalh",
        aliases: &[],
        opcode: 0x78e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpb",
        aliases: &[],
        opcode: 0x38208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swph",
        aliases: &[],
        opcode: 0x78208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpl",
        aliases: &[],
        opcode: 0xb8608000,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swplb",
        aliases: &[],
        opcode: 0x38608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swplh",
        aliases: &[],
        opcode: 0x78608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_4,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpp",
        aliases: &[],
        opcode: 0x19208000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppa",
        aliases: &[],
        opcode: 0x19a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppal",
        aliases: &[],
        opcode: 0x19e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppl",
        aliases: &[],
        opcode: 0x19608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
];
pub fn decode(insn: u32) -> Option<Opcode> {
    if insn & 0x2000000 == 0 {
        if insn & 0x4000000 == 0 {
            if insn & 0x10000000 == 0 {
                if insn & 0x400000 == 0 {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe0fc00 == 0x8007c00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[263],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x48007c00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[264],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x88007c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[262],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8207c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[18],
                                                });
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88200000 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[261],
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe0fc00 == 0x800fc00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[234],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x4800fc00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[235],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x8800fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[233],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x820fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[21],
                                                });
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88208000 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[232],
                                                });
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x7fc00000 == 0x28000000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[236],
                                    });
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7fc00000 == 0x29000000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[238],
                                    });
                                }
                            } else {
                                if insn & 0xffc00000 == 0x69000000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[220],
                                    });
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89f7c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[223],
                                                });
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489f7c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[224],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889f7c00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[222],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a07c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[13],
                                                });
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a07c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[14],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a07c00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[6],
                                            });
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x89ffc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[226],
                                                });
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x489ffc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[227],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x889ffc00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[225],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a0fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[16],
                                                });
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a0fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[17],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a0fc00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[15],
                                            });
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28800000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[239],
                                    });
                                }
                            } else {
                                if insn & 0xfec00000 == 0x68800000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[221],
                                    });
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x85f7c00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[203],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485f7c00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[204],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885f7c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[202],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8607c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[19],
                                                });
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f0000 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[201],
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x85ffc00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[55],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485ffc00 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[56],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885ffc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[54],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x860fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[20],
                                                });
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f8000 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[53],
                                                });
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x7fc00000 == 0x28400000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[90],
                                    });
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7fc00000 == 0x29400000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[92],
                                    });
                                }
                            } else {
                                if insn & 0xffc00000 == 0x69400000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[96],
                                    });
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8df7c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[88],
                                                });
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48df7c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[89],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88df7c00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[87],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e07c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[8],
                                                });
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e07c00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[9],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e07c00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[7],
                                            });
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xfffffc00 == 0x8dffc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[51],
                                                });
                                            }
                                        } else {
                                            if insn & 0xfffffc00 == 0x48dffc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[52],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbffffc00 == 0x88dffc00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[50],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e0fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[11],
                                                });
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e0fc00 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[12],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e0fc00 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[10],
                                            });
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28c00000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[93],
                                    });
                                }
                            } else {
                                if insn & 0xfec00000 == 0x68c00000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[97],
                                    });
                                }
                            }
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x80000000 == 0 {
                            if insn & 0xbf000000 == 0x18000000 {
                                return Some(Opcode {
                                    bits: insn,
                                    def: &INSNS[98],
                                });
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0xff000000 == 0x98000000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[120],
                                    });
                                }
                            } else {
                                if insn & 0xff000000 == 0xd8000000 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[208],
                                    });
                                }
                            }
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[259],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[260],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[257],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[196],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[197],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[194],
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[198],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[200],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78800000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[199],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8800000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[211],
                                                    });
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38200000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[33],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78200000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[34],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8200000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[26],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a00000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[28],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a00000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[29],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a00000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[27],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38600000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[36],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78600000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[37],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8600000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[35],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e00000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[31],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e00000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[32],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e00000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[30],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38208000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[277],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78208000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[278],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8208000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[270],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a08000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[272],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a08000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[273],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a08000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[271],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38608000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[280],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78608000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[281],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8608000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[279],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e08000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[275],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e08000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[276],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e08000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[274],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38204000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[147],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78204000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[148],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8204000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[140],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a04000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[142],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a04000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[143],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a04000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[141],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38604000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[150],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78604000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[151],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8604000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[149],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e04000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[145],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e04000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[146],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e04000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[144],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xfffffc00 == 0x38bfc000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[39],
                                                                });
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0x78bfc000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[40],
                                                                });
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbffffc00 == 0xb8bfc000 {
                                                            return Some(Opcode {
                                                                bits: insn,
                                                                def: &INSNS[38],
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38202000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[80],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78202000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[81],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8202000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[73],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a02000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[75],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a02000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[76],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a02000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[74],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38602000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[83],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78602000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[84],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8602000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[82],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e02000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[78],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e02000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[79],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e02000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[77],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820a000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[216],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38206000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[177],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78206000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[178],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8206000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[170],
                                                                });
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a06000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[172],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a06000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[173],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a06000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[171],
                                                                });
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38606000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[180],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78606000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[181],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8606000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[179],
                                                                });
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e06000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[175],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e06000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[176],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e06000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[174],
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38201000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[64],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78201000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[65],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8201000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[57],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a01000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[59],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a01000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[60],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a01000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[58],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38601000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[67],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78601000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[68],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8601000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[66],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e01000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[62],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e01000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[63],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e01000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[61],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83f9000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[214],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38205000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[159],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78205000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[160],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8205000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[152],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a05000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[154],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a05000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[155],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a05000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[153],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38605000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[162],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78605000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[163],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8605000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[161],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e05000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[157],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e05000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[158],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e05000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[156],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83fd000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[25],
                                                        });
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38203000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[131],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78203000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[132],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8203000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[124],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a03000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[126],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a03000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[127],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a03000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[125],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38603000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[134],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78603000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[135],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8603000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[133],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e03000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[129],
                                                                        });
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e03000
                                                                    {
                                                                        return Some(Opcode {
                                                                            bits: insn,
                                                                            def: &INSNS[130],
                                                                        });
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e03000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[128],
                                                                    });
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820b000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[215],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38207000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[189],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78207000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[190],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8207000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[182],
                                                                });
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a07000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[184],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a07000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[185],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a07000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[183],
                                                                });
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38607000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[192],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78607000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[193],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8607000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[191],
                                                                });
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e07000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[187],
                                                                    });
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e07000 {
                                                                    return Some(Opcode {
                                                                        bits: insn,
                                                                        def: &INSNS[188],
                                                                    });
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e07000 {
                                                                return Some(Opcode {
                                                                    bits: insn,
                                                                    def: &INSNS[186],
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38000800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[255],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[256],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[254],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[165],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[166],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[164],
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[167],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[169],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00c00 == 0x78800800 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[168],
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38200800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[248],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78200800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[251],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8200800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[242],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38600800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[108],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78600800 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[111],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8600800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[99],
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38a00800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[114],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8a00800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[121],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78a00800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[117],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8a00800 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[209],
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x200000 == 0 {
                                if insn & 0x800000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38000400 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[249],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78000400 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[252],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8000400 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[243],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38400400 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[109],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78400400 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[112],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8400400 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[100],
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffa00400 == 0x38800400 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[115],
                                                });
                                            }
                                        } else {
                                            if insn & 0xffe00400 == 0xb8800400 {
                                                return Some(Opcode {
                                                    bits: insn,
                                                    def: &INSNS[122],
                                                });
                                            }
                                        }
                                    } else {
                                        if insn & 0xffa00400 == 0x78800400 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[118],
                                            });
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x800000 == 0 {
                                    if insn & 0xffa00400 == 0xf8200400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[106],
                                        });
                                    }
                                } else {
                                    if insn & 0xffa00400 == 0xf8a00400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[107],
                                        });
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x400000 == 0 {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[230],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[228],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[231],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9000000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[229],
                                                        });
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9200000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[269],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19208000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[282],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19201000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[69],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19203000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[136],
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9200800 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[217],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9200400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[218],
                                        });
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39000000 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[250],
                                            });
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79000000 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[253],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9000000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[244],
                                        });
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x000400 == 0 {
                                    if insn & 0x000800 == 0 {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[43],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[41],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[44],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9400000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[42],
                                                        });
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19608000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[285],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x19601000 {
                                                            return Some(Opcode {
                                                                bits: insn,
                                                                def: &INSNS[72],
                                                            });
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x19603000 {
                                                            return Some(Opcode {
                                                                bits: insn,
                                                                def: &INSNS[139],
                                                            });
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xd9600000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[85],
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9600800 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[267],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9600400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[268],
                                        });
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39400000 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[110],
                                            });
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79400000 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[113],
                                            });
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9400000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[101],
                                        });
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x000400 == 0 {
                                if insn & 0x000800 == 0 {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x19800000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[45],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99800000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[49],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59800000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[47],
                                                    });
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x19c00000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[46],
                                                    });
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59c00000 {
                                                    return Some(Opcode {
                                                        bits: insn,
                                                        def: &INSNS[48],
                                                    });
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9a00000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[219],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xd9e00000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[86],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a08000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[283],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e08000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[284],
                                                        });
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a01000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[70],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e01000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[71],
                                                        });
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a03000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[137],
                                                        });
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e03000 {
                                                        return Some(Opcode {
                                                            bits: insn,
                                                            def: &INSNS[138],
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0xffe00c00 == 0xd9a00800 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[212],
                                            });
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9e00800 {
                                            return Some(Opcode {
                                                bits: insn,
                                                def: &INSNS[265],
                                            });
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0xffe00400 == 0xd9a00400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[213],
                                        });
                                    }
                                } else {
                                    if insn & 0xffe00400 == 0xd9e00400 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[266],
                                        });
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x39800000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[116],
                                        });
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xb9800000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[123],
                                        });
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x79800000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[119],
                                        });
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xf9800000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[210],
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if insn & 0x10000000 == 0 {
                if insn & 0x400000 == 0 {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x3fc00000 == 0x2c000000 {
                                return Some(Opcode {
                                    bits: insn,
                                    def: &INSNS[237],
                                });
                            }
                        } else {
                            if insn & 0x3fc00000 == 0x2d000000 {
                                return Some(Opcode {
                                    bits: insn,
                                    def: &INSNS[240],
                                });
                            }
                        }
                    } else {
                        if insn & 0x3ec00000 == 0x2c800000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[241],
                            });
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0x3fc00000 == 0x2c400000 {
                                return Some(Opcode {
                                    bits: insn,
                                    def: &INSNS[91],
                                });
                            }
                        } else {
                            if insn & 0x3fc00000 == 0x2d400000 {
                                return Some(Opcode {
                                    bits: insn,
                                    def: &INSNS[94],
                                });
                            }
                        }
                    } else {
                        if insn & 0x3ec00000 == 0x2cc00000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[95],
                            });
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x3f000000 == 0x1c000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[102],
                            });
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600c00 == 0x3c000000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[258],
                                        });
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c400000 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[195],
                                        });
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600c00 == 0x3c200800 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[245],
                                        });
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c600800 {
                                        return Some(Opcode {
                                            bits: insn,
                                            def: &INSNS[103],
                                        });
                                    }
                                }
                            }
                        } else {
                            if insn & 0x400000 == 0 {
                                if insn & 0x3f600400 == 0x3c000400 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[246],
                                    });
                                }
                            } else {
                                if insn & 0x3f600400 == 0x3c400400 {
                                    return Some(Opcode {
                                        bits: insn,
                                        def: &INSNS[104],
                                    });
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x400000 == 0 {
                        if insn & 0x3f400000 == 0x3d000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[247],
                            });
                        }
                    } else {
                        if insn & 0x3f400000 == 0x3d400000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[105],
                            });
                        }
                    }
                }
            }
        }
    } else {
        if insn & 0x8000000 == 0 {
            if insn & 0x20000000 == 0 {
                if insn & 0x40000000 == 0 {
                    if insn & 0x7f800000 == 0x12000000 {
                        return Some(Opcode {
                            bits: insn,
                            def: &INSNS[0],
                        });
                    }
                } else {
                    if insn & 0x7f800000 == 0x52000000 {
                        return Some(Opcode {
                            bits: insn,
                            def: &INSNS[23],
                        });
                    }
                }
            } else {
                if insn & 0x40000000 == 0 {
                    if insn & 0x7f800000 == 0x32000000 {
                        return Some(Opcode {
                            bits: insn,
                            def: &INSNS[206],
                        });
                    }
                } else {
                    if insn & 0x7f800000 == 0x72000000 {
                        return Some(Opcode {
                            bits: insn,
                            def: &INSNS[2],
                        });
                    }
                }
            }
        } else {
            if insn & 0x200000 == 0 {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[1],
                            });
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[24],
                            });
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[207],
                            });
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a000000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[3],
                            });
                        }
                    }
                }
            } else {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa200000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[4],
                            });
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a200000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[22],
                            });
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a200000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[205],
                            });
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a200000 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[5],
                            });
                        }
                    }
                }
            }
        }
    }
    None
}
