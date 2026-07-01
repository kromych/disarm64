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
#[doc = r" The decoded instruction definitions referenced by the decoder."]
static INSNS: [Insn; 9] = [
    Insn {
        mnemonic: "brk",
        aliases: &[],
        opcode: 0xd4200000,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "dcps1",
        aliases: &[],
        opcode: 0xd4a00001,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "dcps2",
        aliases: &[],
        opcode: 0xd4a00002,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "dcps3",
        aliases: &[],
        opcode: 0xd4a00003,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "hlt",
        aliases: &[],
        opcode: 0xd4400000,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "hvc",
        aliases: &[],
        opcode: 0xd4000002,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "smc",
        aliases: &[],
        opcode: 0xd4000003,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "svc",
        aliases: &[],
        opcode: 0xd4000001,
        mask: 0xffe0001f,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_0,
        flags: InsnFlags::empty(),
    },
    Insn {
        mnemonic: "udf",
        aliases: &[],
        opcode: 0x000000,
        mask: 0xffff0000,
        class: InsnClass::EXCEPTION,
        feature_set: InsnFeatureSet::V8,
        operands: OPERANDS_1,
        flags: InsnFlags::empty(),
    },
];
pub fn decode(insn: u32) -> Option<Opcode> {
    if insn & 0x200000 == 0 {
        if insn & 0x400000 == 0 {
            if insn & 0x4000000 == 0 {
                if insn & 0xffff0000 == 0x000000 {
                    return Some(Opcode {
                        bits: insn,
                        def: &INSNS[8],
                    });
                }
            } else {
                if insn & 0x000001 == 0 {
                    if insn & 0xffe0001f == 0xd4000002 {
                        return Some(Opcode {
                            bits: insn,
                            def: &INSNS[5],
                        });
                    }
                } else {
                    if insn & 0x000002 == 0 {
                        if insn & 0xffe0001f == 0xd4000001 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[7],
                            });
                        }
                    } else {
                        if insn & 0xffe0001f == 0xd4000003 {
                            return Some(Opcode {
                                bits: insn,
                                def: &INSNS[6],
                            });
                        }
                    }
                }
            }
        } else {
            if insn & 0xffe0001f == 0xd4400000 {
                return Some(Opcode {
                    bits: insn,
                    def: &INSNS[4],
                });
            }
        }
    } else {
        if insn & 0x000001 == 0 {
            if insn & 0x000002 == 0 {
                if insn & 0xffe0001f == 0xd4200000 {
                    return Some(Opcode {
                        bits: insn,
                        def: &INSNS[0],
                    });
                }
            } else {
                if insn & 0xffe0001f == 0xd4a00002 {
                    return Some(Opcode {
                        bits: insn,
                        def: &INSNS[2],
                    });
                }
            }
        } else {
            if insn & 0x000002 == 0 {
                if insn & 0xffe0001f == 0xd4a00001 {
                    return Some(Opcode {
                        bits: insn,
                        def: &INSNS[1],
                    });
                }
            } else {
                if insn & 0xffe0001f == 0xd4a00003 {
                    return Some(Opcode {
                        bits: insn,
                        def: &INSNS[3],
                    });
                }
            }
        }
    }
    None
}
