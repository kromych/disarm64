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
    bitfield: InsnBitField::CRm,
    lsb: 8,
    width: 4,
}];
const BITFIELDS_1: &[BitfieldSpec] = &[
    BitfieldSpec {
        bitfield: InsnBitField::CRm,
        lsb: 8,
        width: 4,
    },
    BitfieldSpec {
        bitfield: InsnBitField::op2,
        lsb: 5,
        width: 3,
    },
];
const BITFIELDS_2: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rt,
    lsb: 0,
    width: 5,
}];
const BITFIELDS_3: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rn,
    lsb: 5,
    width: 5,
}];
const BITFIELDS_4: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm6_15,
    lsb: 15,
    width: 6,
}];
const BITFIELDS_5: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm4_0,
    lsb: 0,
    width: 4,
}];
const BITFIELDS_6: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::op1,
    lsb: 16,
    width: 3,
}];
const BITFIELDS_7: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::CRn,
    lsb: 12,
    width: 4,
}];
const BITFIELDS_8: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::op2,
    lsb: 5,
    width: 3,
}];
const BITFIELDS_9: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::Rd,
    lsb: 0,
    width: 5,
}];
const OPERANDS_0: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::X16,
    class: InsnOperandClass::INT_REG,
    qualifiers: &[InsnOperandQualifier::X],
    bit_fields: &[],
}];
const OPERANDS_1: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::UIMM4,
    class: InsnOperandClass::IMMEDIATE,
    qualifiers: &[],
    bit_fields: BITFIELDS_0,
}];
const OPERANDS_2: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::BARRIER,
    class: InsnOperandClass::SYSTEM,
    qualifiers: &[],
    bit_fields: &[],
}];
const OPERANDS_3: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::BARRIER_DSB_NXS,
    class: InsnOperandClass::SYSTEM,
    qualifiers: &[],
    bit_fields: &[],
}];
const OPERANDS_4: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::UIMM7,
    class: InsnOperandClass::IMMEDIATE,
    qualifiers: &[],
    bit_fields: BITFIELDS_1,
}];
const OPERANDS_5: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::BARRIER_ISB,
    class: InsnOperandClass::SYSTEM,
    qualifiers: &[],
    bit_fields: &[],
}];
const OPERANDS_6: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_2,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::SYSREG128,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_7: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_2,
    },
    InsnOperand {
        kind: InsnOperandKind::SYSREG,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_8: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::PSTATEFIELD,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::UIMM4,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_0,
    },
];
const OPERANDS_9: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::SYSREG,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
];
const OPERANDS_10: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::SYSREG128,
        class: InsnOperandClass::SYSTEM,
        qualifiers: &[],
        bit_fields: &[],
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_11: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rn,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_3,
    },
    InsnOperand {
        kind: InsnOperandKind::IMM_2,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[InsnOperandQualifier::imm_0_63],
        bit_fields: BITFIELDS_4,
    },
    InsnOperand {
        kind: InsnOperandKind::MASK,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[InsnOperandQualifier::imm_0_15],
        bit_fields: BITFIELDS_5,
    },
];
const OPERANDS_12: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::Rn,
    class: InsnOperandClass::INT_REG,
    qualifiers: &[InsnOperandQualifier::W],
    bit_fields: BITFIELDS_3,
}];
const OPERANDS_13: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP1,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::CRn,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::CRm,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP2,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_8,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
];
const OPERANDS_14: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[InsnOperandQualifier::X],
        bit_fields: BITFIELDS_2,
    },
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP1,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::CRn,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::CRm,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP2,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_8,
    },
];
const OPERANDS_15: &[InsnOperand] = &[
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP1,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_6,
    },
    InsnOperand {
        kind: InsnOperandKind::CRn,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_7,
    },
    InsnOperand {
        kind: InsnOperandKind::CRm,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_0,
    },
    InsnOperand {
        kind: InsnOperandKind::UIMM3_OP2,
        class: InsnOperandClass::IMMEDIATE,
        qualifiers: &[],
        bit_fields: BITFIELDS_8,
    },
    InsnOperand {
        kind: InsnOperandKind::Rt,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: BITFIELDS_2,
    },
    InsnOperand {
        kind: InsnOperandKind::PAIRREG_OR_XZR,
        class: InsnOperandClass::INT_REG,
        qualifiers: &[],
        bit_fields: &[],
    },
];
const OPERANDS_16: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::Rd,
    class: InsnOperandClass::INT_REG,
    qualifiers: &[InsnOperandQualifier::X],
    bit_fields: BITFIELDS_9,
}];
#[doc = r" A matchable identity for each instruction. The discriminant is the"]
#[doc = r" index into INSNS and INSN_IDS."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum InsnId {
    CFINV,
    CHKFEAT_X16,
    CLREX_UIMM4,
    DMB_BARRIER,
    DSB_BARRIER_DSB_NXS,
    DSB_BARRIER,
    HINT_UIMM7,
    ISB_BARRIER_ISB,
    MRRS_Rt_PAIRREG_SYSREG128,
    MRS_Rt_SYSREG,
    MSR_PSTATEFIELD_UIMM4,
    MSR_SYSREG_Rt,
    MSRR_SYSREG128_Rt_PAIRREG,
    RMIF_Rn_IMM_2_MASK,
    SB,
    SETF16_Rn,
    SETF8_Rn,
    SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt,
    SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2,
    SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR,
    WFET_Rd,
    WFIT_Rd,
}
#[doc = r" The identity of each instruction, parallel to INSNS."]
static INSN_IDS: [InsnId; 22] = [
    InsnId::CFINV,
    InsnId::CHKFEAT_X16,
    InsnId::CLREX_UIMM4,
    InsnId::DMB_BARRIER,
    InsnId::DSB_BARRIER_DSB_NXS,
    InsnId::DSB_BARRIER,
    InsnId::HINT_UIMM7,
    InsnId::ISB_BARRIER_ISB,
    InsnId::MRRS_Rt_PAIRREG_SYSREG128,
    InsnId::MRS_Rt_SYSREG,
    InsnId::MSR_PSTATEFIELD_UIMM4,
    InsnId::MSR_SYSREG_Rt,
    InsnId::MSRR_SYSREG128_Rt_PAIRREG,
    InsnId::RMIF_Rn_IMM_2_MASK,
    InsnId::SB,
    InsnId::SETF16_Rn,
    InsnId::SETF8_Rn,
    InsnId::SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt,
    InsnId::SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2,
    InsnId::SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR,
    InsnId::WFET_Rd,
    InsnId::WFIT_Rd,
];
#[doc = r" The decoded instruction definitions, indexed by InsnId."]
static INSNS: [Insn; 22] = [
    Insn {
        mnemonic: "cfinv",
        aliases: &[],
        opcode: 0xd500401f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[],
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "chkfeat",
        aliases: &[],
        opcode: 0xd503251f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::CHK,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "clrex",
        aliases: &[],
        opcode: 0xd503305f,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "dmb",
        aliases: &[],
        opcode: 0xd50330bf,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_2,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "dsb",
        aliases: &[],
        opcode: 0xd503323f,
        mask: 0xfffff3ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::XS,
        operands: OPERANDS_3,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "dsb",
        aliases: &[],
        opcode: 0xd503309f,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_2,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "hint",
        aliases: &[],
        opcode: 0xd503201f,
        mask: 0xfffff01f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_4,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "isb",
        aliases: &[],
        opcode: 0xd50330df,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_5,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "mrrs",
        aliases: &[],
        opcode: 0xd5700000,
        mask: 0xfff00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: OPERANDS_6,
        flags: InsnFlags::const_from_bits(8388608u64),
    },
    Insn {
        mnemonic: "mrs",
        aliases: &[],
        opcode: 0xd5200000,
        mask: 0xffe00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_7,
        flags: InsnFlags::const_from_bits(8388608u64),
    },
    Insn {
        mnemonic: "msr",
        aliases: &[],
        opcode: 0xd500401f,
        mask: 0xfff8f01f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_8,
        flags: InsnFlags::const_from_bits(16777216u64),
    },
    Insn {
        mnemonic: "msr",
        aliases: &[],
        opcode: 0xd5000000,
        mask: 0xffe00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_9,
        flags: InsnFlags::const_from_bits(16777216u64),
    },
    Insn {
        mnemonic: "msrr",
        aliases: &[],
        opcode: 0xd5500000,
        mask: 0xfff00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: OPERANDS_10,
        flags: InsnFlags::const_from_bits(16777216u64),
    },
    Insn {
        mnemonic: "rmif",
        aliases: &[],
        opcode: 0xba000400,
        mask: 0xffe07c10,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: OPERANDS_11,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sb",
        aliases: &[],
        opcode: 0xd50330ff,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::SB,
        operands: &[],
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "setf16",
        aliases: &[],
        opcode: 0x3a00480d,
        mask: 0xfffffc1f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "setf8",
        aliases: &[],
        opcode: 0x3a00080d,
        mask: 0xfffffc1f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: OPERANDS_12,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sys",
        aliases: &[],
        opcode: 0xd5080000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_13,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "sysl",
        aliases: &[],
        opcode: 0xd5280000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_14,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "sysp",
        aliases: &[],
        opcode: 0xd5480000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: OPERANDS_15,
        flags: InsnFlags::const_from_bits(136u64),
    },
    Insn {
        mnemonic: "wfet",
        aliases: &[],
        opcode: 0xd5031000,
        mask: 0xffffffe0,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::WFXT,
        operands: OPERANDS_16,
        flags: InsnFlags::const_from_bits(8u64),
    },
    Insn {
        mnemonic: "wfit",
        aliases: &[],
        opcode: 0xd5031020,
        mask: 0xffffffe0,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::WFXT,
        operands: OPERANDS_16,
        flags: InsnFlags::const_from_bits(8u64),
    },
];
#[doc = r" Return the index of the matching instruction in INSNS, or -1."]
fn decode_index(insn: u32) -> i32 {
    if insn & 0x200000 == 0 {
        if insn & 0x400000 == 0 {
            if insn & 0x1000000 == 0 {
                if insn & 0x000400 == 0 {
                    if insn & 0x004000 == 0 {
                        if insn & 0xfffffc1f == 0x3a00080d {
                            return 16;
                        }
                    } else {
                        if insn & 0xfffffc1f == 0x3a00480d {
                            return 15;
                        }
                    }
                } else {
                    if insn & 0xffe07c10 == 0xba000400 {
                        return 13;
                    }
                }
            } else {
                if insn == 0xd503251f {
                    return 1;
                }
                if insn == 0xd50330ff {
                    return 14;
                }
                if insn == 0xd500401f {
                    return 0;
                }
                if insn & 0xfffff3ff == 0xd503323f {
                    return 4;
                }
                if insn & 0xfffff0ff == 0xd503305f {
                    return 2;
                }
                if insn & 0xfffff0ff == 0xd503309f {
                    return 5;
                }
                if insn & 0xfffff0ff == 0xd50330bf {
                    return 3;
                }
                if insn & 0xfffff0ff == 0xd50330df {
                    return 7;
                }
                if insn & 0xffffffe0 == 0xd5031000 {
                    return 20;
                }
                if insn & 0xffffffe0 == 0xd5031020 {
                    return 21;
                }
                if insn & 0xfffff01f == 0xd503201f {
                    return 6;
                }
                if insn & 0xfff8f01f == 0xd500401f {
                    return 10;
                }
                if insn & 0xfff80000 == 0xd5080000 {
                    return 17;
                }
                if insn & 0xffe00000 == 0xd5000000 {
                    return 11;
                }
            }
        } else {
            if insn & 0x100000 == 0 {
                if insn & 0xfff80000 == 0xd5480000 {
                    return 19;
                }
            } else {
                if insn & 0xfff00000 == 0xd5500000 {
                    return 12;
                }
            }
        }
    } else {
        if insn & 0x400000 == 0 {
            if insn & 0xfff80000 == 0xd5280000 {
                return 18;
            }
            if insn & 0xffe00000 == 0xd5200000 {
                return 9;
            }
        } else {
            if insn & 0xfff00000 == 0xd5700000 {
                return 8;
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
