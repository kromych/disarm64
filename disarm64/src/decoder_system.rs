// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
use bitfield_struct::bitfield;
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
#[doc = r" Leaf nodes in the decision tree"]
struct Leaf {
    insn: &'static Insn,
    factory: fn(u32) -> Opcode,
}
#[doc = r" The decision tree node"]
enum Decode {
    #[doc = r" Branch in the decision tree"]
    Branch {
        mask: u32,
        next: [Option<u16>; 2],
    },
    Leaf(&'static [Leaf]),
}
#[doc = r" The decode table"]
type DecodeTable = &'static [Decode];
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mnemonic {
    r#cfinv,
    r#chkfeat,
    r#clrex,
    r#dgh,
    r#dmb,
    r#dsb,
    r#hint,
    r#isb,
    r#mrrs,
    r#mrs,
    r#msr,
    r#msrr,
    r#rmif,
    r#sb,
    r#setf16,
    r#setf8,
    r#sys,
    r#sysl,
    r#sysp,
    r#wfet,
    r#wfit,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CFINV {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CHKFEAT_X16 {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CLREX_UIMM4 {
    #[bits(8)]
    pub _op_0: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(20)]
    pub _op_12: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DGH {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DMB_BARRIER {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DSB_BARRIER_DSB_NXS {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DSB_BARRIER {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HINT_UIMM7 {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(3)]
    pub op2: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(20)]
    pub _op_12: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ISB_BARRIER_ISB {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MRRS_Rt_PAIRREG_SYSREG128 {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MRS_Rt_SYSREG {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MSR_PSTATEFIELD_UIMM4 {
    #[bits(8)]
    pub _op_0: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(20)]
    pub _op_12: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MSR_SYSREG_Rt {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MSRR_SYSREG128_Rt_PAIRREG {
    #[bits(5)]
    pub rt: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RMIF_Rn_IMM_2_MASK {
    #[bits(4)]
    pub imm4_0: u32,
    #[bits(1)]
    pub _op_4: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(5)]
    pub _op_10: u32,
    #[bits(6)]
    pub imm6_15: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SB {
    #[bits(32)]
    pub _op_0: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SETF16_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SETF8_Rn {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(5)]
    pub rn: u32,
    #[bits(22)]
    pub _op_10: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt {
    #[bits(5)]
    pub rt: u32,
    #[bits(3)]
    pub op2: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(4)]
    pub crn: u32,
    #[bits(3)]
    pub op1: u32,
    #[bits(13)]
    pub _op_19: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2 {
    #[bits(5)]
    pub rt: u32,
    #[bits(3)]
    pub op2: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(4)]
    pub crn: u32,
    #[bits(3)]
    pub op1: u32,
    #[bits(13)]
    pub _op_19: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR {
    #[bits(5)]
    pub rt: u32,
    #[bits(3)]
    pub op2: u32,
    #[bits(4)]
    pub crm: u32,
    #[bits(4)]
    pub crn: u32,
    #[bits(3)]
    pub op1: u32,
    #[bits(13)]
    pub _op_19: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct WFET_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct WFIT_Rd {
    #[bits(5)]
    pub rd: u32,
    #[bits(27)]
    pub _op_5: u32,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum IC_SYSTEM {
    CFINV(CFINV),
    CHKFEAT_X16(CHKFEAT_X16),
    CLREX_UIMM4(CLREX_UIMM4),
    DGH(DGH),
    DMB_BARRIER(DMB_BARRIER),
    DSB_BARRIER(DSB_BARRIER),
    DSB_BARRIER_DSB_NXS(DSB_BARRIER_DSB_NXS),
    HINT_UIMM7(HINT_UIMM7),
    ISB_BARRIER_ISB(ISB_BARRIER_ISB),
    MRRS_Rt_PAIRREG_SYSREG128(MRRS_Rt_PAIRREG_SYSREG128),
    MRS_Rt_SYSREG(MRS_Rt_SYSREG),
    MSRR_SYSREG128_Rt_PAIRREG(MSRR_SYSREG128_Rt_PAIRREG),
    MSR_PSTATEFIELD_UIMM4(MSR_PSTATEFIELD_UIMM4),
    MSR_SYSREG_Rt(MSR_SYSREG_Rt),
    RMIF_Rn_IMM_2_MASK(RMIF_Rn_IMM_2_MASK),
    SB(SB),
    SETF16_Rn(SETF16_Rn),
    SETF8_Rn(SETF8_Rn),
    SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2(SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2),
    SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR(
        SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR,
    ),
    SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt(SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt),
    WFET_Rd(WFET_Rd),
    WFIT_Rd(WFIT_Rd),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    IC_SYSTEM(IC_SYSTEM),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub operation: Operation,
}
impl CFINV {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "cfinv",
        aliases: &[],
        opcode: 0xd500401f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#cfinv,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::CFINV(CFINV::from(bits))),
        }
    }
}
impl InsnOpcode for CFINV {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CHKFEAT_X16 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "chkfeat",
        aliases: &[],
        opcode: 0xd503251f,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::CHK,
        operands: &[InsnOperand {
            kind: InsnOperandKind::X16,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#chkfeat,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::CHKFEAT_X16(CHKFEAT_X16::from(bits))),
        }
    }
}
impl InsnOpcode for CHKFEAT_X16 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl CLREX_UIMM4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "clrex",
        aliases: &[],
        opcode: 0xd503305f,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::UIMM4,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::CRm,
                lsb: 8,
                width: 4,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#clrex,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::CLREX_UIMM4(CLREX_UIMM4::from(bits))),
        }
    }
}
impl InsnOpcode for CLREX_UIMM4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DGH {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dgh",
        aliases: &[],
        opcode: 0xd50320df,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dgh,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::DGH(DGH::from(bits))),
        }
    }
}
impl InsnOpcode for DGH {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DMB_BARRIER {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dmb",
        aliases: &[],
        opcode: 0xd50330bf,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::BARRIER,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dmb,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::DMB_BARRIER(DMB_BARRIER::from(bits))),
        }
    }
}
impl InsnOpcode for DMB_BARRIER {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DSB_BARRIER_DSB_NXS {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dsb",
        aliases: &[],
        opcode: 0xd503323f,
        mask: 0xfffff3ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::XS,
        operands: &[InsnOperand {
            kind: InsnOperandKind::BARRIER_DSB_NXS,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dsb,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::DSB_BARRIER_DSB_NXS(
                DSB_BARRIER_DSB_NXS::from(bits),
            )),
        }
    }
}
impl InsnOpcode for DSB_BARRIER_DSB_NXS {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DSB_BARRIER {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dsb",
        aliases: &[],
        opcode: 0xd503309f,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::BARRIER,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dsb,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::DSB_BARRIER(DSB_BARRIER::from(bits))),
        }
    }
}
impl InsnOpcode for DSB_BARRIER {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl HINT_UIMM7 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "hint",
        aliases: &[],
        opcode: 0xd503201f,
        mask: 0xfffff01f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::UIMM7,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[
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
            ],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#hint,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::HINT_UIMM7(HINT_UIMM7::from(bits))),
        }
    }
}
impl InsnOpcode for HINT_UIMM7 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl ISB_BARRIER_ISB {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "isb",
        aliases: &[],
        opcode: 0xd50330df,
        mask: 0xfffff0ff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::BARRIER_ISB,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#isb,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::ISB_BARRIER_ISB(ISB_BARRIER_ISB::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for ISB_BARRIER_ISB {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MRRS_Rt_PAIRREG_SYSREG128 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mrrs",
        aliases: &[],
        opcode: 0xd5700000,
        mask: 0xfff00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
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
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::IS_SYS_READ.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mrrs,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::MRRS_Rt_PAIRREG_SYSREG128(
                MRRS_Rt_PAIRREG_SYSREG128::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MRRS_Rt_PAIRREG_SYSREG128 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MRS_Rt_SYSREG {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "mrs",
        aliases: &[],
        opcode: 0xd5200000,
        mask: 0xffe00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::SYSREG,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::IS_SYS_READ.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#mrs,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::MRS_Rt_SYSREG(MRS_Rt_SYSREG::from(bits))),
        }
    }
}
impl InsnOpcode for MRS_Rt_SYSREG {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MSR_PSTATEFIELD_UIMM4 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "msr",
        aliases: &[],
        opcode: 0xd500401f,
        mask: 0xfff8f01f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRm,
                    lsb: 8,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::IS_SYS_WRITE.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#msr,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::MSR_PSTATEFIELD_UIMM4(
                MSR_PSTATEFIELD_UIMM4::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MSR_PSTATEFIELD_UIMM4 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MSR_SYSREG_Rt {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "msr",
        aliases: &[],
        opcode: 0xd5000000,
        mask: 0xffe00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
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
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::IS_SYS_WRITE.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#msr,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::MSR_SYSREG_Rt(MSR_SYSREG_Rt::from(bits))),
        }
    }
}
impl InsnOpcode for MSR_SYSREG_Rt {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl MSRR_SYSREG128_Rt_PAIRREG {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "msrr",
        aliases: &[],
        opcode: 0xd5500000,
        mask: 0xfff00000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: &[
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
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::IS_SYS_WRITE.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#msrr,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::MSRR_SYSREG128_Rt_PAIRREG(
                MSRR_SYSREG128_Rt_PAIRREG::from(bits),
            )),
        }
    }
}
impl InsnOpcode for MSRR_SYSREG128_Rt_PAIRREG {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl RMIF_Rn_IMM_2_MASK {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "rmif",
        aliases: &[],
        opcode: 0xba000400,
        mask: 0xffe07c10,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::imm_0_63],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm6_15,
                    lsb: 15,
                    width: 6,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::MASK,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::imm_0_15],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm4_0,
                    lsb: 0,
                    width: 4,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#rmif,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::RMIF_Rn_IMM_2_MASK(
                RMIF_Rn_IMM_2_MASK::from(bits),
            )),
        }
    }
}
impl InsnOpcode for RMIF_Rn_IMM_2_MASK {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SB {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sb",
        aliases: &[],
        opcode: 0xd50330ff,
        mask: 0xffffffff,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::SB,
        operands: &[],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#sb,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::SB(SB::from(bits))),
        }
    }
}
impl InsnOpcode for SB {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SETF16_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "setf16",
        aliases: &[],
        opcode: 0x3a00480d,
        mask: 0xfffffc1f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::W],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#setf16,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::SETF16_Rn(SETF16_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for SETF16_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SETF8_Rn {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "setf8",
        aliases: &[],
        opcode: 0x3a00080d,
        mask: 0xfffffc1f,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::FLAGM,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::W],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_SF_FIELD.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#setf8,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::SETF8_Rn(SETF8_Rn::from(bits))),
        }
    }
}
impl InsnOpcode for SETF8_Rn {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sys",
        aliases: &[],
        opcode: 0xd5080000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP1,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op1,
                    lsb: 16,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRn,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRn,
                    lsb: 12,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRm,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRm,
                    lsb: 8,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op2,
                    lsb: 5,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
        ],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#sys,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt(
                SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2 {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sysl",
        aliases: &[],
        opcode: 0xd5280000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::V8,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP1,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op1,
                    lsb: 16,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRn,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRn,
                    lsb: 12,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRm,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRm,
                    lsb: 8,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op2,
                    lsb: 5,
                    width: 3,
                }],
            },
        ],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#sysl,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2(
                SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2::from(bits),
            )),
        }
    }
}
impl InsnOpcode for SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2 {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "sysp",
        aliases: &[],
        opcode: 0xd5480000,
        mask: 0xfff80000,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::D128,
        operands: &[
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP1,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op1,
                    lsb: 16,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRn,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRn,
                    lsb: 12,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::CRm,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::CRm,
                    lsb: 8,
                    width: 4,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::UIMM3_OP2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::op2,
                    lsb: 5,
                    width: 3,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG_OR_XZR,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[],
                bit_fields: &[],
            },
        ],
        flags: InsnFlags::const_from_bits(
            InsnFlags::HAS_ALIAS.bits() | InsnFlags::HAS_NARROW.bits(),
        ),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#sysp,
            operation: Operation::IC_SYSTEM(
                IC_SYSTEM::SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR(
                    SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR::from(bits),
                ),
            ),
        }
    }
}
impl InsnOpcode for SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl WFET_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "wfet",
        aliases: &[],
        opcode: 0xd5031000,
        mask: 0xffffffe0,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::WFXT,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#wfet,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::WFET_Rd(WFET_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for WFET_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl WFIT_Rd {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "wfit",
        aliases: &[],
        opcode: 0xd5031020,
        mask: 0xffffffe0,
        class: InsnClass::IC_SYSTEM,
        feature_set: InsnFeatureSet::WFXT,
        operands: &[InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }],
        flags: InsnFlags::const_from_bits(InsnFlags::HAS_ALIAS.bits()),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#wfit,
            operation: Operation::IC_SYSTEM(IC_SYSTEM::WFIT_Rd(WFIT_Rd::from(bits))),
        }
    }
}
impl InsnOpcode for WFIT_Rd {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl InsnOpcode for IC_SYSTEM {
    fn definition(&self) -> &'static Insn {
        match self {
            IC_SYSTEM::CFINV(opcode) => opcode.definition(),
            IC_SYSTEM::CHKFEAT_X16(opcode) => opcode.definition(),
            IC_SYSTEM::CLREX_UIMM4(opcode) => opcode.definition(),
            IC_SYSTEM::DGH(opcode) => opcode.definition(),
            IC_SYSTEM::DMB_BARRIER(opcode) => opcode.definition(),
            IC_SYSTEM::DSB_BARRIER(opcode) => opcode.definition(),
            IC_SYSTEM::DSB_BARRIER_DSB_NXS(opcode) => opcode.definition(),
            IC_SYSTEM::HINT_UIMM7(opcode) => opcode.definition(),
            IC_SYSTEM::ISB_BARRIER_ISB(opcode) => opcode.definition(),
            IC_SYSTEM::MRRS_Rt_PAIRREG_SYSREG128(opcode) => opcode.definition(),
            IC_SYSTEM::MRS_Rt_SYSREG(opcode) => opcode.definition(),
            IC_SYSTEM::MSRR_SYSREG128_Rt_PAIRREG(opcode) => opcode.definition(),
            IC_SYSTEM::MSR_PSTATEFIELD_UIMM4(opcode) => opcode.definition(),
            IC_SYSTEM::MSR_SYSREG_Rt(opcode) => opcode.definition(),
            IC_SYSTEM::RMIF_Rn_IMM_2_MASK(opcode) => opcode.definition(),
            IC_SYSTEM::SB(opcode) => opcode.definition(),
            IC_SYSTEM::SETF16_Rn(opcode) => opcode.definition(),
            IC_SYSTEM::SETF8_Rn(opcode) => opcode.definition(),
            IC_SYSTEM::SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2(opcode) => opcode.definition(),
            IC_SYSTEM::SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR(opcode) => {
                opcode.definition()
            }
            IC_SYSTEM::SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt(opcode) => opcode.definition(),
            IC_SYSTEM::WFET_Rd(opcode) => opcode.definition(),
            IC_SYSTEM::WFIT_Rd(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            IC_SYSTEM::CFINV(opcode) => opcode.bits(),
            IC_SYSTEM::CHKFEAT_X16(opcode) => opcode.bits(),
            IC_SYSTEM::CLREX_UIMM4(opcode) => opcode.bits(),
            IC_SYSTEM::DGH(opcode) => opcode.bits(),
            IC_SYSTEM::DMB_BARRIER(opcode) => opcode.bits(),
            IC_SYSTEM::DSB_BARRIER(opcode) => opcode.bits(),
            IC_SYSTEM::DSB_BARRIER_DSB_NXS(opcode) => opcode.bits(),
            IC_SYSTEM::HINT_UIMM7(opcode) => opcode.bits(),
            IC_SYSTEM::ISB_BARRIER_ISB(opcode) => opcode.bits(),
            IC_SYSTEM::MRRS_Rt_PAIRREG_SYSREG128(opcode) => opcode.bits(),
            IC_SYSTEM::MRS_Rt_SYSREG(opcode) => opcode.bits(),
            IC_SYSTEM::MSRR_SYSREG128_Rt_PAIRREG(opcode) => opcode.bits(),
            IC_SYSTEM::MSR_PSTATEFIELD_UIMM4(opcode) => opcode.bits(),
            IC_SYSTEM::MSR_SYSREG_Rt(opcode) => opcode.bits(),
            IC_SYSTEM::RMIF_Rn_IMM_2_MASK(opcode) => opcode.bits(),
            IC_SYSTEM::SB(opcode) => opcode.bits(),
            IC_SYSTEM::SETF16_Rn(opcode) => opcode.bits(),
            IC_SYSTEM::SETF8_Rn(opcode) => opcode.bits(),
            IC_SYSTEM::SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2(opcode) => opcode.bits(),
            IC_SYSTEM::SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR(opcode) => opcode.bits(),
            IC_SYSTEM::SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt(opcode) => opcode.bits(),
            IC_SYSTEM::WFET_Rd(opcode) => opcode.bits(),
            IC_SYSTEM::WFIT_Rd(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for Operation {
    fn definition(&self) -> &'static Insn {
        match self {
            Operation::IC_SYSTEM(class) => class.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            Operation::IC_SYSTEM(class) => class.bits(),
        }
    }
}
impl InsnOpcode for Opcode {
    fn definition(&self) -> &'static Insn {
        self.operation.definition()
    }
    fn bits(&self) -> u32 {
        self.operation.bits()
    }
}
pub fn decode(insn: u32) -> Option<Opcode> {
    if insn & 0x200000 == 0 {
        if insn & 0x400000 == 0 {
            if insn & 0x1000000 == 0 {
                if insn & 0x000400 == 0 {
                    if insn & 0x004000 == 0 {
                        if insn & 0xfffffc1f == 0x3a00080d {
                            return Some(SETF8_Rn::make_opcode(insn));
                        }
                    } else {
                        if insn & 0xfffffc1f == 0x3a00480d {
                            return Some(SETF16_Rn::make_opcode(insn));
                        }
                    }
                } else {
                    if insn & 0xffe07c10 == 0xba000400 {
                        return Some(RMIF_Rn_IMM_2_MASK::make_opcode(insn));
                    }
                }
            } else {
                if insn == 0xd503251f {
                    return Some(CHKFEAT_X16::make_opcode(insn));
                }
                if insn == 0xd50320df {
                    return Some(DGH::make_opcode(insn));
                }
                if insn == 0xd50330ff {
                    return Some(SB::make_opcode(insn));
                }
                if insn == 0xd500401f {
                    return Some(CFINV::make_opcode(insn));
                }
                if insn & 0xfffff3ff == 0xd503323f {
                    return Some(DSB_BARRIER_DSB_NXS::make_opcode(insn));
                }
                if insn & 0xfffff0ff == 0xd503305f {
                    return Some(CLREX_UIMM4::make_opcode(insn));
                }
                if insn & 0xfffff0ff == 0xd503309f {
                    return Some(DSB_BARRIER::make_opcode(insn));
                }
                if insn & 0xfffff0ff == 0xd50330bf {
                    return Some(DMB_BARRIER::make_opcode(insn));
                }
                if insn & 0xfffff0ff == 0xd50330df {
                    return Some(ISB_BARRIER_ISB::make_opcode(insn));
                }
                if insn & 0xffffffe0 == 0xd5031000 {
                    return Some(WFET_Rd::make_opcode(insn));
                }
                if insn & 0xffffffe0 == 0xd5031020 {
                    return Some(WFIT_Rd::make_opcode(insn));
                }
                if insn & 0xfffff01f == 0xd503201f {
                    return Some(HINT_UIMM7::make_opcode(insn));
                }
                if insn & 0xfff8f01f == 0xd500401f {
                    return Some(MSR_PSTATEFIELD_UIMM4::make_opcode(insn));
                }
                if insn & 0xfff80000 == 0xd5080000 {
                    return Some(SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt::make_opcode(insn));
                }
                if insn & 0xffe00000 == 0xd5000000 {
                    return Some(MSR_SYSREG_Rt::make_opcode(insn));
                }
            }
        } else {
            if insn & 0x100000 == 0 {
                if insn & 0xfff80000 == 0xd5480000 {
                    return Some(
                        SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR::make_opcode(insn),
                    );
                }
            } else {
                if insn & 0xfff00000 == 0xd5500000 {
                    return Some(MSRR_SYSREG128_Rt_PAIRREG::make_opcode(insn));
                }
            }
        }
    } else {
        if insn & 0x400000 == 0 {
            if insn & 0xfff80000 == 0xd5280000 {
                return Some(SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2::make_opcode(insn));
            }
            if insn & 0xffe00000 == 0xd5200000 {
                return Some(MRS_Rt_SYSREG::make_opcode(insn));
            }
        } else {
            if insn & 0xfff00000 == 0xd5700000 {
                return Some(MRRS_Rt_PAIRREG_SYSREG128::make_opcode(insn));
            }
        }
    }
    None
}
