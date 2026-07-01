// Auto-generated.
// The changes will be LOST.

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macro_rules)]
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
#[doc = r" Define instruction newtype structs with Debug impl."]
macro_rules ! define_insn_types { ($ ($ name : ident) , * $ (,) ?) => { $ (# [derive (Copy , Clone , PartialEq , Eq)] pub struct $ name (pub u32) ; impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{}({:#010x})" , stringify ! ($ name) , self . 0) } }) * } ; }
#[doc = r" Define DEFINITION, make_opcode, and InsnOpcode for each instruction struct."]
macro_rules ! define_insn_impls { ($ ($ name : ident ($ mnemonic_str : expr , $ mnemonic_ident : ident , $ opcode : expr , $ mask : expr , $ class : ident , $ feature_set : ident , $ flags : expr , [$ ($ operand : expr) , * $ (,) ?])) , * $ (,) ?) => { $ (impl $ name { pub const DEFINITION : Insn = Insn { mnemonic : $ mnemonic_str , aliases : & [] , opcode : $ opcode , mask : $ mask , class : InsnClass :: $ class , feature_set : InsnFeatureSet :: $ feature_set , operands : & [$ ($ operand) , *] , flags : $ flags , } ; fn make_opcode (bits : u32) -> Opcode { Opcode { mnemonic : Mnemonic :: $ mnemonic_ident , operation : Operation :: $ class ($ class :: $ name ($ name (bits))) } } } impl InsnOpcode for $ name { fn definition (& self) -> & 'static Insn { & Self :: DEFINITION } fn bits (& self) -> u32 { self . 0 } }) * } ; }
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
define_insn_types!(
    CFINV,
    CHKFEAT_X16,
    CLREX_UIMM4,
    DGH,
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
    WFIT_Rd
);
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
define_insn_impls!(
    CFINV(
        "cfinv",
        r#cfinv,
        0xd500401f,
        0xffffffff,
        IC_SYSTEM,
        FLAGM,
        InsnFlags::empty(),
        []
    ),
    CHKFEAT_X16(
        "chkfeat",
        r#chkfeat,
        0xd503251f,
        0xffffffff,
        IC_SYSTEM,
        CHK,
        InsnFlags::empty(),
        [InsnOperand {
            kind: InsnOperandKind::X16,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X,],
            bit_fields: &[],
        }]
    ),
    CLREX_UIMM4(
        "clrex",
        r#clrex,
        0xd503305f,
        0xfffff0ff,
        IC_SYSTEM,
        V8,
        InsnFlags::empty(),
        [InsnOperand {
            kind: InsnOperandKind::UIMM4,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::CRm,
                lsb: 8,
                width: 4,
            }],
        }]
    ),
    DGH(
        "dgh",
        r#dgh,
        0xd50320df,
        0xffffffff,
        IC_SYSTEM,
        V8,
        InsnFlags::empty(),
        []
    ),
    DMB_BARRIER(
        "dmb",
        r#dmb,
        0xd50330bf,
        0xfffff0ff,
        IC_SYSTEM,
        V8,
        InsnFlags::empty(),
        [InsnOperand {
            kind: InsnOperandKind::BARRIER,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }]
    ),
    DSB_BARRIER_DSB_NXS(
        "dsb",
        r#dsb,
        0xd503323f,
        0xfffff3ff,
        IC_SYSTEM,
        XS,
        InsnFlags::const_from_bits(8u64),
        [InsnOperand {
            kind: InsnOperandKind::BARRIER_DSB_NXS,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }]
    ),
    DSB_BARRIER(
        "dsb",
        r#dsb,
        0xd503309f,
        0xfffff0ff,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(8u64),
        [InsnOperand {
            kind: InsnOperandKind::BARRIER,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }]
    ),
    HINT_UIMM7(
        "hint",
        r#hint,
        0xd503201f,
        0xfffff01f,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(8u64),
        [InsnOperand {
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
                }
            ],
        }]
    ),
    ISB_BARRIER_ISB(
        "isb",
        r#isb,
        0xd50330df,
        0xfffff0ff,
        IC_SYSTEM,
        V8,
        InsnFlags::empty(),
        [InsnOperand {
            kind: InsnOperandKind::BARRIER_ISB,
            class: InsnOperandClass::SYSTEM,
            qualifiers: &[],
            bit_fields: &[],
        }]
    ),
    MRRS_Rt_PAIRREG_SYSREG128(
        "mrrs",
        r#mrrs,
        0xd5700000,
        0xfff00000,
        IC_SYSTEM,
        D128,
        InsnFlags::const_from_bits(8388608u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rt,
                    lsb: 0,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::PAIRREG,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[],
            },
            InsnOperand {
                kind: InsnOperandKind::SYSREG128,
                class: InsnOperandClass::SYSTEM,
                qualifiers: &[],
                bit_fields: &[],
            }
        ]
    ),
    MRS_Rt_SYSREG(
        "mrs",
        r#mrs,
        0xd5200000,
        0xffe00000,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(8388608u64),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    MSR_PSTATEFIELD_UIMM4(
        "msr",
        r#msr,
        0xd500401f,
        0xfff8f01f,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(16777216u64),
        [
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
            }
        ]
    ),
    MSR_SYSREG_Rt(
        "msr",
        r#msr,
        0xd5000000,
        0xffe00000,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(16777216u64),
        [
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
            }
        ]
    ),
    MSRR_SYSREG128_Rt_PAIRREG(
        "msrr",
        r#msrr,
        0xd5500000,
        0xfff00000,
        IC_SYSTEM,
        D128,
        InsnFlags::const_from_bits(16777216u64),
        [
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
            }
        ]
    ),
    RMIF_Rn_IMM_2_MASK(
        "rmif",
        r#rmif,
        0xba000400,
        0xffe07c10,
        IC_SYSTEM,
        FLAGM,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rn,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::Rn,
                    lsb: 5,
                    width: 5,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::IMM_2,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::imm_0_63,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm6_15,
                    lsb: 15,
                    width: 6,
                }],
            },
            InsnOperand {
                kind: InsnOperandKind::MASK,
                class: InsnOperandClass::IMMEDIATE,
                qualifiers: &[InsnOperandQualifier::imm_0_15,],
                bit_fields: &[BitfieldSpec {
                    bitfield: InsnBitField::imm4_0,
                    lsb: 0,
                    width: 4,
                }],
            }
        ]
    ),
    SB(
        "sb",
        r#sb,
        0xd50330ff,
        0xffffffff,
        IC_SYSTEM,
        SB,
        InsnFlags::empty(),
        []
    ),
    SETF16_Rn(
        "setf16",
        r#setf16,
        0x3a00480d,
        0xfffffc1f,
        IC_SYSTEM,
        FLAGM,
        InsnFlags::const_from_bits(131072u64),
        [InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::W,],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }]
    ),
    SETF8_Rn(
        "setf8",
        r#setf8,
        0x3a00080d,
        0xfffffc1f,
        IC_SYSTEM,
        FLAGM,
        InsnFlags::const_from_bits(131072u64),
        [InsnOperand {
            kind: InsnOperandKind::Rn,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::W,],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rn,
                lsb: 5,
                width: 5,
            }],
        }]
    ),
    SYS_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt(
        "sys",
        r#sys,
        0xd5080000,
        0xfff80000,
        IC_SYSTEM,
        V8,
        InsnFlags::const_from_bits(8u64),
        [
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
            }
        ]
    ),
    SYSL_Rt_UIMM3_OP1_CRn_CRm_UIMM3_OP2(
        "sysl",
        r#sysl,
        0xd5280000,
        0xfff80000,
        IC_SYSTEM,
        V8,
        InsnFlags::empty(),
        [
            InsnOperand {
                kind: InsnOperandKind::Rt,
                class: InsnOperandClass::INT_REG,
                qualifiers: &[InsnOperandQualifier::X,],
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
            }
        ]
    ),
    SYSP_UIMM3_OP1_CRn_CRm_UIMM3_OP2_Rt_PAIRREG_OR_XZR(
        "sysp",
        r#sysp,
        0xd5480000,
        0xfff80000,
        IC_SYSTEM,
        D128,
        InsnFlags::const_from_bits(136u64),
        [
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
            }
        ]
    ),
    WFET_Rd(
        "wfet",
        r#wfet,
        0xd5031000,
        0xffffffe0,
        IC_SYSTEM,
        WFXT,
        InsnFlags::const_from_bits(8u64),
        [InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X,],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }]
    ),
    WFIT_Rd(
        "wfit",
        r#wfit,
        0xd5031020,
        0xffffffe0,
        IC_SYSTEM,
        WFXT,
        InsnFlags::const_from_bits(8u64),
        [InsnOperand {
            kind: InsnOperandKind::Rd,
            class: InsnOperandClass::INT_REG,
            qualifiers: &[InsnOperandQualifier::X,],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::Rd,
                lsb: 0,
                width: 5,
            }],
        }]
    )
);
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
