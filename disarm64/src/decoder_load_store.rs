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
#[doc = r" A decoded instruction: its raw bits, its definition, and its matchable"]
#[doc = r" identity."]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Opcode {
    bits: u32,
    def: &'static Insn,
    id: InsnId,
}
impl Opcode {
    #[doc = r" The instruction's identity, for matching against `InsnId`."]
    pub fn id(&self) -> InsnId {
        self.id
    }
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
    bitfield: InsnBitField::Rm,
    lsb: 16,
    width: 5,
}];
const BITFIELDS_8: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::LSE128_Rt,
    lsb: 0,
    width: 5,
}];
const BITFIELDS_9: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::LSE128_Rt2,
    lsb: 16,
    width: 5,
}];
const BITFIELDS_10: &[BitfieldSpec] = &[
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
const BITFIELDS_11: &[BitfieldSpec] = &[
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
const BITFIELDS_12: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm19,
    lsb: 5,
    width: 19,
}];
const BITFIELDS_13: &[BitfieldSpec] = &[
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
const BITFIELDS_14: &[BitfieldSpec] = &[
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
const BITFIELDS_15: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Ra,
    lsb: 10,
    width: 5,
}];
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
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
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
const OPERANDS_6: &[InsnOperand] = &[
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
const OPERANDS_7: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: &[],
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
const OPERANDS_9: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::Rs,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_10: &[InsnOperand] = &[
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
const OPERANDS_11: &[InsnOperand] = &[
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
const OPERANDS_12: &[InsnOperand] = &[
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
const OPERANDS_13: &[InsnOperand] = &[
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
const OPERANDS_14: &[InsnOperand] = &[
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
const OPERANDS_15: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Fm,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::Fd,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_16: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::LSE128_Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_8,
    },
    InsnOperand {
        kind: InsnOperandKind::LSE128_Rt2,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_9,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_17: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Fm,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
        ],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::Fd,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
        ],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_18: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_19: &[InsnOperand] = &[
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
const OPERANDS_20: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_21: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_22: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_23: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_24: &[InsnOperand] = &[
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
const OPERANDS_25: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_26: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_27: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_28: &[InsnOperand] = &[
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
const OPERANDS_29: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_30: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_31: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_14,
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
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B],
        bit_fields: &[],
    },
];
const OPERANDS_33: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_34: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_35: &[InsnOperand] = &[
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
const OPERANDS_36: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_37: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
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
        kind: InsnOperandKind::ADDR_REGOFF,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_B, InsnOperandQualifier::S_B],
        bit_fields: &[],
    },
];
const OPERANDS_39: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_40: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_41: &[InsnOperand] = &[
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
const OPERANDS_42: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_43: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_44: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_45: &[InsnOperand] = &[
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
const OPERANDS_46: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_47: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_48: &[InsnOperand] = &[
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
        qualifiers: &[InsnOperandQualifier::S_D],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_49: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Fd,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[InsnOperandQualifier::S_Q],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::Fa,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[InsnOperandQualifier::S_Q],
        bit_fields: BITFIELDS_15,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMM7,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[InsnOperandQualifier::S_Q],
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_50: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_12,
    },
];
const OPERANDS_51: &[InsnOperand] = &[
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
const OPERANDS_52: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_13,
    },
];
const OPERANDS_53: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_54: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_10,
    },
];
const OPERANDS_55: &[InsnOperand] = &[
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
const OPERANDS_56: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Fm,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[InsnOperandQualifier::S_H],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_57: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Fm,
        class: InsnOperandClass::FP_REG,
        qualifiers: &[
            InsnOperandQualifier::S_H,
            InsnOperandQualifier::S_S,
            InsnOperandQualifier::S_D,
        ],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::ADDR_SIMPLE,
        class: InsnOperandClass::ADDRESS,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_58: &[InsnOperand] = &[
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
        bit_fields: BITFIELDS_11,
    },
];
const OPERANDS_59: &[InsnOperand] = &[
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
const OPERANDS_60: &[InsnOperand] = &[
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
#[doc = r" A matchable identity for each instruction. The discriminant is the"]
#[doc = r" index into INSNS and INSN_IDS."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum InsnId {
    AND_Rd_SP_Rn_LIMM,
    AND_Rd_Rn_Rm_SFT,
    ANDS_Rd_Rn_LIMM,
    ANDS_Rd_Rn_Rm_SFT,
    BIC_Rd_Rn_Rm_SFT,
    BICS_Rd_Rn_Rm_SFT,
    CAS_Rs_Rt_ADDR_SIMPLE,
    CASA_Rs_Rt_ADDR_SIMPLE,
    CASAB_Rs_Rt_ADDR_SIMPLE,
    CASAH_Rs_Rt_ADDR_SIMPLE,
    CASAL_Rs_Rt_ADDR_SIMPLE,
    CASALB_Rs_Rt_ADDR_SIMPLE,
    CASALH_Rs_Rt_ADDR_SIMPLE,
    CASALT_Rs_Rt_ADDR_SIMPLE,
    CASAT_Rs_Rt_ADDR_SIMPLE,
    CASB_Rs_Rt_ADDR_SIMPLE,
    CASH_Rs_Rt_ADDR_SIMPLE,
    CASL_Rs_Rt_ADDR_SIMPLE,
    CASLB_Rs_Rt_ADDR_SIMPLE,
    CASLH_Rs_Rt_ADDR_SIMPLE,
    CASLT_Rs_Rt_ADDR_SIMPLE,
    CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPALT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPAT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPLT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CASPT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    CAST_Rs_Rt_ADDR_SIMPLE,
    EON_Rd_Rn_Rm_SFT,
    EOR_Rd_SP_Rn_LIMM,
    EOR_Rd_Rn_Rm_SFT,
    LD64B_Rt_LS64_ADDR_SIMPLE,
    LDADD_Rs_Rt_ADDR_SIMPLE,
    LDADDA_Rs_Rt_ADDR_SIMPLE,
    LDADDAB_Rs_Rt_ADDR_SIMPLE,
    LDADDAH_Rs_Rt_ADDR_SIMPLE,
    LDADDAL_Rs_Rt_ADDR_SIMPLE,
    LDADDALB_Rs_Rt_ADDR_SIMPLE,
    LDADDALH_Rs_Rt_ADDR_SIMPLE,
    LDADDB_Rs_Rt_ADDR_SIMPLE,
    LDADDH_Rs_Rt_ADDR_SIMPLE,
    LDADDL_Rs_Rt_ADDR_SIMPLE,
    LDADDLB_Rs_Rt_ADDR_SIMPLE,
    LDADDLH_Rs_Rt_ADDR_SIMPLE,
    LDAP_Rt_Rs_ADDR_SIMPLE,
    LDAPP_Rt_Rs_ADDR_SIMPLE,
    LDAPR_Rt_ADDR_SIMPLE,
    LDAPRB_Rt_ADDR_SIMPLE,
    LDAPRH_Rt_ADDR_SIMPLE,
    LDAPUR_Rt_ADDR_OFFSET,
    LDAPUR_Rt_X_ADDR_OFFSET,
    LDAPURB_Rt_ADDR_OFFSET,
    LDAPURH_Rt_ADDR_OFFSET,
    LDAPURSB_Rt_ADDR_OFFSET,
    LDAPURSB_Rt_W_ADDR_OFFSET,
    LDAPURSH_Rt_ADDR_OFFSET,
    LDAPURSH_Rt_W_ADDR_OFFSET,
    LDAPURSW_Rt_ADDR_OFFSET,
    LDAR_Rt_ADDR_SIMPLE,
    LDARB_Rt_ADDR_SIMPLE,
    LDARH_Rt_ADDR_SIMPLE,
    LDATXR_Rt_ADDR_SIMPLE,
    LDAXP_Rt_Rt2_ADDR_SIMPLE,
    LDAXR_Rt_ADDR_SIMPLE,
    LDAXRB_Rt_ADDR_SIMPLE,
    LDAXRH_Rt_ADDR_SIMPLE,
    LDBFADD_Fm_Fd_ADDR_SIMPLE,
    LDBFADDA_Fm_Fd_ADDR_SIMPLE,
    LDBFADDAL_Fm_Fd_ADDR_SIMPLE,
    LDBFADDL_Fm_Fd_ADDR_SIMPLE,
    LDBFMAX_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXA_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXAL_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXL_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXNM_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXNMA_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXNMAL_Fm_Fd_ADDR_SIMPLE,
    LDBFMAXNML_Fm_Fd_ADDR_SIMPLE,
    LDBFMIN_Fm_Fd_ADDR_SIMPLE,
    LDBFMINA_Fm_Fd_ADDR_SIMPLE,
    LDBFMINAL_Fm_Fd_ADDR_SIMPLE,
    LDBFMINL_Fm_Fd_ADDR_SIMPLE,
    LDBFMINNM_Fm_Fd_ADDR_SIMPLE,
    LDBFMINNMA_Fm_Fd_ADDR_SIMPLE,
    LDBFMINNMAL_Fm_Fd_ADDR_SIMPLE,
    LDBFMINNML_Fm_Fd_ADDR_SIMPLE,
    LDCLR_Rs_Rt_ADDR_SIMPLE,
    LDCLRA_Rs_Rt_ADDR_SIMPLE,
    LDCLRAB_Rs_Rt_ADDR_SIMPLE,
    LDCLRAH_Rs_Rt_ADDR_SIMPLE,
    LDCLRAL_Rs_Rt_ADDR_SIMPLE,
    LDCLRALB_Rs_Rt_ADDR_SIMPLE,
    LDCLRALH_Rs_Rt_ADDR_SIMPLE,
    LDCLRB_Rs_Rt_ADDR_SIMPLE,
    LDCLRH_Rs_Rt_ADDR_SIMPLE,
    LDCLRL_Rs_Rt_ADDR_SIMPLE,
    LDCLRLB_Rs_Rt_ADDR_SIMPLE,
    LDCLRLH_Rs_Rt_ADDR_SIMPLE,
    LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDEOR_Rs_Rt_ADDR_SIMPLE,
    LDEORA_Rs_Rt_ADDR_SIMPLE,
    LDEORAB_Rs_Rt_ADDR_SIMPLE,
    LDEORAH_Rs_Rt_ADDR_SIMPLE,
    LDEORAL_Rs_Rt_ADDR_SIMPLE,
    LDEORALB_Rs_Rt_ADDR_SIMPLE,
    LDEORALH_Rs_Rt_ADDR_SIMPLE,
    LDEORB_Rs_Rt_ADDR_SIMPLE,
    LDEORH_Rs_Rt_ADDR_SIMPLE,
    LDEORL_Rs_Rt_ADDR_SIMPLE,
    LDEORLB_Rs_Rt_ADDR_SIMPLE,
    LDEORLH_Rs_Rt_ADDR_SIMPLE,
    LDFADD_Fm_Fd_ADDR_SIMPLE,
    LDFADDA_Fm_Fd_ADDR_SIMPLE,
    LDFADDAL_Fm_Fd_ADDR_SIMPLE,
    LDFADDL_Fm_Fd_ADDR_SIMPLE,
    LDFMAX_Fm_Fd_ADDR_SIMPLE,
    LDFMAXA_Fm_Fd_ADDR_SIMPLE,
    LDFMAXAL_Fm_Fd_ADDR_SIMPLE,
    LDFMAXL_Fm_Fd_ADDR_SIMPLE,
    LDFMAXNM_Fm_Fd_ADDR_SIMPLE,
    LDFMAXNMA_Fm_Fd_ADDR_SIMPLE,
    LDFMAXNMAL_Fm_Fd_ADDR_SIMPLE,
    LDFMAXNML_Fm_Fd_ADDR_SIMPLE,
    LDFMIN_Fm_Fd_ADDR_SIMPLE,
    LDFMINA_Fm_Fd_ADDR_SIMPLE,
    LDFMINAL_Fm_Fd_ADDR_SIMPLE,
    LDFMINL_Fm_Fd_ADDR_SIMPLE,
    LDFMINNM_Fm_Fd_ADDR_SIMPLE,
    LDFMINNMA_Fm_Fd_ADDR_SIMPLE,
    LDFMINNMAL_Fm_Fd_ADDR_SIMPLE,
    LDFMINNML_Fm_Fd_ADDR_SIMPLE,
    LDG_Rt_ADDR_SIMM13,
    LDGM_Rt_ADDR_SIMPLE,
    LDLAR_Rt_ADDR_SIMPLE,
    LDLARB_Rt_ADDR_SIMPLE,
    LDLARH_Rt_ADDR_SIMPLE,
    LDNP_Rt_Rt2_ADDR_SIMM7,
    LDNP_Ft_Ft2_ADDR_SIMM7,
    LDP_Rt_Rt2_ADDR_SIMM7,
    LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    LDP_Ft_Ft2_ADDR_SIMM7,
    LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    LDPSW_Rt_Rt2_ADDR_SIMM7,
    LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S,
    LDR_Rt_ADDR_PCREL19,
    LDR_Rt_ADDR_REGOFF,
    LDR_Rt_ADDR_SIMM9,
    LDR_Rt_ADDR_UIMM12,
    LDR_Ft_ADDR_PCREL19,
    LDR_Ft_ADDR_REGOFF,
    LDR_Ft_ADDR_SIMM9,
    LDR_Ft_ADDR_UIMM12,
    LDRAA_Rt_ADDR_SIMM10,
    LDRAB_Rt_ADDR_SIMM10,
    LDRB_Rt_ADDR_REGOFF,
    LDRB_Rt_ADDR_SIMM9,
    LDRB_Rt_ADDR_UIMM12,
    LDRH_Rt_ADDR_REGOFF,
    LDRH_Rt_ADDR_SIMM9,
    LDRH_Rt_ADDR_UIMM12,
    LDRSB_Rt_ADDR_REGOFF,
    LDRSB_Rt_ADDR_SIMM9,
    LDRSB_Rt_ADDR_UIMM12,
    LDRSH_Rt_ADDR_REGOFF,
    LDRSH_Rt_ADDR_SIMM9,
    LDRSH_Rt_ADDR_UIMM12,
    LDRSW_Rt_ADDR_PCREL19,
    LDRSW_Rt_ADDR_REGOFF,
    LDRSW_Rt_ADDR_SIMM9,
    LDRSW_Rt_ADDR_UIMM12,
    LDSET_Rs_Rt_ADDR_SIMPLE,
    LDSETA_Rs_Rt_ADDR_SIMPLE,
    LDSETAB_Rs_Rt_ADDR_SIMPLE,
    LDSETAH_Rs_Rt_ADDR_SIMPLE,
    LDSETAL_Rs_Rt_ADDR_SIMPLE,
    LDSETALB_Rs_Rt_ADDR_SIMPLE,
    LDSETALH_Rs_Rt_ADDR_SIMPLE,
    LDSETB_Rs_Rt_ADDR_SIMPLE,
    LDSETH_Rs_Rt_ADDR_SIMPLE,
    LDSETL_Rs_Rt_ADDR_SIMPLE,
    LDSETLB_Rs_Rt_ADDR_SIMPLE,
    LDSETLH_Rs_Rt_ADDR_SIMPLE,
    LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    LDSMAX_Rs_Rt_ADDR_SIMPLE,
    LDSMAXA_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXAL_Rs_Rt_ADDR_SIMPLE,
    LDSMAXALB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXALH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXH_Rs_Rt_ADDR_SIMPLE,
    LDSMAXL_Rs_Rt_ADDR_SIMPLE,
    LDSMAXLB_Rs_Rt_ADDR_SIMPLE,
    LDSMAXLH_Rs_Rt_ADDR_SIMPLE,
    LDSMIN_Rs_Rt_ADDR_SIMPLE,
    LDSMINA_Rs_Rt_ADDR_SIMPLE,
    LDSMINAB_Rs_Rt_ADDR_SIMPLE,
    LDSMINAH_Rs_Rt_ADDR_SIMPLE,
    LDSMINAL_Rs_Rt_ADDR_SIMPLE,
    LDSMINALB_Rs_Rt_ADDR_SIMPLE,
    LDSMINALH_Rs_Rt_ADDR_SIMPLE,
    LDSMINB_Rs_Rt_ADDR_SIMPLE,
    LDSMINH_Rs_Rt_ADDR_SIMPLE,
    LDSMINL_Rs_Rt_ADDR_SIMPLE,
    LDSMINLB_Rs_Rt_ADDR_SIMPLE,
    LDSMINLH_Rs_Rt_ADDR_SIMPLE,
    LDTADD_Rs_Rt_ADDR_SIMPLE,
    LDTADDA_Rs_Rt_ADDR_SIMPLE,
    LDTADDAL_Rs_Rt_ADDR_SIMPLE,
    LDTADDL_Rs_Rt_ADDR_SIMPLE,
    LDTCLR_Rs_Rt_ADDR_SIMPLE,
    LDTCLRA_Rs_Rt_ADDR_SIMPLE,
    LDTCLRAL_Rs_Rt_ADDR_SIMPLE,
    LDTCLRL_Rs_Rt_ADDR_SIMPLE,
    LDTNP_Rt_Rt2_ADDR_SIMM7,
    LDTNP_Fd_Fa_ADDR_SIMM7,
    LDTP_Rt_Rt2_ADDR_SIMM7,
    LDTP_Rt_X_Rt2_X_ADDR_SIMM7_S_D,
    LDTP_Fd_Fa_ADDR_SIMM7,
    LDTP_Fd_S_Q_Fa_S_Q_ADDR_SIMM7_S_Q,
    LDTR_Rt_ADDR_SIMM9,
    LDTRB_Rt_ADDR_SIMM9,
    LDTRH_Rt_ADDR_SIMM9,
    LDTRSB_Rt_ADDR_SIMM9,
    LDTRSH_Rt_ADDR_SIMM9,
    LDTRSW_Rt_ADDR_SIMM9,
    LDTSET_Rs_Rt_ADDR_SIMPLE,
    LDTSETA_Rs_Rt_ADDR_SIMPLE,
    LDTSETAL_Rs_Rt_ADDR_SIMPLE,
    LDTSETL_Rs_Rt_ADDR_SIMPLE,
    LDTXR_Rt_ADDR_SIMPLE,
    LDUMAX_Rs_Rt_ADDR_SIMPLE,
    LDUMAXA_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXAL_Rs_Rt_ADDR_SIMPLE,
    LDUMAXALB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXALH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXH_Rs_Rt_ADDR_SIMPLE,
    LDUMAXL_Rs_Rt_ADDR_SIMPLE,
    LDUMAXLB_Rs_Rt_ADDR_SIMPLE,
    LDUMAXLH_Rs_Rt_ADDR_SIMPLE,
    LDUMIN_Rs_Rt_ADDR_SIMPLE,
    LDUMINA_Rs_Rt_ADDR_SIMPLE,
    LDUMINAB_Rs_Rt_ADDR_SIMPLE,
    LDUMINAH_Rs_Rt_ADDR_SIMPLE,
    LDUMINAL_Rs_Rt_ADDR_SIMPLE,
    LDUMINALB_Rs_Rt_ADDR_SIMPLE,
    LDUMINALH_Rs_Rt_ADDR_SIMPLE,
    LDUMINB_Rs_Rt_ADDR_SIMPLE,
    LDUMINH_Rs_Rt_ADDR_SIMPLE,
    LDUMINL_Rs_Rt_ADDR_SIMPLE,
    LDUMINLB_Rs_Rt_ADDR_SIMPLE,
    LDUMINLH_Rs_Rt_ADDR_SIMPLE,
    LDUR_Rt_ADDR_SIMM9,
    LDUR_Ft_ADDR_SIMM9,
    LDURB_Rt_ADDR_SIMM9,
    LDURH_Rt_ADDR_SIMM9,
    LDURSB_Rt_ADDR_SIMM9,
    LDURSH_Rt_ADDR_SIMM9,
    LDURSW_Rt_ADDR_SIMM9,
    LDXP_Rt_Rt2_ADDR_SIMPLE,
    LDXR_Rt_ADDR_SIMPLE,
    LDXRB_Rt_ADDR_SIMPLE,
    LDXRH_Rt_ADDR_SIMPLE,
    ORN_Rd_Rn_Rm_SFT,
    ORR_Rd_SP_Rn_LIMM,
    ORR_Rd_Rn_Rm_SFT,
    PRFM_PRFOP_ADDR_PCREL19,
    PRFM_PRFOP_ADDR_REGOFF,
    PRFM_PRFOP_ADDR_UIMM12,
    PRFUM_PRFOP_ADDR_SIMM9,
    ST2G_Rt_SP_ADDR_SIMM13,
    ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    ST64B_Rt_LS64_ADDR_SIMPLE,
    ST64BV_Rs_Rt_LS64_ADDR_SIMPLE,
    ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE,
    STBFADD_Fm_ADDR_SIMPLE,
    STBFADDL_Fm_ADDR_SIMPLE,
    STBFMAX_Fm_ADDR_SIMPLE,
    STBFMAXL_Fm_ADDR_SIMPLE,
    STBFMAXNM_Fm_ADDR_SIMPLE,
    STBFMAXNML_Fm_ADDR_SIMPLE,
    STBFMIN_Fm_ADDR_SIMPLE,
    STBFMINL_Fm_ADDR_SIMPLE,
    STBFMINNM_Fm_ADDR_SIMPLE,
    STBFMINNML_Fm_ADDR_SIMPLE,
    STFADD_Fm_ADDR_SIMPLE,
    STFADDL_Fm_ADDR_SIMPLE,
    STFMAX_Fm_ADDR_SIMPLE,
    STFMAXL_Fm_ADDR_SIMPLE,
    STFMAXNM_Fm_ADDR_SIMPLE,
    STFMAXNML_Fm_ADDR_SIMPLE,
    STFMIN_Fm_ADDR_SIMPLE,
    STFMINL_Fm_ADDR_SIMPLE,
    STFMINNM_Fm_ADDR_SIMPLE,
    STFMINNML_Fm_ADDR_SIMPLE,
    STG_Rt_SP_ADDR_SIMM13,
    STG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STGM_Rt_ADDR_SIMPLE,
    STGP_Rt_Rt2_ADDR_SIMM11,
    STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag,
    STLLR_Rt_ADDR_SIMPLE,
    STLLRB_Rt_ADDR_SIMPLE,
    STLLRH_Rt_ADDR_SIMPLE,
    STLP_Rt_Rs_ADDR_SIMPLE,
    STLR_Rt_ADDR_SIMPLE,
    STLRB_Rt_ADDR_SIMPLE,
    STLRH_Rt_ADDR_SIMPLE,
    STLTXR_Rs_Rt_ADDR_SIMPLE,
    STLUR_Rt_ADDR_OFFSET,
    STLUR_Rt_X_ADDR_OFFSET,
    STLURB_Rt_ADDR_OFFSET,
    STLURH_Rt_ADDR_OFFSET,
    STLXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    STLXR_Rs_Rt_ADDR_SIMPLE,
    STLXRB_Rs_Rt_ADDR_SIMPLE,
    STLXRH_Rs_Rt_ADDR_SIMPLE,
    STNP_Rt_Rt2_ADDR_SIMM7,
    STNP_Ft_Ft2_ADDR_SIMM7,
    STP_Rt_Rt2_ADDR_SIMM7,
    STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    STP_Ft_Ft2_ADDR_SIMM7,
    STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    STR_Rt_ADDR_REGOFF,
    STR_Rt_ADDR_SIMM9,
    STR_Rt_ADDR_UIMM12,
    STR_Ft_ADDR_REGOFF,
    STR_Ft_ADDR_SIMM9,
    STR_Ft_ADDR_UIMM12,
    STRB_Rt_ADDR_REGOFF,
    STRB_Rt_ADDR_SIMM9,
    STRB_Rt_ADDR_UIMM12,
    STRH_Rt_ADDR_REGOFF,
    STRH_Rt_ADDR_SIMM9,
    STRH_Rt_ADDR_UIMM12,
    STTNP_Rt_Rt2_ADDR_SIMM7,
    STTNP_Fd_Fa_ADDR_SIMM7,
    STTP_Rt_Rt2_ADDR_SIMM7,
    STTP_Rt_X_Rt2_X_ADDR_SIMM7_S_D,
    STTP_Fd_Fa_ADDR_SIMM7,
    STTP_Fd_S_Q_Fa_S_Q_ADDR_SIMM7_S_Q,
    STTR_Rt_ADDR_SIMM9,
    STTRB_Rt_ADDR_SIMM9,
    STTRH_Rt_ADDR_SIMM9,
    STTXR_Rs_Rt_ADDR_SIMPLE,
    STUR_Rt_ADDR_SIMM9,
    STUR_Ft_ADDR_SIMM9,
    STURB_Rt_ADDR_SIMM9,
    STURH_Rt_ADDR_SIMM9,
    STXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    STXR_Rs_Rt_ADDR_SIMPLE,
    STXRB_Rs_Rt_ADDR_SIMPLE,
    STXRH_Rs_Rt_ADDR_SIMPLE,
    STZ2G_Rt_SP_ADDR_SIMM13,
    STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STZG_Rt_SP_ADDR_SIMM13,
    STZG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    STZGM_Rt_ADDR_SIMPLE,
    SWP_Rs_Rt_ADDR_SIMPLE,
    SWPA_Rs_Rt_ADDR_SIMPLE,
    SWPAB_Rs_Rt_ADDR_SIMPLE,
    SWPAH_Rs_Rt_ADDR_SIMPLE,
    SWPAL_Rs_Rt_ADDR_SIMPLE,
    SWPALB_Rs_Rt_ADDR_SIMPLE,
    SWPALH_Rs_Rt_ADDR_SIMPLE,
    SWPB_Rs_Rt_ADDR_SIMPLE,
    SWPH_Rs_Rt_ADDR_SIMPLE,
    SWPL_Rs_Rt_ADDR_SIMPLE,
    SWPLB_Rs_Rt_ADDR_SIMPLE,
    SWPLH_Rs_Rt_ADDR_SIMPLE,
    SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    SWPT_Rs_Rt_ADDR_SIMPLE,
    SWPTA_Rs_Rt_ADDR_SIMPLE,
    SWPTAL_Rs_Rt_ADDR_SIMPLE,
    SWPTL_Rs_Rt_ADDR_SIMPLE,
}
#[doc = r" The identity of each instruction, parallel to INSNS."]
static INSN_IDS: [InsnId; 389] = [
    InsnId::AND_Rd_SP_Rn_LIMM,
    InsnId::AND_Rd_Rn_Rm_SFT,
    InsnId::ANDS_Rd_Rn_LIMM,
    InsnId::ANDS_Rd_Rn_Rm_SFT,
    InsnId::BIC_Rd_Rn_Rm_SFT,
    InsnId::BICS_Rd_Rn_Rm_SFT,
    InsnId::CAS_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASA_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASALT_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASAT_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASB_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASH_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASL_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASLT_Rs_Rt_ADDR_SIMPLE,
    InsnId::CASP_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPA_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPAL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPALT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPAT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPL_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPLT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CASPT_Rs_PAIRREG_Rt_PAIRREG_ADDR_SIMPLE,
    InsnId::CAST_Rs_Rt_ADDR_SIMPLE,
    InsnId::EON_Rd_Rn_Rm_SFT,
    InsnId::EOR_Rd_SP_Rn_LIMM,
    InsnId::EOR_Rd_Rn_Rm_SFT,
    InsnId::LD64B_Rt_LS64_ADDR_SIMPLE,
    InsnId::LDADD_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDADDLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDAP_Rt_Rs_ADDR_SIMPLE,
    InsnId::LDAPP_Rt_Rs_ADDR_SIMPLE,
    InsnId::LDAPR_Rt_ADDR_SIMPLE,
    InsnId::LDAPRB_Rt_ADDR_SIMPLE,
    InsnId::LDAPRH_Rt_ADDR_SIMPLE,
    InsnId::LDAPUR_Rt_ADDR_OFFSET,
    InsnId::LDAPUR_Rt_X_ADDR_OFFSET,
    InsnId::LDAPURB_Rt_ADDR_OFFSET,
    InsnId::LDAPURH_Rt_ADDR_OFFSET,
    InsnId::LDAPURSB_Rt_ADDR_OFFSET,
    InsnId::LDAPURSB_Rt_W_ADDR_OFFSET,
    InsnId::LDAPURSH_Rt_ADDR_OFFSET,
    InsnId::LDAPURSH_Rt_W_ADDR_OFFSET,
    InsnId::LDAPURSW_Rt_ADDR_OFFSET,
    InsnId::LDAR_Rt_ADDR_SIMPLE,
    InsnId::LDARB_Rt_ADDR_SIMPLE,
    InsnId::LDARH_Rt_ADDR_SIMPLE,
    InsnId::LDATXR_Rt_ADDR_SIMPLE,
    InsnId::LDAXP_Rt_Rt2_ADDR_SIMPLE,
    InsnId::LDAXR_Rt_ADDR_SIMPLE,
    InsnId::LDAXRB_Rt_ADDR_SIMPLE,
    InsnId::LDAXRH_Rt_ADDR_SIMPLE,
    InsnId::LDBFADD_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFADDA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFADDAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFADDL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAX_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXNM_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXNMA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXNMAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMAXNML_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMIN_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINNM_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINNMA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINNMAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDBFMINNML_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDCLR_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDCLRP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDCLRPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDCLRPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDCLRPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDEOR_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDEORLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDFADD_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFADDA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFADDAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFADDL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAX_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXNM_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXNMA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXNMAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMAXNML_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMIN_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINNM_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINNMA_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINNMAL_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDFMINNML_Fm_Fd_ADDR_SIMPLE,
    InsnId::LDG_Rt_ADDR_SIMM13,
    InsnId::LDGM_Rt_ADDR_SIMPLE,
    InsnId::LDLAR_Rt_ADDR_SIMPLE,
    InsnId::LDLARB_Rt_ADDR_SIMPLE,
    InsnId::LDLARH_Rt_ADDR_SIMPLE,
    InsnId::LDNP_Rt_Rt2_ADDR_SIMM7,
    InsnId::LDNP_Ft_Ft2_ADDR_SIMM7,
    InsnId::LDP_Rt_Rt2_ADDR_SIMM7,
    InsnId::LDP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    InsnId::LDP_Ft_Ft2_ADDR_SIMM7,
    InsnId::LDP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    InsnId::LDPSW_Rt_Rt2_ADDR_SIMM7,
    InsnId::LDPSW_Rt_X_Rt2_X_ADDR_SIMM7_S_S,
    InsnId::LDR_Rt_ADDR_PCREL19,
    InsnId::LDR_Rt_ADDR_REGOFF,
    InsnId::LDR_Rt_ADDR_SIMM9,
    InsnId::LDR_Rt_ADDR_UIMM12,
    InsnId::LDR_Ft_ADDR_PCREL19,
    InsnId::LDR_Ft_ADDR_REGOFF,
    InsnId::LDR_Ft_ADDR_SIMM9,
    InsnId::LDR_Ft_ADDR_UIMM12,
    InsnId::LDRAA_Rt_ADDR_SIMM10,
    InsnId::LDRAB_Rt_ADDR_SIMM10,
    InsnId::LDRB_Rt_ADDR_REGOFF,
    InsnId::LDRB_Rt_ADDR_SIMM9,
    InsnId::LDRB_Rt_ADDR_UIMM12,
    InsnId::LDRH_Rt_ADDR_REGOFF,
    InsnId::LDRH_Rt_ADDR_SIMM9,
    InsnId::LDRH_Rt_ADDR_UIMM12,
    InsnId::LDRSB_Rt_ADDR_REGOFF,
    InsnId::LDRSB_Rt_ADDR_SIMM9,
    InsnId::LDRSB_Rt_ADDR_UIMM12,
    InsnId::LDRSH_Rt_ADDR_REGOFF,
    InsnId::LDRSH_Rt_ADDR_SIMM9,
    InsnId::LDRSH_Rt_ADDR_UIMM12,
    InsnId::LDRSW_Rt_ADDR_PCREL19,
    InsnId::LDRSW_Rt_ADDR_REGOFF,
    InsnId::LDRSW_Rt_ADDR_SIMM9,
    InsnId::LDRSW_Rt_ADDR_UIMM12,
    InsnId::LDSET_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSETP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDSETPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDSETPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDSETPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::LDSMAX_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMAXLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMIN_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDSMINLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTADD_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTADDA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTADDAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTADDL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTCLR_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTCLRA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTCLRAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTCLRL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTNP_Rt_Rt2_ADDR_SIMM7,
    InsnId::LDTNP_Fd_Fa_ADDR_SIMM7,
    InsnId::LDTP_Rt_Rt2_ADDR_SIMM7,
    InsnId::LDTP_Rt_X_Rt2_X_ADDR_SIMM7_S_D,
    InsnId::LDTP_Fd_Fa_ADDR_SIMM7,
    InsnId::LDTP_Fd_S_Q_Fa_S_Q_ADDR_SIMM7_S_Q,
    InsnId::LDTR_Rt_ADDR_SIMM9,
    InsnId::LDTRB_Rt_ADDR_SIMM9,
    InsnId::LDTRH_Rt_ADDR_SIMM9,
    InsnId::LDTRSB_Rt_ADDR_SIMM9,
    InsnId::LDTRSH_Rt_ADDR_SIMM9,
    InsnId::LDTRSW_Rt_ADDR_SIMM9,
    InsnId::LDTSET_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTSETA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTSETAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTSETL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDTXR_Rt_ADDR_SIMPLE,
    InsnId::LDUMAX_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMAXLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMIN_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINA_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINL_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUMINLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::LDUR_Rt_ADDR_SIMM9,
    InsnId::LDUR_Ft_ADDR_SIMM9,
    InsnId::LDURB_Rt_ADDR_SIMM9,
    InsnId::LDURH_Rt_ADDR_SIMM9,
    InsnId::LDURSB_Rt_ADDR_SIMM9,
    InsnId::LDURSH_Rt_ADDR_SIMM9,
    InsnId::LDURSW_Rt_ADDR_SIMM9,
    InsnId::LDXP_Rt_Rt2_ADDR_SIMPLE,
    InsnId::LDXR_Rt_ADDR_SIMPLE,
    InsnId::LDXRB_Rt_ADDR_SIMPLE,
    InsnId::LDXRH_Rt_ADDR_SIMPLE,
    InsnId::ORN_Rd_Rn_Rm_SFT,
    InsnId::ORR_Rd_SP_Rn_LIMM,
    InsnId::ORR_Rd_Rn_Rm_SFT,
    InsnId::PRFM_PRFOP_ADDR_PCREL19,
    InsnId::PRFM_PRFOP_ADDR_REGOFF,
    InsnId::PRFM_PRFOP_ADDR_UIMM12,
    InsnId::PRFUM_PRFOP_ADDR_SIMM9,
    InsnId::ST2G_Rt_SP_ADDR_SIMM13,
    InsnId::ST2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    InsnId::ST64B_Rt_LS64_ADDR_SIMPLE,
    InsnId::ST64BV_Rs_Rt_LS64_ADDR_SIMPLE,
    InsnId::ST64BV0_Rs_Rt_LS64_ADDR_SIMPLE,
    InsnId::STBFADD_Fm_ADDR_SIMPLE,
    InsnId::STBFADDL_Fm_ADDR_SIMPLE,
    InsnId::STBFMAX_Fm_ADDR_SIMPLE,
    InsnId::STBFMAXL_Fm_ADDR_SIMPLE,
    InsnId::STBFMAXNM_Fm_ADDR_SIMPLE,
    InsnId::STBFMAXNML_Fm_ADDR_SIMPLE,
    InsnId::STBFMIN_Fm_ADDR_SIMPLE,
    InsnId::STBFMINL_Fm_ADDR_SIMPLE,
    InsnId::STBFMINNM_Fm_ADDR_SIMPLE,
    InsnId::STBFMINNML_Fm_ADDR_SIMPLE,
    InsnId::STFADD_Fm_ADDR_SIMPLE,
    InsnId::STFADDL_Fm_ADDR_SIMPLE,
    InsnId::STFMAX_Fm_ADDR_SIMPLE,
    InsnId::STFMAXL_Fm_ADDR_SIMPLE,
    InsnId::STFMAXNM_Fm_ADDR_SIMPLE,
    InsnId::STFMAXNML_Fm_ADDR_SIMPLE,
    InsnId::STFMIN_Fm_ADDR_SIMPLE,
    InsnId::STFMINL_Fm_ADDR_SIMPLE,
    InsnId::STFMINNM_Fm_ADDR_SIMPLE,
    InsnId::STFMINNML_Fm_ADDR_SIMPLE,
    InsnId::STG_Rt_SP_ADDR_SIMM13,
    InsnId::STG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    InsnId::STGM_Rt_ADDR_SIMPLE,
    InsnId::STGP_Rt_Rt2_ADDR_SIMM11,
    InsnId::STGP_Rt_X_Rt2_X_ADDR_SIMM11_imm_tag,
    InsnId::STLLR_Rt_ADDR_SIMPLE,
    InsnId::STLLRB_Rt_ADDR_SIMPLE,
    InsnId::STLLRH_Rt_ADDR_SIMPLE,
    InsnId::STLP_Rt_Rs_ADDR_SIMPLE,
    InsnId::STLR_Rt_ADDR_SIMPLE,
    InsnId::STLRB_Rt_ADDR_SIMPLE,
    InsnId::STLRH_Rt_ADDR_SIMPLE,
    InsnId::STLTXR_Rs_Rt_ADDR_SIMPLE,
    InsnId::STLUR_Rt_ADDR_OFFSET,
    InsnId::STLUR_Rt_X_ADDR_OFFSET,
    InsnId::STLURB_Rt_ADDR_OFFSET,
    InsnId::STLURH_Rt_ADDR_OFFSET,
    InsnId::STLXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    InsnId::STLXR_Rs_Rt_ADDR_SIMPLE,
    InsnId::STLXRB_Rs_Rt_ADDR_SIMPLE,
    InsnId::STLXRH_Rs_Rt_ADDR_SIMPLE,
    InsnId::STNP_Rt_Rt2_ADDR_SIMM7,
    InsnId::STNP_Ft_Ft2_ADDR_SIMM7,
    InsnId::STP_Rt_Rt2_ADDR_SIMM7,
    InsnId::STP_Rt_W_Rt2_W_ADDR_SIMM7_S_S,
    InsnId::STP_Ft_Ft2_ADDR_SIMM7,
    InsnId::STP_Ft_S_S_Ft2_S_S_ADDR_SIMM7_S_S,
    InsnId::STR_Rt_ADDR_REGOFF,
    InsnId::STR_Rt_ADDR_SIMM9,
    InsnId::STR_Rt_ADDR_UIMM12,
    InsnId::STR_Ft_ADDR_REGOFF,
    InsnId::STR_Ft_ADDR_SIMM9,
    InsnId::STR_Ft_ADDR_UIMM12,
    InsnId::STRB_Rt_ADDR_REGOFF,
    InsnId::STRB_Rt_ADDR_SIMM9,
    InsnId::STRB_Rt_ADDR_UIMM12,
    InsnId::STRH_Rt_ADDR_REGOFF,
    InsnId::STRH_Rt_ADDR_SIMM9,
    InsnId::STRH_Rt_ADDR_UIMM12,
    InsnId::STTNP_Rt_Rt2_ADDR_SIMM7,
    InsnId::STTNP_Fd_Fa_ADDR_SIMM7,
    InsnId::STTP_Rt_Rt2_ADDR_SIMM7,
    InsnId::STTP_Rt_X_Rt2_X_ADDR_SIMM7_S_D,
    InsnId::STTP_Fd_Fa_ADDR_SIMM7,
    InsnId::STTP_Fd_S_Q_Fa_S_Q_ADDR_SIMM7_S_Q,
    InsnId::STTR_Rt_ADDR_SIMM9,
    InsnId::STTRB_Rt_ADDR_SIMM9,
    InsnId::STTRH_Rt_ADDR_SIMM9,
    InsnId::STTXR_Rs_Rt_ADDR_SIMPLE,
    InsnId::STUR_Rt_ADDR_SIMM9,
    InsnId::STUR_Ft_ADDR_SIMM9,
    InsnId::STURB_Rt_ADDR_SIMM9,
    InsnId::STURH_Rt_ADDR_SIMM9,
    InsnId::STXP_Rs_Rt_Rt2_ADDR_SIMPLE,
    InsnId::STXR_Rs_Rt_ADDR_SIMPLE,
    InsnId::STXRB_Rs_Rt_ADDR_SIMPLE,
    InsnId::STXRH_Rs_Rt_ADDR_SIMPLE,
    InsnId::STZ2G_Rt_SP_ADDR_SIMM13,
    InsnId::STZ2G_Rt_SP_X_ADDR_SIMM13_imm_tag,
    InsnId::STZG_Rt_SP_ADDR_SIMM13,
    InsnId::STZG_Rt_SP_X_ADDR_SIMM13_imm_tag,
    InsnId::STZGM_Rt_ADDR_SIMPLE,
    InsnId::SWP_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPA_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPAB_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPAH_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPALB_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPALH_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPB_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPH_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPL_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPLB_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPLH_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPP_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::SWPPA_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::SWPPAL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::SWPPL_LSE128_Rt_LSE128_Rt2_ADDR_SIMPLE,
    InsnId::SWPT_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPTA_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPTAL_Rs_Rt_ADDR_SIMPLE,
    InsnId::SWPTL_Rs_Rt_ADDR_SIMPLE,
];
#[doc = r" The decoded instruction definitions, indexed by InsnId."]
static INSNS: [Insn; 389] = [
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
        mnemonic: "casalt",
        aliases: &[],
        opcode: 0xc9c0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_5,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casat",
        aliases: &[],
        opcode: 0xc9c07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_5,
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
        mnemonic: "caslt",
        aliases: &[],
        opcode: 0xc980fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_5,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "casp",
        aliases: &[],
        opcode: 0x8207c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_6,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspa",
        aliases: &[],
        opcode: 0x8607c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_6,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspal",
        aliases: &[],
        opcode: 0x860fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_6,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "caspalt",
        aliases: &[],
        opcode: 0x49c0fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_7,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "caspat",
        aliases: &[],
        opcode: 0x49c07c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_7,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "caspl",
        aliases: &[],
        opcode: 0x820fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSE,
        operands: OPERANDS_6,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "casplt",
        aliases: &[],
        opcode: 0x4980fc00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_7,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "caspt",
        aliases: &[],
        opcode: 0x49807c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_7,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "cast",
        aliases: &[],
        opcode: 0xc9807c00,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_5,
        flags: InsnFlags::empty(),
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
        operands: OPERANDS_8,
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
        mnemonic: "ldap",
        aliases: &[],
        opcode: 0xd9405800,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSCP,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapp",
        aliases: &[],
        opcode: 0xd9407800,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSCP,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapr",
        aliases: &[],
        opcode: 0xb8bfc000,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaprb",
        aliases: &[],
        opcode: 0x38bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaprh",
        aliases: &[],
        opcode: 0x78bfc000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::RCPC,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0x99400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapur",
        aliases: &[],
        opcode: 0xd9400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapurb",
        aliases: &[],
        opcode: 0x19400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapurh",
        aliases: &[],
        opcode: 0x59400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursb",
        aliases: &[],
        opcode: 0x19c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursh",
        aliases: &[],
        opcode: 0x59c00000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldapursw",
        aliases: &[],
        opcode: 0x99800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldar",
        aliases: &[],
        opcode: 0x88dffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldarb",
        aliases: &[],
        opcode: 0x8dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldarh",
        aliases: &[],
        opcode: 0x48dffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldatxr",
        aliases: &[],
        opcode: 0x895ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaxp",
        aliases: &[],
        opcode: 0x887f8000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_14,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaxr",
        aliases: &[],
        opcode: 0x885ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldaxrb",
        aliases: &[],
        opcode: 0x85ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldaxrh",
        aliases: &[],
        opcode: 0x485ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfadd",
        aliases: &[],
        opcode: 0x3c200000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfadda",
        aliases: &[],
        opcode: 0x3ca00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfaddal",
        aliases: &[],
        opcode: 0x3ce00000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfaddl",
        aliases: &[],
        opcode: 0x3c600000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmax",
        aliases: &[],
        opcode: 0x3c204000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxa",
        aliases: &[],
        opcode: 0x3ca04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxal",
        aliases: &[],
        opcode: 0x3ce04000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxl",
        aliases: &[],
        opcode: 0x3c604000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxnm",
        aliases: &[],
        opcode: 0x3c206000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxnma",
        aliases: &[],
        opcode: 0x3ca06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxnmal",
        aliases: &[],
        opcode: 0x3ce06000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmaxnml",
        aliases: &[],
        opcode: 0x3c606000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmin",
        aliases: &[],
        opcode: 0x3c205000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfmina",
        aliases: &[],
        opcode: 0x3ca05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminal",
        aliases: &[],
        opcode: 0x3ce05000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminl",
        aliases: &[],
        opcode: 0x3c605000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminnm",
        aliases: &[],
        opcode: 0x3c207000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminnma",
        aliases: &[],
        opcode: 0x3ca07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminnmal",
        aliases: &[],
        opcode: 0x3ce07000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldbfminnml",
        aliases: &[],
        opcode: 0x3c607000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_15,
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
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpa",
        aliases: &[],
        opcode: 0x19a01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpal",
        aliases: &[],
        opcode: 0x19e01000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldclrpl",
        aliases: &[],
        opcode: 0x19601000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
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
        mnemonic: "ldfadd",
        aliases: &[],
        opcode: 0x3c200000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfadda",
        aliases: &[],
        opcode: 0x3ca00000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfaddal",
        aliases: &[],
        opcode: 0x3ce00000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfaddl",
        aliases: &[],
        opcode: 0x3c600000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmax",
        aliases: &[],
        opcode: 0x3c204000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxa",
        aliases: &[],
        opcode: 0x3ca04000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxal",
        aliases: &[],
        opcode: 0x3ce04000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxl",
        aliases: &[],
        opcode: 0x3c604000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxnm",
        aliases: &[],
        opcode: 0x3c206000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxnma",
        aliases: &[],
        opcode: 0x3ca06000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxnmal",
        aliases: &[],
        opcode: 0x3ce06000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmaxnml",
        aliases: &[],
        opcode: 0x3c606000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmin",
        aliases: &[],
        opcode: 0x3c205000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfmina",
        aliases: &[],
        opcode: 0x3ca05000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminal",
        aliases: &[],
        opcode: 0x3ce05000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminl",
        aliases: &[],
        opcode: 0x3c605000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminnm",
        aliases: &[],
        opcode: 0x3c207000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminnma",
        aliases: &[],
        opcode: 0x3ca07000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminnmal",
        aliases: &[],
        opcode: 0x3ce07000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldfminnml",
        aliases: &[],
        opcode: 0x3c607000,
        mask: 0x3fe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_17,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "ldg",
        aliases: &[],
        opcode: 0xd9600000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_18,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldgm",
        aliases: &[],
        opcode: 0xd9e00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_19,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldlar",
        aliases: &[],
        opcode: 0x88df7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldlarb",
        aliases: &[],
        opcode: 0x8df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldlarh",
        aliases: &[],
        opcode: 0x48df7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x28400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldnp",
        aliases: &[],
        opcode: 0x2c400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x29400000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x28c00000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2d400000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldp",
        aliases: &[],
        opcode: 0x2cc00000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x69400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_22,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldpsw",
        aliases: &[],
        opcode: 0x68c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_22,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x18000000,
        mask: 0xbf000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_23,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8600800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb8400400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0xb9400000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_26,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x1c000000,
        mask: 0x3f000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_27,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c600800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3c400400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldr",
        aliases: &[],
        opcode: 0x3d400000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_30,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldraa",
        aliases: &[],
        opcode: 0xf8200400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAUTH,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrab",
        aliases: &[],
        opcode: 0xf8a00400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM10,
        feature_set: InsnFeatureSet::PAUTH,
        operands: OPERANDS_31,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_32,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x38400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrb",
        aliases: &[],
        opcode: 0x39400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_34,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_35,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x78400400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrh",
        aliases: &[],
        opcode: 0x79400000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_37,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_38,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x38800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_39,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsb",
        aliases: &[],
        opcode: 0x39800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_40,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78a00800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_41,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x78800400,
        mask: 0xffa00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_42,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsh",
        aliases: &[],
        opcode: 0x79800000,
        mask: 0xff800000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_43,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0x98000000,
        mask: 0xff000000,
        class: InsnClass::LOADLIT,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_44,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_45,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb8800400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_46,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldrsw",
        aliases: &[],
        opcode: 0xb9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_47,
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
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpa",
        aliases: &[],
        opcode: 0x19a03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpal",
        aliases: &[],
        opcode: 0x19e03000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldsetpl",
        aliases: &[],
        opcode: 0x19603000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
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
        mnemonic: "ldtadd",
        aliases: &[],
        opcode: 0x19200400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtadda",
        aliases: &[],
        opcode: 0x19a00400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtaddal",
        aliases: &[],
        opcode: 0x19e00400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtaddl",
        aliases: &[],
        opcode: 0x19600400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtclr",
        aliases: &[],
        opcode: 0x19201400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtclra",
        aliases: &[],
        opcode: 0x19a01400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtclral",
        aliases: &[],
        opcode: 0x19e01400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtclrl",
        aliases: &[],
        opcode: 0x19601400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtnp",
        aliases: &[],
        opcode: 0xe8400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtnp",
        aliases: &[],
        opcode: 0xec400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtp",
        aliases: &[],
        opcode: 0xe9400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtp",
        aliases: &[],
        opcode: 0xe8c00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtp",
        aliases: &[],
        opcode: 0xed400000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtp",
        aliases: &[],
        opcode: 0xecc00000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtr",
        aliases: &[],
        opcode: 0xb8400800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldtrb",
        aliases: &[],
        opcode: 0x38400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtrh",
        aliases: &[],
        opcode: 0x78400800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtrsb",
        aliases: &[],
        opcode: 0x38800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_39,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldtrsh",
        aliases: &[],
        opcode: 0x78800800,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_42,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldtrsw",
        aliases: &[],
        opcode: 0xb8800800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_46,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldtset",
        aliases: &[],
        opcode: 0x19203400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtseta",
        aliases: &[],
        opcode: 0x19a03400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtsetal",
        aliases: &[],
        opcode: 0x19e03400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "ldtsetl",
        aliases: &[],
        opcode: 0x19603400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(72u64),
    },
    Insn {
        mnemonic: "ldtxr",
        aliases: &[],
        opcode: 0x895f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
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
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldur",
        aliases: &[],
        opcode: 0x3c400000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldurb",
        aliases: &[],
        opcode: 0x38400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldurh",
        aliases: &[],
        opcode: 0x78400000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldursb",
        aliases: &[],
        opcode: 0x38800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_39,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldursh",
        aliases: &[],
        opcode: 0x78800000,
        mask: 0xffa00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_42,
        flags: InsnFlags::const_from_bits(32u64),
    },
    Insn {
        mnemonic: "ldursw",
        aliases: &[],
        opcode: 0xb8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_46,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldxp",
        aliases: &[],
        opcode: 0x887f0000,
        mask: 0xbfff8000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_14,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldxr",
        aliases: &[],
        opcode: 0x885f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "ldxrb",
        aliases: &[],
        opcode: 0x85f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "ldxrh",
        aliases: &[],
        opcode: 0x485f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
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
        operands: OPERANDS_50,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf8a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_51,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfm",
        aliases: &[],
        opcode: 0xf9800000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_52,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "prfum",
        aliases: &[],
        opcode: 0xf8800000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_53,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st2g",
        aliases: &[],
        opcode: 0xd9a00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64b",
        aliases: &[],
        opcode: 0xf83f9000,
        mask: 0xfffffc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_8,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64bv",
        aliases: &[],
        opcode: 0xf820b000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_55,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "st64bv0",
        aliases: &[],
        opcode: 0xf820a000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LS64,
        operands: OPERANDS_55,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfadd",
        aliases: &[],
        opcode: 0x3c20801f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfaddl",
        aliases: &[],
        opcode: 0x3c60801f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfmax",
        aliases: &[],
        opcode: 0x3c20c01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfmaxl",
        aliases: &[],
        opcode: 0x3c60c01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfmaxnm",
        aliases: &[],
        opcode: 0x3c20e01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfmaxnml",
        aliases: &[],
        opcode: 0x3c60e01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfmin",
        aliases: &[],
        opcode: 0x3c20d01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfminl",
        aliases: &[],
        opcode: 0x3c60d01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfminnm",
        aliases: &[],
        opcode: 0x3c20f01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stbfminnml",
        aliases: &[],
        opcode: 0x3c60f01f,
        mask: 0xffe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_56,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stfadd",
        aliases: &[],
        opcode: 0x3c20801f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfaddl",
        aliases: &[],
        opcode: 0x3c60801f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfmax",
        aliases: &[],
        opcode: 0x3c20c01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfmaxl",
        aliases: &[],
        opcode: 0x3c60c01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfmaxnm",
        aliases: &[],
        opcode: 0x3c20e01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfmaxnml",
        aliases: &[],
        opcode: 0x3c60e01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfmin",
        aliases: &[],
        opcode: 0x3c20d01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfminl",
        aliases: &[],
        opcode: 0x3c60d01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfminnm",
        aliases: &[],
        opcode: 0x3c20f01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stfminnml",
        aliases: &[],
        opcode: 0x3c60f01f,
        mask: 0x3fe0fc1f,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSFE,
        operands: OPERANDS_57,
        flags: InsnFlags::const_from_bits(67108864u64),
    },
    Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stg",
        aliases: &[],
        opcode: 0xd9200400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgm",
        aliases: &[],
        opcode: 0xd9a00000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_19,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x69000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_58,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stgp",
        aliases: &[],
        opcode: 0x68800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_58,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stllr",
        aliases: &[],
        opcode: 0x889f7c00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stllrb",
        aliases: &[],
        opcode: 0x89f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stllrh",
        aliases: &[],
        opcode: 0x489f7c00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LOR,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlp",
        aliases: &[],
        opcode: 0xd9005800,
        mask: 0xffe0fc00,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSCP,
        operands: OPERANDS_9,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlr",
        aliases: &[],
        opcode: 0x889ffc00,
        mask: 0xbffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlrb",
        aliases: &[],
        opcode: 0x89ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlrh",
        aliases: &[],
        opcode: 0x489ffc00,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stltxr",
        aliases: &[],
        opcode: 0x8900fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_59,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0x99000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlur",
        aliases: &[],
        opcode: 0xd9000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_13,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlurb",
        aliases: &[],
        opcode: 0x19000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlurh",
        aliases: &[],
        opcode: 0x59000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::RCPC2,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stlxp",
        aliases: &[],
        opcode: 0x88208000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_60,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stlxr",
        aliases: &[],
        opcode: 0x8800fc00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_59,
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
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stnp",
        aliases: &[],
        opcode: 0x2c000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x29000000,
        mask: 0x7fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x28800000,
        mask: 0x7ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_20,
        flags: InsnFlags::const_from_bits(131072u64),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2d000000,
        mask: 0x3fc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stp",
        aliases: &[],
        opcode: 0x2c800000,
        mask: 0x3ec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_21,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8200800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_24,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb8000400,
        mask: 0xbfe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0xb9000000,
        mask: 0xbfc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_26,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c200800,
        mask: 0x3f600c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_28,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3c000400,
        mask: 0x3f600400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "str",
        aliases: &[],
        opcode: 0x3d000000,
        mask: 0x3f400000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_30,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_32,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x38000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strb",
        aliases: &[],
        opcode: 0x39000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_34,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78200800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_REGOFF,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_35,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x78000400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "strh",
        aliases: &[],
        opcode: 0x79000000,
        mask: 0xffc00000,
        class: InsnClass::LDST_POS,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_37,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttnp",
        aliases: &[],
        opcode: 0xe8000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttnp",
        aliases: &[],
        opcode: 0xec000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTNAPAIR_OFFS,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttp",
        aliases: &[],
        opcode: 0xe9000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttp",
        aliases: &[],
        opcode: 0xe8800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_48,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttp",
        aliases: &[],
        opcode: 0xed000000,
        mask: 0xffc00000,
        class: InsnClass::LDSTPAIR_OFF,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttp",
        aliases: &[],
        opcode: 0xec800000,
        mask: 0xfec00000,
        class: InsnClass::LDSTPAIR_INDEXED,
        feature_set: InsnFeatureSet::LSUI_FP,
        operands: OPERANDS_49,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttr",
        aliases: &[],
        opcode: 0xb8000800,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "sttrb",
        aliases: &[],
        opcode: 0x38000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttrh",
        aliases: &[],
        opcode: 0x78000800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNPRIV,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sttxr",
        aliases: &[],
        opcode: 0x89007c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_59,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0xb8000000,
        mask: 0xbfe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_25,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stur",
        aliases: &[],
        opcode: 0x3c000000,
        mask: 0x3f600c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_29,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sturb",
        aliases: &[],
        opcode: 0x38000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_33,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sturh",
        aliases: &[],
        opcode: 0x78000000,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_36,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stxp",
        aliases: &[],
        opcode: 0x88200000,
        mask: 0xbfe08000,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_60,
        flags: InsnFlags::const_from_bits(2u64),
    },
    Insn {
        mnemonic: "stxr",
        aliases: &[],
        opcode: 0x88007c00,
        mask: 0xbfe0fc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_59,
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
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stz2g",
        aliases: &[],
        opcode: 0xd9e00400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600800,
        mask: 0xffe00c00,
        class: InsnClass::LDST_UNSCALED,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stzg",
        aliases: &[],
        opcode: 0xd9600400,
        mask: 0xffe00400,
        class: InsnClass::LDST_IMM9,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_54,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "stzgm",
        aliases: &[],
        opcode: 0xd9200000,
        mask: 0xfffffc00,
        class: InsnClass::LDSTEXCL,
        feature_set: InsnFeatureSet::MEMTAG,
        operands: OPERANDS_19,
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
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppa",
        aliases: &[],
        opcode: 0x19a08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppal",
        aliases: &[],
        opcode: 0x19e08000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swppl",
        aliases: &[],
        opcode: 0x19608000,
        mask: 0xffe0fc00,
        class: InsnClass::LSE128_ATOMIC,
        feature_set: InsnFeatureSet::LSE128,
        operands: OPERANDS_16,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "swpt",
        aliases: &[],
        opcode: 0x19208400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swpta",
        aliases: &[],
        opcode: 0x19a08400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swptal",
        aliases: &[],
        opcode: 0x19e08400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
    Insn {
        mnemonic: "swptl",
        aliases: &[],
        opcode: 0x19608400,
        mask: 0xbfe0fc00,
        class: InsnClass::LSE_ATOMIC,
        feature_set: InsnFeatureSet::LSUI,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(64u64),
    },
];
#[doc = r" Return the index of the matching instruction in INSNS, or -1."]
fn decode_index(insn: u32) -> i32 {
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
                                                    return 362;
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x48007c00 {
                                                    return 363;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x88007c00 {
                                                return 361;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8207c00 {
                                                return 21;
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88200000 {
                                                return 360;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe0fc00 == 0x800fc00 {
                                                    return 326;
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0x4800fc00 {
                                                    return 327;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe0fc00 == 0x8800fc00 {
                                                return 325;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x820fc00 {
                                                return 26;
                                            }
                                        } else {
                                            if insn & 0xbfe08000 == 0x88208000 {
                                                return 324;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fc00000 == 0x28000000 {
                                        return 328;
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xe8000000 {
                                        return 346;
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0xbfe0fc00 == 0x89007c00 {
                                        return 355;
                                    }
                                } else {
                                    if insn & 0xbfe0fc00 == 0x8900fc00 {
                                        return 319;
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fc00000 == 0x29000000 {
                                        return 330;
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0xffc00000 == 0x69000000 {
                                            return 310;
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0xe9000000 {
                                            return 348;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x89f7c00 {
                                                    return 313;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x489f7c00 {
                                                    return 314;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x889f7c00 {
                                                return 312;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x49807c00 {
                                                return 28;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0xc9807c00 {
                                                return 29;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a07c00 {
                                                return 15;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a07c00 {
                                                return 16;
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a07c00 {
                                            return 6;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x89ffc00 {
                                                    return 317;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x489ffc00 {
                                                    return 318;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x889ffc00 {
                                                return 316;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x4980fc00 {
                                                return 27;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0xc980fc00 {
                                                return 20;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8a0fc00 {
                                                return 18;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48a0fc00 {
                                                return 19;
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88a0fc00 {
                                            return 17;
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28800000 {
                                    return 331;
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xfec00000 == 0x68800000 {
                                        return 311;
                                    }
                                } else {
                                    if insn & 0xfec00000 == 0xe8800000 {
                                        return 349;
                                    }
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
                                                    return 273;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485f7c00 {
                                                    return 274;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885f7c00 {
                                                return 272;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x8607c00 {
                                                return 22;
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f0000 {
                                                return 271;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x200000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x85ffc00 {
                                                    return 66;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x485ffc00 {
                                                    return 67;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x885ffc00 {
                                                return 65;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xbfe0fc00 == 0x860fc00 {
                                                return 23;
                                            }
                                        } else {
                                            if insn & 0xbfff8000 == 0x887f8000 {
                                                return 64;
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fc00000 == 0x28400000 {
                                        return 141;
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xe8400000 {
                                        return 223;
                                    }
                                }
                            }
                        } else {
                            if insn & 0x20000000 == 0 {
                                if insn & 0x008000 == 0 {
                                    if insn & 0xbffffc00 == 0x895f7c00 {
                                        return 239;
                                    }
                                } else {
                                    if insn & 0xbffffc00 == 0x895ffc00 {
                                        return 63;
                                    }
                                }
                            } else {
                                if insn & 0x40000000 == 0 {
                                    if insn & 0x7fc00000 == 0x29400000 {
                                        return 143;
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0xffc00000 == 0x69400000 {
                                            return 147;
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0xe9400000 {
                                            return 225;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if insn & 0x20000000 == 0 {
                            if insn & 0x008000 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x8df7c00 {
                                                    return 139;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x48df7c00 {
                                                    return 140;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x88df7c00 {
                                                return 138;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x49c07c00 {
                                                return 25;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0xc9c07c00 {
                                                return 14;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e07c00 {
                                                return 8;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e07c00 {
                                                return 9;
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e07c00 {
                                            return 7;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x1000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xfffffc00 == 0x8dffc00 {
                                                    return 61;
                                                }
                                            } else {
                                                if insn & 0xfffffc00 == 0x48dffc00 {
                                                    return 62;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbffffc00 == 0x88dffc00 {
                                                return 60;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x49c0fc00 {
                                                return 24;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0xc9c0fc00 {
                                                return 13;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0xffe0fc00 == 0x8e0fc00 {
                                                return 11;
                                            }
                                        } else {
                                            if insn & 0xffe0fc00 == 0x48e0fc00 {
                                                return 12;
                                            }
                                        }
                                    } else {
                                        if insn & 0xbfe0fc00 == 0x88e0fc00 {
                                            return 10;
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x7ec00000 == 0x28c00000 {
                                    return 144;
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xfec00000 == 0x68c00000 {
                                        return 148;
                                    }
                                } else {
                                    if insn & 0xfec00000 == 0xe8c00000 {
                                        return 226;
                                    }
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
                                return 149;
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0xff000000 == 0x98000000 {
                                    return 171;
                                }
                            } else {
                                if insn & 0xff000000 == 0xd8000000 {
                                    return 278;
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
                                                        return 358;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000000 {
                                                        return 359;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000000 {
                                                    return 356;
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400000 {
                                                        return 266;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400000 {
                                                        return 267;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400000 {
                                                    return 264;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800000 {
                                                    return 268;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800000 {
                                                    return 270;
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78800000 {
                                                    return 269;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8800000 {
                                                    return 281;
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
                                                                        return 41;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78200000
                                                                    {
                                                                        return 42;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8200000 {
                                                                    return 34;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a00000
                                                                    {
                                                                        return 36;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a00000
                                                                    {
                                                                        return 37;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a00000 {
                                                                    return 35;
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
                                                                        return 44;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78600000
                                                                    {
                                                                        return 45;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8600000 {
                                                                    return 43;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e00000
                                                                    {
                                                                        return 39;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e00000
                                                                    {
                                                                        return 40;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e00000 {
                                                                    return 38;
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
                                                                        return 376;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78208000
                                                                    {
                                                                        return 377;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8208000 {
                                                                    return 369;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a08000
                                                                    {
                                                                        return 371;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a08000
                                                                    {
                                                                        return 372;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a08000 {
                                                                    return 370;
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
                                                                        return 379;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78608000
                                                                    {
                                                                        return 380;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8608000 {
                                                                    return 378;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e08000
                                                                    {
                                                                        return 374;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e08000
                                                                    {
                                                                        return 375;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e08000 {
                                                                    return 373;
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
                                                                        return 198;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78204000
                                                                    {
                                                                        return 199;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8204000 {
                                                                    return 191;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a04000
                                                                    {
                                                                        return 193;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a04000
                                                                    {
                                                                        return 194;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a04000 {
                                                                    return 192;
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
                                                                        return 201;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78604000
                                                                    {
                                                                        return 202;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8604000 {
                                                                    return 200;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e04000
                                                                    {
                                                                        return 196;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e04000
                                                                    {
                                                                        return 197;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e04000 {
                                                                    return 195;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x80000000 == 0 {
                                                        if insn & 0x40000000 == 0 {
                                                            if insn & 0xfffffc00 == 0x38bfc000 {
                                                                return 49;
                                                            }
                                                        } else {
                                                            if insn & 0xfffffc00 == 0x78bfc000 {
                                                                return 50;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0xbffffc00 == 0xb8bfc000 {
                                                            return 48;
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
                                                                        return 111;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78202000
                                                                    {
                                                                        return 112;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8202000 {
                                                                    return 104;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a02000
                                                                    {
                                                                        return 106;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a02000
                                                                    {
                                                                        return 107;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a02000 {
                                                                    return 105;
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
                                                                        return 114;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78602000
                                                                    {
                                                                        return 115;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8602000 {
                                                                    return 113;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e02000
                                                                    {
                                                                        return 109;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e02000
                                                                    {
                                                                        return 110;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e02000 {
                                                                    return 108;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820a000 {
                                                        return 286;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38206000 {
                                                                    return 247;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78206000 {
                                                                    return 248;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8206000 {
                                                                return 240;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a06000 {
                                                                    return 242;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a06000 {
                                                                    return 243;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a06000 {
                                                                return 241;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38606000 {
                                                                    return 250;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78606000 {
                                                                    return 251;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8606000 {
                                                                return 249;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e06000 {
                                                                    return 245;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e06000 {
                                                                    return 246;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e06000 {
                                                                return 244;
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
                                                                        return 95;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78201000
                                                                    {
                                                                        return 96;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8201000 {
                                                                    return 88;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a01000
                                                                    {
                                                                        return 90;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a01000
                                                                    {
                                                                        return 91;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a01000 {
                                                                    return 89;
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
                                                                        return 98;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78601000
                                                                    {
                                                                        return 99;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8601000 {
                                                                    return 97;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e01000
                                                                    {
                                                                        return 93;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e01000
                                                                    {
                                                                        return 94;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e01000 {
                                                                    return 92;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83f9000 {
                                                        return 284;
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
                                                                        return 210;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78205000
                                                                    {
                                                                        return 211;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8205000 {
                                                                    return 203;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a05000
                                                                    {
                                                                        return 205;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a05000
                                                                    {
                                                                        return 206;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a05000 {
                                                                    return 204;
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
                                                                        return 213;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78605000
                                                                    {
                                                                        return 214;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8605000 {
                                                                    return 212;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e05000
                                                                    {
                                                                        return 208;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e05000
                                                                    {
                                                                        return 209;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e05000 {
                                                                    return 207;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xf83fd000 {
                                                        return 33;
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
                                                                        return 182;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78203000
                                                                    {
                                                                        return 183;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8203000 {
                                                                    return 175;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38a03000
                                                                    {
                                                                        return 177;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78a03000
                                                                    {
                                                                        return 178;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8a03000 {
                                                                    return 176;
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
                                                                        return 185;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78603000
                                                                    {
                                                                        return 186;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8603000 {
                                                                    return 184;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0x80000000 == 0 {
                                                                if insn & 0x40000000 == 0 {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x38e03000
                                                                    {
                                                                        return 180;
                                                                    }
                                                                } else {
                                                                    if insn & 0xffe0fc00
                                                                        == 0x78e03000
                                                                    {
                                                                        return 181;
                                                                    }
                                                                }
                                                            } else {
                                                                if insn & 0xbfe0fc00 == 0xb8e03000 {
                                                                    return 179;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0xf820b000 {
                                                        return 285;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38207000 {
                                                                    return 259;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78207000 {
                                                                    return 260;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8207000 {
                                                                return 252;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38a07000 {
                                                                    return 254;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78a07000 {
                                                                    return 255;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8a07000 {
                                                                return 253;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38607000 {
                                                                    return 262;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78607000 {
                                                                    return 263;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8607000 {
                                                                return 261;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x80000000 == 0 {
                                                            if insn & 0x40000000 == 0 {
                                                                if insn & 0xffe0fc00 == 0x38e07000 {
                                                                    return 257;
                                                                }
                                                            } else {
                                                                if insn & 0xffe0fc00 == 0x78e07000 {
                                                                    return 258;
                                                                }
                                                            }
                                                        } else {
                                                            if insn & 0xbfe0fc00 == 0xb8e07000 {
                                                                return 256;
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
                                                        return 353;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78000800 {
                                                        return 354;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8000800 {
                                                    return 352;
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38400800 {
                                                        return 230;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78400800 {
                                                        return 231;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8400800 {
                                                    return 229;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38800800 {
                                                    return 232;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8800800 {
                                                    return 234;
                                                }
                                            }
                                        } else {
                                            if insn & 0xffa00c00 == 0x78800800 {
                                                return 233;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x800000 == 0 {
                                        if insn & 0x400000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38200800 {
                                                        return 340;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78200800 {
                                                        return 343;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8200800 {
                                                    return 334;
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0x40000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x38600800 {
                                                        return 159;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x78600800 {
                                                        return 162;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xbfe00c00 == 0xb8600800 {
                                                    return 150;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x40000000 == 0 {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x38a00800 {
                                                    return 165;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xb8a00800 {
                                                    return 172;
                                                }
                                            }
                                        } else {
                                            if insn & 0x80000000 == 0 {
                                                if insn & 0xffa00c00 == 0x78a00800 {
                                                    return 168;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xf8a00800 {
                                                    return 279;
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
                                                    return 341;
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78000400 {
                                                    return 344;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8000400 {
                                                return 335;
                                            }
                                        }
                                    } else {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00400 == 0x38400400 {
                                                    return 160;
                                                }
                                            } else {
                                                if insn & 0xffe00400 == 0x78400400 {
                                                    return 163;
                                                }
                                            }
                                        } else {
                                            if insn & 0xbfe00400 == 0xb8400400 {
                                                return 151;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0x80000000 == 0 {
                                            if insn & 0xffa00400 == 0x38800400 {
                                                return 166;
                                            }
                                        } else {
                                            if insn & 0xffe00400 == 0xb8800400 {
                                                return 173;
                                            }
                                        }
                                    } else {
                                        if insn & 0xffa00400 == 0x78800400 {
                                            return 169;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x800000 == 0 {
                                    if insn & 0xffa00400 == 0xf8200400 {
                                        return 157;
                                    }
                                } else {
                                    if insn & 0xffa00400 == 0xf8a00400 {
                                        return 158;
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
                                                        return 322;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99000000 {
                                                        return 320;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59000000 {
                                                        return 323;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9000000 {
                                                        return 321;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x001000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9200000 {
                                                        return 368;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19208000 {
                                                        return 381;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x002000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19201000 {
                                                        return 100;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19203000 {
                                                        return 187;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0xffe0fc00 == 0xd9005800 {
                                                return 315;
                                            }
                                        } else {
                                            if insn & 0xffe00c00 == 0xd9200800 {
                                                return 307;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19200400 {
                                                    return 215;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19208400 {
                                                    return 385;
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19201400 {
                                                    return 219;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19203400 {
                                                    return 235;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9200400 {
                                            return 308;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39000000 {
                                            return 342;
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79000000 {
                                            return 345;
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9000000 {
                                        return 336;
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
                                                        return 53;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99400000 {
                                                        return 51;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x80000000 == 0 {
                                                    if insn & 0xffe00c00 == 0x59400000 {
                                                        return 54;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0xd9400000 {
                                                        return 52;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0x001000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19608000 {
                                                        return 384;
                                                    }
                                                } else {
                                                    if insn & 0x002000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x19601000 {
                                                            return 103;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x19603000 {
                                                            return 190;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0xd9600000 {
                                                    return 136;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x200000 == 0 {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xffe0fc00 == 0xd9405800 {
                                                    return 46;
                                                }
                                            } else {
                                                if insn & 0xffe0fc00 == 0xd9407800 {
                                                    return 47;
                                                }
                                            }
                                        } else {
                                            if insn & 0xffe00c00 == 0xd9600800 {
                                                return 366;
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19600400 {
                                                    return 218;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19608400 {
                                                    return 388;
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19601400 {
                                                    return 222;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19603400 {
                                                    return 238;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9600400 {
                                            return 367;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0x40000000 == 0 {
                                        if insn & 0xffc00000 == 0x39400000 {
                                            return 161;
                                        }
                                    } else {
                                        if insn & 0xffc00000 == 0x79400000 {
                                            return 164;
                                        }
                                    }
                                } else {
                                    if insn & 0xbfc00000 == 0xb9400000 {
                                        return 152;
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
                                                        return 55;
                                                    }
                                                } else {
                                                    if insn & 0xffe00c00 == 0x99800000 {
                                                        return 59;
                                                    }
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59800000 {
                                                    return 57;
                                                }
                                            }
                                        } else {
                                            if insn & 0x40000000 == 0 {
                                                if insn & 0xffe00c00 == 0x19c00000 {
                                                    return 56;
                                                }
                                            } else {
                                                if insn & 0xffe00c00 == 0x59c00000 {
                                                    return 58;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xfffffc00 == 0xd9a00000 {
                                                        return 309;
                                                    }
                                                } else {
                                                    if insn & 0xfffffc00 == 0xd9e00000 {
                                                        return 137;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a08000 {
                                                        return 382;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e08000 {
                                                        return 383;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a01000 {
                                                        return 101;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e01000 {
                                                        return 102;
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc00 == 0x19a03000 {
                                                        return 188;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc00 == 0x19e03000 {
                                                        return 189;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0xffe00c00 == 0xd9a00800 {
                                            return 282;
                                        }
                                    } else {
                                        if insn & 0xffe00c00 == 0xd9e00800 {
                                            return 364;
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19a00400 {
                                                    return 216;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19a08400 {
                                                    return 386;
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19a01400 {
                                                    return 220;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19a03400 {
                                                    return 236;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9a00400 {
                                            return 283;
                                        }
                                    }
                                } else {
                                    if insn & 0x80000000 == 0 {
                                        if insn & 0x001000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19e00400 {
                                                    return 217;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19e08400 {
                                                    return 387;
                                                }
                                            }
                                        } else {
                                            if insn & 0x002000 == 0 {
                                                if insn & 0xbfe0fc00 == 0x19e01400 {
                                                    return 221;
                                                }
                                            } else {
                                                if insn & 0xbfe0fc00 == 0x19e03400 {
                                                    return 237;
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0xffe00400 == 0xd9e00400 {
                                            return 365;
                                        }
                                    }
                                }
                            }
                        } else {
                            if insn & 0x40000000 == 0 {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x39800000 {
                                        return 167;
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xb9800000 {
                                        return 174;
                                    }
                                }
                            } else {
                                if insn & 0x80000000 == 0 {
                                    if insn & 0xff800000 == 0x79800000 {
                                        return 170;
                                    }
                                } else {
                                    if insn & 0xffc00000 == 0xf9800000 {
                                        return 280;
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
                            if insn & 0xffc00000 == 0xec000000 {
                                return 347;
                            }
                            if insn & 0x3fc00000 == 0x2c000000 {
                                return 329;
                            }
                        } else {
                            if insn & 0xffc00000 == 0xed000000 {
                                return 350;
                            }
                            if insn & 0x3fc00000 == 0x2d000000 {
                                return 332;
                            }
                        }
                    } else {
                        if insn & 0xfec00000 == 0xec800000 {
                            return 351;
                        }
                        if insn & 0x3ec00000 == 0x2c800000 {
                            return 333;
                        }
                    }
                } else {
                    if insn & 0x800000 == 0 {
                        if insn & 0x1000000 == 0 {
                            if insn & 0xffc00000 == 0xec400000 {
                                return 224;
                            }
                            if insn & 0x3fc00000 == 0x2c400000 {
                                return 142;
                            }
                        } else {
                            if insn & 0xffc00000 == 0xed400000 {
                                return 227;
                            }
                            if insn & 0x3fc00000 == 0x2d400000 {
                                return 145;
                            }
                        }
                    } else {
                        if insn & 0xfec00000 == 0xecc00000 {
                            return 228;
                        }
                        if insn & 0x3ec00000 == 0x2cc00000 {
                            return 146;
                        }
                    }
                }
            } else {
                if insn & 0x1000000 == 0 {
                    if insn & 0x20000000 == 0 {
                        if insn & 0x3f000000 == 0x1c000000 {
                            return 153;
                        }
                    } else {
                        if insn & 0x000400 == 0 {
                            if insn & 0x000800 == 0 {
                                if insn & 0x200000 == 0 {
                                    if insn & 0x400000 == 0 {
                                        if insn & 0x3f600c00 == 0x3c000000 {
                                            return 357;
                                        }
                                    } else {
                                        if insn & 0x3f600c00 == 0x3c400000 {
                                            return 265;
                                        }
                                    }
                                } else {
                                    if insn & 0x001000 == 0 {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x004000 == 0 {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x3c200000 {
                                                                return 68;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3c200000 {
                                                                return 116;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x3ca00000 {
                                                                return 69;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3ca00000 {
                                                                return 117;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x3c600000 {
                                                                return 71;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3c600000 {
                                                                return 119;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x3ce00000 {
                                                                return 70;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3ce00000 {
                                                                return 118;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xffe0fc1f == 0x3c20801f {
                                                            return 287;
                                                        }
                                                        if insn & 0x3fe0fc1f == 0x3c20801f {
                                                            return 297;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc1f == 0x3c60801f {
                                                            return 288;
                                                        }
                                                        if insn & 0x3fe0fc1f == 0x3c60801f {
                                                            return 298;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x008000 == 0 {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x3c204000 {
                                                                return 72;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3c204000 {
                                                                return 120;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x3ca04000 {
                                                                return 73;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3ca04000 {
                                                                return 121;
                                                            }
                                                        }
                                                    } else {
                                                        if insn & 0x800000 == 0 {
                                                            if insn & 0xffe0fc00 == 0x3c604000 {
                                                                return 75;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3c604000 {
                                                                return 123;
                                                            }
                                                        } else {
                                                            if insn & 0xffe0fc00 == 0x3ce04000 {
                                                                return 74;
                                                            }
                                                            if insn & 0x3fe0fc00 == 0x3ce04000 {
                                                                return 122;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x400000 == 0 {
                                                        if insn & 0xffe0fc1f == 0x3c20c01f {
                                                            return 289;
                                                        }
                                                        if insn & 0x3fe0fc1f == 0x3c20c01f {
                                                            return 299;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc1f == 0x3c60c01f {
                                                            return 290;
                                                        }
                                                        if insn & 0x3fe0fc1f == 0x3c60c01f {
                                                            return 300;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c206000 {
                                                            return 76;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c206000 {
                                                            return 124;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ca06000 {
                                                            return 77;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ca06000 {
                                                            return 125;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c606000 {
                                                            return 79;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c606000 {
                                                            return 127;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ce06000 {
                                                            return 78;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ce06000 {
                                                            return 126;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc1f == 0x3c20e01f {
                                                        return 291;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c20e01f {
                                                        return 301;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc1f == 0x3c60e01f {
                                                        return 292;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c60e01f {
                                                        return 302;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if insn & 0x002000 == 0 {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c205000 {
                                                            return 80;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c205000 {
                                                            return 128;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ca05000 {
                                                            return 81;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ca05000 {
                                                            return 129;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c605000 {
                                                            return 83;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c605000 {
                                                            return 131;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ce05000 {
                                                            return 82;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ce05000 {
                                                            return 130;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc1f == 0x3c20d01f {
                                                        return 293;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c20d01f {
                                                        return 303;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc1f == 0x3c60d01f {
                                                        return 294;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c60d01f {
                                                        return 304;
                                                    }
                                                }
                                            }
                                        } else {
                                            if insn & 0x008000 == 0 {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c207000 {
                                                            return 84;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c207000 {
                                                            return 132;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ca07000 {
                                                            return 85;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ca07000 {
                                                            return 133;
                                                        }
                                                    }
                                                } else {
                                                    if insn & 0x800000 == 0 {
                                                        if insn & 0xffe0fc00 == 0x3c607000 {
                                                            return 87;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3c607000 {
                                                            return 135;
                                                        }
                                                    } else {
                                                        if insn & 0xffe0fc00 == 0x3ce07000 {
                                                            return 86;
                                                        }
                                                        if insn & 0x3fe0fc00 == 0x3ce07000 {
                                                            return 134;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if insn & 0x400000 == 0 {
                                                    if insn & 0xffe0fc1f == 0x3c20f01f {
                                                        return 295;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c20f01f {
                                                        return 305;
                                                    }
                                                } else {
                                                    if insn & 0xffe0fc1f == 0x3c60f01f {
                                                        return 296;
                                                    }
                                                    if insn & 0x3fe0fc1f == 0x3c60f01f {
                                                        return 306;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if insn & 0x400000 == 0 {
                                    if insn & 0x3f600c00 == 0x3c200800 {
                                        return 337;
                                    }
                                } else {
                                    if insn & 0x3f600c00 == 0x3c600800 {
                                        return 154;
                                    }
                                }
                            }
                        } else {
                            if insn & 0x400000 == 0 {
                                if insn & 0x3f600400 == 0x3c000400 {
                                    return 338;
                                }
                            } else {
                                if insn & 0x3f600400 == 0x3c400400 {
                                    return 155;
                                }
                            }
                        }
                    }
                } else {
                    if insn & 0x400000 == 0 {
                        if insn & 0x3f400000 == 0x3d000000 {
                            return 339;
                        }
                    } else {
                        if insn & 0x3f400000 == 0x3d400000 {
                            return 156;
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
                        return 0;
                    }
                } else {
                    if insn & 0x7f800000 == 0x52000000 {
                        return 31;
                    }
                }
            } else {
                if insn & 0x40000000 == 0 {
                    if insn & 0x7f800000 == 0x32000000 {
                        return 276;
                    }
                } else {
                    if insn & 0x7f800000 == 0x72000000 {
                        return 2;
                    }
                }
            }
        } else {
            if insn & 0x200000 == 0 {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa000000 {
                            return 1;
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a000000 {
                            return 32;
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a000000 {
                            return 277;
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a000000 {
                            return 3;
                        }
                    }
                }
            } else {
                if insn & 0x20000000 == 0 {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0xa200000 {
                            return 4;
                        }
                    } else {
                        if insn & 0x7f200000 == 0x4a200000 {
                            return 30;
                        }
                    }
                } else {
                    if insn & 0x40000000 == 0 {
                        if insn & 0x7f200000 == 0x2a200000 {
                            return 275;
                        }
                    } else {
                        if insn & 0x7f200000 == 0x6a200000 {
                            return 5;
                        }
                    }
                }
            }
        }
    }
    -1
}
#[doc = r" Decode a 32-bit instruction word."]
pub fn decode(insn: u32) -> Option<Opcode> {
    let index = decode_index(insn);
    if index < 0 {
        return None;
    }
    Some(Opcode {
        bits: insn,
        def: &INSNS[index as usize],
        id: INSN_IDS[index as usize],
    })
}
