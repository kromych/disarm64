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
#[doc = r" Define instruction newtype structs with Debug impl."]
macro_rules ! define_insn_types { ($ ($ name : ident) , * $ (,) ?) => { $ (# [derive (Copy , Clone , PartialEq , Eq)] pub struct $ name (pub u32) ; impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{}({:#010x})" , stringify ! ($ name) , self . 0) } }) * } ; }
#[doc = r" Define DEFINITION, make_opcode, and InsnOpcode for each instruction struct."]
macro_rules ! define_insn_impls { ($ ($ name : ident ($ mnemonic_str : expr , $ mnemonic_ident : ident , $ opcode : expr , $ mask : expr , $ class : ident , $ feature_set : ident , $ flags : expr , $ operands : expr)) , * $ (,) ?) => { $ (impl $ name { pub const DEFINITION : Insn = Insn { mnemonic : $ mnemonic_str , aliases : & [] , opcode : $ opcode , mask : $ mask , class : InsnClass :: $ class , feature_set : InsnFeatureSet :: $ feature_set , operands : $ operands , flags : $ flags , } ; fn make_opcode (bits : u32) -> Opcode { Opcode { mnemonic : Mnemonic :: $ mnemonic_ident , operation : Operation :: $ class ($ class :: $ name ($ name (bits))) } } } impl InsnOpcode for $ name { fn definition (& self) -> & 'static Insn { & Self :: DEFINITION } fn bits (& self) -> u32 { self . 0 } }) * } ; }
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
define_insn_types!(
    BRK_EXCEPTION,
    DCPS1_EXCEPTION,
    DCPS2_EXCEPTION,
    DCPS3_EXCEPTION,
    HLT_EXCEPTION,
    HVC_EXCEPTION,
    SMC_EXCEPTION,
    SVC_EXCEPTION,
    UDF_UNDEFINED
);
const BITFIELDS_0: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm16_5,
    lsb: 5,
    width: 16,
}];
const BITFIELDS_1: &[BitfieldSpec] = &[BitfieldSpec {
    bitfield: InsnBitField::imm16_0,
    lsb: 0,
    width: 16,
}];
const OPERANDS_0: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::EXCEPTION,
    class: InsnOperandClass::IMMEDIATE,
    qualifiers: &[],
    bit_fields: BITFIELDS_0,
}];
const OPERANDS_1: &[InsnOperand] = &[InsnOperand {
    kind: InsnOperandKind::UNDEFINED,
    class: InsnOperandClass::IMMEDIATE,
    qualifiers: &[],
    bit_fields: BITFIELDS_1,
}];
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
define_insn_impls!(
    BRK_EXCEPTION(
        "brk",
        r#brk,
        0xd4200000,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    DCPS1_EXCEPTION(
        "dcps1",
        r#dcps1,
        0xd4a00001,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    DCPS2_EXCEPTION(
        "dcps2",
        r#dcps2,
        0xd4a00002,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    DCPS3_EXCEPTION(
        "dcps3",
        r#dcps3,
        0xd4a00003,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    HLT_EXCEPTION(
        "hlt",
        r#hlt,
        0xd4400000,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    HVC_EXCEPTION(
        "hvc",
        r#hvc,
        0xd4000002,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    SMC_EXCEPTION(
        "smc",
        r#smc,
        0xd4000003,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    SVC_EXCEPTION(
        "svc",
        r#svc,
        0xd4000001,
        0xffe0001f,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_0
    ),
    UDF_UNDEFINED(
        "udf",
        r#udf,
        0x000000,
        0xffff0000,
        EXCEPTION,
        V8,
        InsnFlags::empty(),
        OPERANDS_1
    )
);
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
