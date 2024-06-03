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
    r#brk,
    r#dcps1,
    r#dcps2,
    r#dcps3,
    r#hlt,
    r#hvc,
    r#smc,
    r#svc,
    r#udf,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BRK_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DCPS1_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DCPS2_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DCPS3_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HLT_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HVC_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SMC_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SVC_EXCEPTION {
    #[bits(5)]
    pub _op_0: u32,
    #[bits(16)]
    pub imm16_5: u32,
    #[bits(11)]
    pub _op_21: u32,
}
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct UDF_UNDEFINED {
    #[bits(16)]
    pub imm16_0: u32,
    #[bits(16)]
    pub _op_16: u32,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum EXCEPTION {
    BRK_EXCEPTION(BRK_EXCEPTION),
    DCPS1_EXCEPTION(DCPS1_EXCEPTION),
    DCPS2_EXCEPTION(DCPS2_EXCEPTION),
    DCPS3_EXCEPTION(DCPS3_EXCEPTION),
    HLT_EXCEPTION(HLT_EXCEPTION),
    HVC_EXCEPTION(HVC_EXCEPTION),
    SMC_EXCEPTION(SMC_EXCEPTION),
    SVC_EXCEPTION(SVC_EXCEPTION),
    UDF_UNDEFINED(UDF_UNDEFINED),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    EXCEPTION(EXCEPTION),
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Opcode {
    pub mnemonic: Mnemonic,
    pub operation: Operation,
}
impl BRK_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "brk",
        aliases: &[],
        opcode: 0xd4200000,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#brk,
            operation: Operation::EXCEPTION(EXCEPTION::BRK_EXCEPTION(BRK_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for BRK_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DCPS1_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dcps1",
        aliases: &[],
        opcode: 0xd4a00001,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dcps1,
            operation: Operation::EXCEPTION(EXCEPTION::DCPS1_EXCEPTION(DCPS1_EXCEPTION::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for DCPS1_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DCPS2_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dcps2",
        aliases: &[],
        opcode: 0xd4a00002,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dcps2,
            operation: Operation::EXCEPTION(EXCEPTION::DCPS2_EXCEPTION(DCPS2_EXCEPTION::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for DCPS2_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl DCPS3_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "dcps3",
        aliases: &[],
        opcode: 0xd4a00003,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#dcps3,
            operation: Operation::EXCEPTION(EXCEPTION::DCPS3_EXCEPTION(DCPS3_EXCEPTION::from(
                bits,
            ))),
        }
    }
}
impl InsnOpcode for DCPS3_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl HLT_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "hlt",
        aliases: &[],
        opcode: 0xd4400000,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#hlt,
            operation: Operation::EXCEPTION(EXCEPTION::HLT_EXCEPTION(HLT_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for HLT_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl HVC_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "hvc",
        aliases: &[],
        opcode: 0xd4000002,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#hvc,
            operation: Operation::EXCEPTION(EXCEPTION::HVC_EXCEPTION(HVC_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for HVC_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SMC_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "smc",
        aliases: &[],
        opcode: 0xd4000003,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#smc,
            operation: Operation::EXCEPTION(EXCEPTION::SMC_EXCEPTION(SMC_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for SMC_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl SVC_EXCEPTION {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "svc",
        aliases: &[],
        opcode: 0xd4000001,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::EXCEPTION,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_5,
                lsb: 5,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#svc,
            operation: Operation::EXCEPTION(EXCEPTION::SVC_EXCEPTION(SVC_EXCEPTION::from(bits))),
        }
    }
}
impl InsnOpcode for SVC_EXCEPTION {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl UDF_UNDEFINED {
    pub const DEFINITION: Insn = Insn {
        mnemonic: "udf",
        aliases: &[],
        opcode: 0x000000,
        mask: 0xffff0000,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: &[InsnOperand {
            kind: InsnOperandKind::UNDEFINED,
            class: InsnOperandClass::IMMEDIATE,
            qualifiers: &[],
            bit_fields: &[BitfieldSpec {
                bitfield: InsnBitField::imm16_0,
                lsb: 0,
                width: 16,
            }],
        }],
        flags: InsnFlags::empty(),
    };
    fn make_opcode(bits: u32) -> Opcode {
        Opcode {
            mnemonic: Mnemonic::r#udf,
            operation: Operation::EXCEPTION(EXCEPTION::UDF_UNDEFINED(UDF_UNDEFINED::from(bits))),
        }
    }
}
impl InsnOpcode for UDF_UNDEFINED {
    fn definition(&self) -> &'static Insn {
        &Self::DEFINITION
    }
    fn bits(&self) -> u32 {
        (*self).into()
    }
}
impl InsnOpcode for EXCEPTION {
    fn definition(&self) -> &'static Insn {
        match self {
            EXCEPTION::BRK_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::DCPS1_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::DCPS2_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::DCPS3_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::HLT_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::HVC_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::SMC_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::SVC_EXCEPTION(opcode) => opcode.definition(),
            EXCEPTION::UDF_UNDEFINED(opcode) => opcode.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            EXCEPTION::BRK_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::DCPS1_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::DCPS2_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::DCPS3_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::HLT_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::HVC_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::SMC_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::SVC_EXCEPTION(opcode) => opcode.bits(),
            EXCEPTION::UDF_UNDEFINED(opcode) => opcode.bits(),
        }
    }
}
impl InsnOpcode for Operation {
    fn definition(&self) -> &'static Insn {
        match self {
            Operation::EXCEPTION(class) => class.definition(),
        }
    }
    fn bits(&self) -> u32 {
        match self {
            Operation::EXCEPTION(class) => class.bits(),
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
            if insn & 0x4000000 == 0 {
                if insn & 0xffff0000 == 0x000000 {
                    return Some(UDF_UNDEFINED::make_opcode(insn));
                }
            } else {
                if insn & 0x000001 == 0 {
                    if insn & 0xffe0001f == 0xd4000002 {
                        return Some(HVC_EXCEPTION::make_opcode(insn));
                    }
                } else {
                    if insn & 0x000002 == 0 {
                        if insn & 0xffe0001f == 0xd4000001 {
                            return Some(SVC_EXCEPTION::make_opcode(insn));
                        }
                    } else {
                        if insn & 0xffe0001f == 0xd4000003 {
                            return Some(SMC_EXCEPTION::make_opcode(insn));
                        }
                    }
                }
            }
        } else {
            if insn & 0xffe0001f == 0xd4400000 {
                return Some(HLT_EXCEPTION::make_opcode(insn));
            }
        }
    } else {
        if insn & 0x000001 == 0 {
            if insn & 0x000002 == 0 {
                if insn & 0xffe0001f == 0xd4200000 {
                    return Some(BRK_EXCEPTION::make_opcode(insn));
                }
            } else {
                if insn & 0xffe0001f == 0xd4a00002 {
                    return Some(DCPS2_EXCEPTION::make_opcode(insn));
                }
            }
        } else {
            if insn & 0x000002 == 0 {
                if insn & 0xffe0001f == 0xd4a00001 {
                    return Some(DCPS1_EXCEPTION::make_opcode(insn));
                }
            } else {
                if insn & 0xffe0001f == 0xd4a00003 {
                    return Some(DCPS3_EXCEPTION::make_opcode(insn));
                }
            }
        }
    }
    None
}
