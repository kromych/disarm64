//! This module provides operand definitions used for structured matching
//! after decoding.

use crate::bit_range;
use crate::bit_set;
use crate::decode_limm;
use crate::fp_expand_imm;
use crate::registers::get_int_reg_name;
use crate::registers::get_simd_reg_name;
use crate::registers::get_sys_reg_name;
use crate::registers::sys_reg_number;
use crate::registers::FpRegSize;
use crate::registers::SimdRegArrangement;
use crate::sign_extend;
use crate::LOG2_TAG_GRANULE;
use bitfield_struct::bitfield;
use defn::InsnOpcode;
use disarm64_defn::defn;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegKind {
    GeneralPurpose,
    System,
    FloatingPoint,
    Vector,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImmediateKind {
    Signed(i64),
    Unsigned(u64),
    FloatingPoint(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddressKind {
    PcRelative(u64),
    PostIndex,
    PreIndex,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SystemHintKind {
    nop,
    r#yield,
    wfe,
    wfi,
    sev,
    sevl,
    dgh,
    xpaclri,
    pacia1716,
    pacib1716,
    autia1716,
    autib1716,
    esb,
    psb_csync,
    tsb_csync,
    csdb,
    clrbhb,
    gcsb_dsync,
    paciaz,
    paciasp,
    pacibz,
    pacibsp,
    autiaz,
    autiasp,
    autibz,
    autibsp,
    bti,
    bti_c,
    bti_j,
    bti_jc,
    hint(u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ConditionKind {
    eq,
    ne,
    cs,
    cc,
    mi,
    pl,
    vs,
    vc,
    hi,
    ls,
    ge,
    lt,
    gt,
    le,
    al,
    nv,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(RegKind),
    Immediate(ImmediateKind),
    Address(AddressKind),
    SystemHint(SystemHintKind),
    Condition(ConditionKind),
    Unknown,
    Other(&'static str),
}

#[cfg(any(feature = "full", feature = "system"))]
fn operand_hint(bits: u32) -> SystemHintKind {
    let hint = bit_range(bits, 5, 7) as u8;

    #[allow(clippy::unusual_byte_groupings)]
    match hint {
        0b0000_000 => SystemHintKind::nop,
        0b0000_001 => SystemHintKind::r#yield,
        0b0000_010 => SystemHintKind::wfe,
        0b0000_011 => SystemHintKind::wfi,
        0b0000_100 => SystemHintKind::sev,
        0b0000_101 => SystemHintKind::sevl,
        0b0000_110 => SystemHintKind::dgh,
        0b0000_111 => SystemHintKind::xpaclri,
        0b0001_000 => SystemHintKind::pacia1716,
        0b0001_010 => SystemHintKind::pacib1716,
        0b0001_100 => SystemHintKind::autia1716,
        0b0001_110 => SystemHintKind::autib1716,
        0b0010_000 => SystemHintKind::esb,
        0b0010_001 => SystemHintKind::psb_csync,
        0b0010_010 => SystemHintKind::tsb_csync,
        0b0010_100 => SystemHintKind::csdb,
        0b0010_110 => SystemHintKind::clrbhb,
        0b0010_011 => SystemHintKind::gcsb_dsync,
        0b0011_000 => SystemHintKind::paciaz,
        0b0011_001 => SystemHintKind::paciasp,
        0b0011_010 => SystemHintKind::pacibz,
        0b0011_011 => SystemHintKind::pacibsp,
        0b0011_100 => SystemHintKind::autiaz,
        0b0011_101 => SystemHintKind::autiasp,
        0b0011_110 => SystemHintKind::autibz,
        0b0011_111 => SystemHintKind::autibsp,
        0b0100_000 => SystemHintKind::bti,
        0b0100_010 => SystemHintKind::bti_c,
        0b0100_100 => SystemHintKind::bti_j,
        0b0100_110 => SystemHintKind::bti_jc,
        _ => SystemHintKind::hint(hint),
    }
}

#[cfg(feature = "full")]
fn operand_condition(bits: u32) -> ConditionKind {
    let cond = bit_range(bits, 0, 4);
    match cond {
        0x0 => ConditionKind::eq,
        0x1 => ConditionKind::ne,
        0x2 => ConditionKind::cs,
        0x3 => ConditionKind::cc,
        0x4 => ConditionKind::mi,
        0x5 => ConditionKind::pl,
        0x6 => ConditionKind::vs,
        0x7 => ConditionKind::vc,
        0x8 => ConditionKind::hi,
        0x9 => ConditionKind::ls,
        0xa => ConditionKind::ge,
        0xb => ConditionKind::lt,
        0xc => ConditionKind::gt,
        0xd => ConditionKind::le,
        0xe => ConditionKind::al,
        0xf => ConditionKind::nv,
        _ => unreachable!(),
    }
}

/// Produce a floating-point register operand
fn operand_fp_reg(bits: u32, operand: &defn::InsnOperand, definition: &defn::Insn) -> Operand {
    let kind = operand.kind;

    let reg_no = if let Some(bit_filed) = operand.bit_fields.first() {
        bit_range(bits, bit_filed.lsb.into(), bit_filed.width.into())
    } else {
        return write!(f, ":{kind:?}:");
    };

    let fp_reg_name = match definition.class {
        InsnClass::LDST_IMM9
        | InsnClass::LDST_POS
        | InsnClass::LDST_REGOFF
        | InsnClass::LDST_UNSCALED => {
            let size = bit_range(bits, 30, 2);
            let opc = bit_range(bits, 22, 2);
            if opc == 0 || opc == 1 {
                let fp_size = match size {
                    0b00 => FpRegSize::B8,
                    0b01 => FpRegSize::H16,
                    0b10 => FpRegSize::S32,
                    0b11 => FpRegSize::D64,
                    _ => unreachable!(),
                };
                get_fp_reg_name(fp_size, reg_no as usize)
            } else if (opc == 0b10 || opc == 0b11) && size == 0 {
                get_fp_reg_name(FpRegSize::Q128, reg_no as usize)
            } else {
                return write!(f, "<undefined>");
            }
        }
        InsnClass::LDSTPAIR_OFF
        | InsnClass::LDSTNAPAIR_OFFS
        | InsnClass::LDSTPAIR_INDEXED
        | InsnClass::LOADLIT => {
            let opc = bit_range(bits, 30, 2);
            if opc == 0 {
                get_fp_reg_name(FpRegSize::S32, reg_no as usize)
            } else if opc == 1 {
                get_fp_reg_name(FpRegSize::D64, reg_no as usize)
            } else if opc == 2 {
                get_fp_reg_name(FpRegSize::Q128, reg_no as usize)
            } else {
                return write!(f, "<undefined>");
            }
        }
        InsnClass::BFLOAT16 => match kind {
            InsnOperandKind::Fd => get_fp_reg_name(FpRegSize::H16, reg_no as usize),
            InsnOperandKind::Fn => get_fp_reg_name(FpRegSize::S32, reg_no as usize),
            _ => {
                return write!(f, "<undefined>");
            }
        },

        _ => {
            if definition.flags.contains(InsnFlags::HAS_FPTYPE_FIELD) {
                let fp_type = bit_range(bits, 22, 2);
                match fp_type {
                    0b00 => get_fp_reg_name(FpRegSize::S32, reg_no as usize),
                    0b01 => get_fp_reg_name(FpRegSize::D64, reg_no as usize),
                    0b10 => "<undefined>",
                    0b11 => get_fp_reg_name(FpRegSize::H16, reg_no as usize),
                    _ => unreachable!(),
                }
            } else if let Some(qual) = operand.qualifiers.first() {
                let size = match qual {
                    InsnOperandQualifier::S_B => FpRegSize::B8,
                    InsnOperandQualifier::S_H => FpRegSize::H16,
                    InsnOperandQualifier::S_S => FpRegSize::S32,
                    InsnOperandQualifier::S_D => FpRegSize::D64,
                    InsnOperandQualifier::S_Q => FpRegSize::Q128,
                    _ => {
                        return write!(f, "<undefined>");
                    }
                };
                get_fp_reg_name(size, reg_no as usize)
            } else {
                return write!(f, ":{kind:?}:");
            }
        }
    };

    write!(f, "{fp_reg_name}")
}

/// Produce an integer register (32-bit or 64-bit) operand
fn operand_reg_int_pair(
    pair: bool,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    with_zr: bool,
) -> core::fmt::Result {
    let flags = definition.flags;
    let is_64 = if flags.contains(InsnFlags::HAS_SF_FIELD) {
        bit_set(bits, 31)
    } else if flags.contains(InsnFlags::HAS_LDS_SIZE_IN_BIT_22) {
        !bit_set(bits, 22)
    } else if (flags.contains(InsnFlags::HAS_LSE_SZ_FIELD)
        && (operand.kind == InsnOperandKind::Rt || operand.kind == InsnOperandKind::Rs))
        || (flags.contains(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q)
            && (operand.kind == InsnOperandKind::Rt || operand.kind == InsnOperandKind::Rt2))
    {
        bit_set(bits, 30)
    } else if operand.qualifiers.is_empty() || operand.qualifiers == [InsnOperandQualifier::X] {
        true
    } else if definition.class == InsnClass::LDST_IMM9
        || definition.class == InsnClass::LDST_POS
        || definition.class == InsnClass::LDST_REGOFF
        || definition.class == InsnClass::LDST_UNPRIV
        || definition.class == InsnClass::LDST_UNSCALED
    {
        let size = bit_range(bits, 30, 2);
        let opc1 = bit_set(bits, 23);
        let opc0 = bit_set(bits, 22);
        if !opc1 {
            // Store or zero-extending load, not signed.
            // Bit 22 is set if this is a load.
            size == 0b11
        } else {
            // Sign-extending load
            if size == 0b10 && opc0 {
                return write!(f, "<undefined>");
            }
            !opc0
        }
    } else {
        false
    };

    if let Some(bit_field_spec) = operand.bit_fields.first() {
        let reg_no = bit_range(bits, bit_field_spec.lsb.into(), bit_field_spec.width.into());
        let reg_no = if pair {
            if reg_no & 1 != 0 {
                return write!(f, "<undefined>");
            }
            reg_no + 1
        } else {
            reg_no
        };
        let reg_name = get_int_reg_name(is_64, reg_no as u8, with_zr);

        write!(f, "{reg_name}")
    } else {
        write!(f, "<undefined>")
    }
}

/// Produce an integer register (32-bit or 64-bit) operand
fn operand_int_operand_reg(
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    with_zr: bool,
) -> core::fmt::Result {
    operand_int_reg_pair(false, bits, operand, definition, with_zr)
}

/// Produce a register operand with extended shift operand to a string.
/// This is a lot of logic and dependencies on the other registers
/// for 4 instructions and 2 aliases.
fn operand_reg_ext(bits: u32) -> Operand {
    // E.g. for ADD (extended register) instruction:
    // https://developer.arm.com/documentation/ddi0602/2023-12/Base-Instructions/ADD--extended-register---Add--extended-register--

    // The bitfields don't come from the operand (might fix up the description in the future).
    #[bitfield(u32)]
    struct RegExt {
        #[bits(5)]
        regd: u32,
        #[bits(5)]
        regn: u32,
        #[bits(3)]
        imm3: u32,
        #[bits(3)]
        option: u32,
        #[bits(5)]
        regm: u32,
        #[bits(10)]
        opcode_bits: u32,
        #[bits(1)]
        sf: bool,
    }
    let reg_ext = RegExt::from_bits(bits);

    if reg_ext.imm3() > 4 {
        write!(f, "<undefined>")?;
        return Ok(());
    }

    let extend_use_lsl = if !reg_ext.sf() { 0b010 } else { 0b011 };
    let extend =
        if (reg_ext.regd() == 31 || reg_ext.regn() == 31) && reg_ext.option() == extend_use_lsl {
            if reg_ext.imm3() != 0 {
                // Logical Shift Left (LSL) by the immediate value.
                "lsl"
            } else {
                ""
            }
        } else {
            // Signed or Unsigned eXTend of a Byte, Halfword, Word, or (X) Doubleword.
            match reg_ext.option() {
                0b000 => "uxtb",
                0b001 => "uxth",
                0b010 => "uxtw",
                0b011 => "uxtx",
                0b100 => "sxtb",
                0b101 => "sxth",
                0b110 => "sxtw",
                0b111 => "sxtx",
                _ => unreachable!(),
            }
        };
    let is_64bit = if reg_ext.sf() {
        reg_ext.option() & 0b11 == 0b11
    } else {
        false
    };

    let reg_name = get_int_reg_name(is_64bit, reg_ext.regm() as u8, true);
    write!(f, "{reg_name}")?;
    if !extend.is_empty() {
        write!(f, ", {}", extend)?;

        if reg_ext.imm3() != 0 {
            write!(f, " #{}", reg_ext.imm3())?;
        }
    }

    Ok(())
}

/// Produce a register operand with shift operand to a string.
/// An example: https://developer.arm.com/documentation/ddi0602/2023-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--
fn operand_reg_shift(bits: u32) -> Operand {
    // The bitfields don't come from the operand (might fix up the description in the future).
    #[bitfield(u32)]
    struct RegShift {
        #[bits(5)]
        regd: u32,
        #[bits(5)]
        regn: u32,
        #[bits(6)]
        imm6: u32,
        #[bits(5)]
        regm: u32,
        #[bits(1)]
        zero: u32,
        #[bits(2)]
        shift: u32,
        #[bits(7)]
        opcode_bits: u32,
        #[bits(1)]
        sf: bool,
    }
    let reg_shift = RegShift::from_bits(bits);
    if !reg_shift.sf() && reg_shift.imm6() & 0b100000 != 0 {
        return write!(f, "<undefined>");
    }

    let shift = match reg_shift.shift() {
        // Logical Shift Left (LSL) by the immediate value.
        0b00 => "lsl",
        // Logical Shift Right (LSR) by the immediate value.
        0b01 => "lsr",
        // Arithmetic Shift Right (ASL) by the immediate value.
        0b10 => "asr",
        // Rotate Right (ROR) by the immediate value.
        0b11 => {
            if !bit_set(bits, 24) {
                // Logical insn
                "ror"
            } else {
                // Arithmetic insn
                return write!(f, "<undefined>");
            }
        }
        _ => unreachable!(),
    };

    let reg_name = get_int_reg_name(reg_shift.sf(), reg_shift.regm() as u8, true);
    write!(f, "{reg_name}")?;
    if reg_shift.imm6() != 0 || shift != "lsl" {
        write!(f, ", {shift} #{}", reg_shift.imm6())?;
    }

    Ok(())
}

/// Produce a SIMD register operand
fn operand_simd_reg(bits: u32, operand: &defn::InsnOperand, definition: &defn::Insn) -> Operand {
    let kind = operand.kind;

    let reg_no = if let Some(bit_filed) = operand.bit_fields.first() {
        bit_range(bits, bit_filed.lsb.into(), bit_filed.width.into())
    } else {
        return write!(f, ":{kind:?}:");
    } as u8;

    let simd_reg_arrangement = if let Some(qual) = operand.qualifiers.first() {
        let double = if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD) {
            bit_set(bits, 30)
        } else {
            false
        };
        if !double {
            match qual {
                InsnOperandQualifier::V_8B => SimdRegArrangement::Vector8B,
                InsnOperandQualifier::V_16B => SimdRegArrangement::Vector16B,
                InsnOperandQualifier::V_2H => SimdRegArrangement::Vector2H,
                InsnOperandQualifier::V_4H => SimdRegArrangement::Vector4H,
                InsnOperandQualifier::V_8H => SimdRegArrangement::Vector8H,
                InsnOperandQualifier::V_2S => SimdRegArrangement::Vector2S,
                InsnOperandQualifier::V_4S => SimdRegArrangement::Vector4S,
                InsnOperandQualifier::V_1D => SimdRegArrangement::Vector1D,
                InsnOperandQualifier::V_2D => SimdRegArrangement::Vector2D,
                InsnOperandQualifier::V_1Q => SimdRegArrangement::Vector1Q,
                _ => {
                    return write!(f, "<undefined>");
                }
            }
        } else {
            match qual {
                InsnOperandQualifier::V_8B => SimdRegArrangement::Vector16B,
                InsnOperandQualifier::V_2H => SimdRegArrangement::Vector4H,
                InsnOperandQualifier::V_4H => SimdRegArrangement::Vector8H,
                InsnOperandQualifier::V_2S => SimdRegArrangement::Vector4S,
                InsnOperandQualifier::V_1D => SimdRegArrangement::Vector2D,
                _ => {
                    return write!(f, "<undefined>");
                }
            }
        }
    } else {
        return write!(f, "<undefined>");
    };
    let simd_reg_name = get_simd_reg_name(reg_no, simd_reg_arrangement);

    write!(f, "{simd_reg_name}")
}

/// Produce an operand from the instruction bits and the definition.
fn operand_get_by_class(
    pos: usize,
    pc: u64,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    stop: &mut bool,
) -> Operand {
    let kind = operand.kind;
    match kind {
        InsnOperandKind::Rd
        | InsnOperandKind::Rn
        | InsnOperandKind::Rm
        | InsnOperandKind::Rt
        | InsnOperandKind::Rt2
        | InsnOperandKind::Rs
        | InsnOperandKind::Ra
        | InsnOperandKind::Rt_LS64
        | InsnOperandKind::Rt_SYS
        | InsnOperandKind::SVE_Rm
        | InsnOperandKind::LSE128_Rt
        | InsnOperandKind::LSE128_Rt2 => {
            let with_zr = true;
            operand_int_reg(bits, operand, definition, with_zr)
        }

        InsnOperandKind::PAIRREG | InsnOperandKind::PAIRREG_OR_XZR => {
            if pos == 0 {
                return Operand::Unknown;
            }

            let prev_operand = &definition.operands[pos - 1];
            let pair = true;
            operand_reg_int_pair(pair, bits, prev_operand, definition, true);
        }

        InsnOperandKind::Rd_SP
        | InsnOperandKind::Rn_SP
        | InsnOperandKind::Rt_SP
        | InsnOperandKind::SVE_Rn_SP
        | InsnOperandKind::Rm_SP => {
            let with_zr = false;
            operand_int_reg(bits, operand, definition, with_zr)
        }

        InsnOperandKind::Rm_EXT => operand_reg_ext(bits),

        InsnOperandKind::Rm_SFT => operand_reg_shift(bits),

        InsnOperandKind::Fd
        | InsnOperandKind::Fn
        | InsnOperandKind::Fm
        | InsnOperandKind::Fa
        | InsnOperandKind::Ft
        | InsnOperandKind::Ft2 => operand_fp_reg(bits, operand, definition),

        #[cfg(feature = "full")]
        InsnOperandKind::Sd
        | InsnOperandKind::Sn
        | InsnOperandKind::Sm
        | InsnOperandKind::SVE_VZn
        | InsnOperandKind::SVE_Vd
        | InsnOperandKind::SVE_Vm
        | InsnOperandKind::SVE_Vn => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::Va | InsnOperandKind::Vd | InsnOperandKind::Vn | InsnOperandKind::Vm => {
            operand_simd_reg(bits, operand, definition)
        }

        #[cfg(feature = "full")]
        InsnOperandKind::Ed | InsnOperandKind::En | InsnOperandKind::Em | InsnOperandKind::Em16 => {
            Operand::Other(kind.as_ref())
        }

        #[cfg(feature = "full")]
        InsnOperandKind::VdD1 | InsnOperandKind::VnD1 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::LVn
        | InsnOperandKind::LVt
        | InsnOperandKind::LVt_AL
        | InsnOperandKind::LEt => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Pd
        | InsnOperandKind::SVE_Pg3
        | InsnOperandKind::SVE_Pg4_5
        | InsnOperandKind::SVE_Pg4_10
        | InsnOperandKind::SVE_Pg4_16
        | InsnOperandKind::SVE_Pm
        | InsnOperandKind::SVE_Pn
        | InsnOperandKind::SVE_Pt
        | InsnOperandKind::SME_Pm => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PNd
        | InsnOperandKind::SVE_PNg4_10
        | InsnOperandKind::SVE_PNn
        | InsnOperandKind::SVE_PNt
        | InsnOperandKind::SME_PNd3
        | InsnOperandKind::SME_PNg3
        | InsnOperandKind::SME_PNn => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_Pdx2 | InsnOperandKind::SME_PdxN => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_PNn3_INDEX1 | InsnOperandKind::SME_PNn3_INDEX2 => {
            Operand::Other(kind.as_ref())
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Za_5
        | InsnOperandKind::SVE_Za_16
        | InsnOperandKind::SVE_Zd
        | InsnOperandKind::SVE_Zm_5
        | InsnOperandKind::SVE_Zm_16
        | InsnOperandKind::SVE_Zn
        | InsnOperandKind::SVE_Zt
        | InsnOperandKind::SME_Zm => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ZnxN
        | InsnOperandKind::SVE_ZtxN
        | InsnOperandKind::SME_Zdnx2
        | InsnOperandKind::SME_Zdnx4
        | InsnOperandKind::SME_Zmx2
        | InsnOperandKind::SME_Zmx4
        | InsnOperandKind::SME_Znx2
        | InsnOperandKind::SME_Znx4
        | InsnOperandKind::SME_Ztx2_STRIDED
        | InsnOperandKind::SME_Ztx4_STRIDED
        | InsnOperandKind::SME_Zt2
        | InsnOperandKind::SME_Zt3
        | InsnOperandKind::SME_Zt4 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Zm3_INDEX
        | InsnOperandKind::SVE_Zm3_22_INDEX
        | InsnOperandKind::SVE_Zm3_19_INDEX
        | InsnOperandKind::SVE_Zm3_11_INDEX
        | InsnOperandKind::SVE_Zm4_11_INDEX
        | InsnOperandKind::SVE_Zm4_INDEX
        | InsnOperandKind::SVE_Zn_INDEX
        | InsnOperandKind::SME_Zm_INDEX1
        | InsnOperandKind::SME_Zm_INDEX2
        | InsnOperandKind::SME_Zm_INDEX3_1
        | InsnOperandKind::SME_Zm_INDEX3_2
        | InsnOperandKind::SME_Zm_INDEX3_10
        | InsnOperandKind::SVE_Zn_5_INDEX
        | InsnOperandKind::SME_Zm_INDEX4_1
        | InsnOperandKind::SME_Zm_INDEX4_10
        | InsnOperandKind::SME_Zn_INDEX1_16
        | InsnOperandKind::SME_Zn_INDEX2_15
        | InsnOperandKind::SME_Zn_INDEX2_16
        | InsnOperandKind::SME_Zn_INDEX3_14
        | InsnOperandKind::SME_Zn_INDEX3_15
        | InsnOperandKind::SME_Zn_INDEX4_14
        | InsnOperandKind::SVE_Zm_imm4 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZAda_2b | InsnOperandKind::SME_ZAda_3b => {
            Operand::Other(kind.as_ref())
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_HV_idx_src
        | InsnOperandKind::SME_ZA_HV_idx_srcxN
        | InsnOperandKind::SME_ZA_HV_idx_dest
        | InsnOperandKind::SME_ZA_HV_idx_destxN
        | InsnOperandKind::SME_ZA_HV_idx_ldstr => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_list_of_64bit_tiles => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_array_off1x4
        | InsnOperandKind::SME_ZA_array_off2x2
        | InsnOperandKind::SME_ZA_array_off2x4
        | InsnOperandKind::SME_ZA_array_off3_0
        | InsnOperandKind::SME_ZA_array_off3_5
        | InsnOperandKind::SME_ZA_array_off3x2
        | InsnOperandKind::SME_ZA_array_off4 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_array_vrsb_1
        | InsnOperandKind::SME_ZA_array_vrsh_1
        | InsnOperandKind::SME_ZA_array_vrss_1
        | InsnOperandKind::SME_ZA_array_vrsd_1
        | InsnOperandKind::SME_ZA_array_vrsb_2
        | InsnOperandKind::SME_ZA_array_vrsh_2
        | InsnOperandKind::SME_ZA_array_vrss_2
        | InsnOperandKind::SME_ZA_array_vrsd_2 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_SM_ZA => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_PnT_Wm_imm => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_VLxN_10 | InsnOperandKind::SME_VLxN_13 => {
            Operand::Other(kind.as_ref())
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::CRn => Operand::Unknown("c{}", bit_range(bits, 12, 4)),
        InsnOperandKind::CRm => Operand::Unknown("c{}", bit_range(bits, 8, 4)),

        #[cfg(feature = "full")]
        InsnOperandKind::IMMR => {
            let immr = bit_range(bits, 16, 6);
            let n = bit_set(bits, 22);
            let sf = bit_set(bits, 31);

            if (sf && !n) || (!sf && (n || bit_set(immr, 5))) {
                return Operand::Unknown;
            }
            Operand::Unknown("#{}", immr)
        }
        #[cfg(feature = "full")]
        InsnOperandKind::IMMS => {
            let imms = bit_range(bits, 10, 6);
            let n = bit_set(bits, 22);
            let sf = bit_set(bits, 31);

            if (sf && !n) || (!sf && (n || bit_set(imms, 5))) {
                return Operand::Unknown;
            }
            Operand::Unknown("#{}", imms)
        }

        #[cfg(feature = "full")]
        InsnOperandKind::BIT_NUM => {
            let b5 = bit_range(bits, 31, 1);
            let b40 = bit_range(bits, 19, 5);
            let bit_num = (b5 << 5) | b40;
            Operand::Unknown("#{}", bit_num)
        }

        #[cfg(feature = "full")]
        InsnOperandKind::FBITS => {
            let ftype = bit_range(bits, 22, 2);
            if ftype == 0b10 {
                return Operand::Unknown;
            }
            let sf = bit_set(bits, 31);
            let scale = 64 - bit_range(bits, 10, 6);
            if !sf && scale > 32 {
                return Operand::Unknown;
            }
            Operand::Unknown("#{scale}")
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IMM0 => Operand::Unknown("#0"),
        #[cfg(feature = "full")]
        InsnOperandKind::TME_UIMM16 => Operand::Unknown("#{}", bit_range(bits, 5, 16)),

        #[cfg(feature = "full")]
        InsnOperandKind::IDX
        | InsnOperandKind::IMM
        | InsnOperandKind::WIDTH
        | InsnOperandKind::IMM_VLSL
        | InsnOperandKind::IMM_VLSR
        | InsnOperandKind::SHLL_IMM
        | InsnOperandKind::SIMM5
        | InsnOperandKind::SME_SHRIMM4
        | InsnOperandKind::SME_SHRIMM5
        | InsnOperandKind::SVE_SHLIMM_PRED
        | InsnOperandKind::SVE_SHLIMM_UNPRED
        | InsnOperandKind::SVE_SHLIMM_UNPRED_22
        | InsnOperandKind::SVE_SHRIMM_PRED
        | InsnOperandKind::SVE_SHRIMM_UNPRED
        | InsnOperandKind::SVE_SHRIMM_UNPRED_22
        | InsnOperandKind::SVE_SIMM5
        | InsnOperandKind::SVE_SIMM5B
        | InsnOperandKind::SVE_SIMM6
        | InsnOperandKind::SVE_SIMM8
        | InsnOperandKind::SVE_UIMM3
        | InsnOperandKind::SVE_UIMM7
        | InsnOperandKind::SVE_UIMM8
        | InsnOperandKind::SVE_UIMM8_53
        | InsnOperandKind::IMM_ROT1
        | InsnOperandKind::IMM_ROT2
        | InsnOperandKind::IMM_ROT3
        | InsnOperandKind::SVE_IMM_ROT1
        | InsnOperandKind::SVE_IMM_ROT2
        | InsnOperandKind::SVE_IMM_ROT3 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_SIMM8 => Operand::Unknown("#{}", bit_range(bits, 10, 8) as i8),
        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_UIMM8 => Operand::Unknown("#{}", bit_range(bits, 10, 8) as u8),
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM4_ADDG => Operand::Unknown("#{}", bit_range(bits, 10, 4)),
        // The 10 bit size comes from 6 bit in the instruction and the shift of 4.
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM10 => {
            Operand::Unknown("#{}", bit_range(bits, 16, 6) << LOG2_TAG_GRANULE)
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_I1_HALF_ONE
        | InsnOperandKind::SVE_I1_HALF_TWO
        | InsnOperandKind::SVE_I1_ZERO_ONE => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PATTERN => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PATTERN_SCALED => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PRFOP => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::IMM_MOV => Operand::Other(kind.as_ref()),
        #[cfg(feature = "full")]
        InsnOperandKind::FPIMM0 => Operand::Unknown("#{:.1}", 0.0),

        #[cfg(feature = "full")]
        InsnOperandKind::AIMM => {
            let shift = bit_set(bits, 22);
            let imm12 = bit_range(bits, 10, 12);
            Operand::Unknown("#{imm12:#x}");

            if shift {
                return Operand::Unknown(", lsl #12");
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::HALF => {
            let hw = bit_range(bits, 21, 2);
            if !bit_set(bits, 31) && bit_set(hw, 1) {
                return Operand::Unknown;
            }

            let imm16 = bit_range(bits, 5, 16);
            let shift = hw << 4;

            Operand::Unknown("#{imm16:#x}, lsl #{shift:#x}")
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::LIMM => {
            let imms = bit_range(bits, 10, 6);
            let immr = bit_range(bits, 16, 6);
            let n = bit_range(bits, 22, 1);
            let is_64bit = bit_set(bits, 31);
            let byte_count = if is_64bit { 8 } else { 4 };
            if let Some(imm) = decode_limm(byte_count, n, immr, imms) {
                Operand::Unknown("#{imm:#x}")
            } else {
                Operand::Unknown
            }
        }
        #[cfg(feature = "full")]
        InsnOperandKind::SVE_INV_LIMM
        | InsnOperandKind::SVE_LIMM
        | InsnOperandKind::SVE_LIMM_MOV => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_IMM | InsnOperandKind::SIMD_IMM_SFT => Operand::Other(kind.as_ref()),
        #[cfg(feature = "full")]
        InsnOperandKind::SVE_AIMM | InsnOperandKind::SVE_ASIMM => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::FPIMM | InsnOperandKind::SIMD_FPIMM | InsnOperandKind::SVE_FPIMM8 => {
            let fp_type = bit_range(bits, 22, 2);
            let size = match fp_type {
                0b00 => 4,
                0b01 => 8,
                0b10 => return Operand::Unknown,
                0b11 => 2,
                _ => unreachable!(),
            };
            let imm8 = bit_range(bits, 13, 8);
            if let Some(imm) = fp_expand_imm(size, imm8) {
                Operand::Unknown("#{}", imm)
            } else {
                Operand::Unknown
            }
        }

        #[cfg(any(feature = "full", feature = "exception"))]
        InsnOperandKind::EXCEPTION => {
            let imm16 = bit_range(bits, 5, 16);
            Operand::Unknown("#{imm16:#x}")
        }
        #[cfg(any(feature = "full", feature = "exception"))]
        InsnOperandKind::UNDEFINED => {
            let imm16 = bit_range(bits, 0, 16);
            Operand::Unknown("#{imm16:#x}")
        }

        #[cfg(feature = "full")]
        InsnOperandKind::CCMP_IMM => {
            let imm5 = bit_range(bits, 16, 5);
            Operand::Unknown("#{imm5:#x}")
        }

        #[cfg(feature = "full")]
        InsnOperandKind::NZCV => {
            let imm4 = bit_range(bits, 0, 4);
            Operand::Unknown("#{imm4:#x}")
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::IMM_2 => {
            let imm = bit_range(bits, 15, 6);
            Operand::Unknown("#{}", imm)
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::MASK => {
            let mask = bit_range(bits, 0, 4);
            Operand::Unknown("#{}", mask)
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM3_OP1 => {
            let imm = bit_range(bits, 16, 3);
            Operand::Unknown("#{}", imm)
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM3_OP2 => {
            let imm = bit_range(bits, 5, 3);
            Operand::Unknown("#{}", imm)
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM4 => {
            let imm4 = bit_range(bits, 8, 4);
            Operand::Unknown("#{imm4:#x}")
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_ISB => {
            let imm4 = bit_range(bits, 8, 4);
            if imm4 != 0xf {
                Operand::Unknown("#{imm4:#x}")
            }
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM7 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::COND | InsnOperandKind::COND1 => {
            let cond = bit_range(bits, 12, 4);
            Operand::Unknown("{}", cond_name(cond))
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL14 => {
            let offset = bit_range(bits, 5, 14);
            let offset = sign_extend(offset, 13) << 2;
            Operand::Unknown("{:#x}", pc.wrapping_add(offset))
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL19 => {
            let offset = bit_range(bits, 5, 19);
            let offset = sign_extend(offset, 18) << 2;
            Operand::Unknown("{:#x}", pc.wrapping_add(offset))
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL21 => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20);
            Operand::Unknown("{:#x}", pc.wrapping_add(offset))
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_ADRP => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20) << 12;
            let pc = pc & !((1 << 12) - 1);
            Operand::Unknown("{:#x}", pc.wrapping_add(offset))
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL26 => {
            let offset = bit_range(bits, 0, 26);
            let offset = sign_extend(offset, 25) << 2;
            Operand::Unknown("{:#x}", pc.wrapping_add(offset))
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMPLE | InsnOperandKind::SIMD_ADDR_SIMPLE => {
            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);
            Operand::Unknown("[{reg_name}]")
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_ADDR_POST => Operand::Other(kind.as_ref()),

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_REGOFF => {
            let option = bit_range(bits, 13, 3);
            let size = bit_range(bits, 30, 2);
            let shift = bit_set(bits, 12);

            let is_64bit = option == 0b011 || option == 0b111;

            let fp = bit_set(bits, 26);
            let scale = if fp {
                (bit_range(bits, 23, 1) << 2) | size
            } else {
                size
            };

            let rn_reg_no = bit_range(bits, 5, 5);
            let rn_reg_name = get_int_reg_name(true, rn_reg_no as u8, false);
            let rm_reg_no = bit_range(bits, 16, 5);
            let rm_reg_name = get_int_reg_name(is_64bit, rm_reg_no as u8, true);

            let extend = match option {
                0b010 => "uxtw",
                0b011 => "lsl",
                0b110 => "sxtw",
                0b111 => "sxtx",
                _ => "<undefined>",
            };
            Operand::Unknown("[{rn_reg_name}, {rm_reg_name}");
            if option == 0b011 {
                if shift {
                    Operand::Unknown(", lsl #{scale}")
                }
            } else if option != 0b011 {
                Operand::Unknown(", {extend}");
                if shift {
                    Operand::Unknown(" #{scale}")
                }
            }
            Operand::Unknown("]");

            *stop = true;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_R
        | InsnOperandKind::SVE_ADDR_RR
        | InsnOperandKind::SVE_ADDR_RR_LSL1
        | InsnOperandKind::SVE_ADDR_RR_LSL2
        | InsnOperandKind::SVE_ADDR_RR_LSL3
        | InsnOperandKind::SVE_ADDR_RR_LSL4
        | InsnOperandKind::SVE_ADDR_RX
        | InsnOperandKind::SVE_ADDR_RX_LSL1
        | InsnOperandKind::SVE_ADDR_RX_LSL2
        | InsnOperandKind::SVE_ADDR_RX_LSL3 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZX => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_RZ
        | InsnOperandKind::SVE_ADDR_RZ_LSL1
        | InsnOperandKind::SVE_ADDR_RZ_LSL2
        | InsnOperandKind::SVE_ADDR_RZ_LSL3
        | InsnOperandKind::SVE_ADDR_RZ_XTW_14
        | InsnOperandKind::SVE_ADDR_RZ_XTW_22
        | InsnOperandKind::SVE_ADDR_RZ_XTW1_14
        | InsnOperandKind::SVE_ADDR_RZ_XTW1_22
        | InsnOperandKind::SVE_ADDR_RZ_XTW2_14
        | InsnOperandKind::SVE_ADDR_RZ_XTW2_22
        | InsnOperandKind::SVE_ADDR_RZ_XTW3_14
        | InsnOperandKind::SVE_ADDR_RZ_XTW3_22 => Operand::Other(kind.as_ref()),

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM7 => {
            let opc = bit_range(bits, 30, 2);
            if opc == 0b11 {
                return Operand::Unknown;
            }

            let fp = bit_set(bits, 26);
            let scale = if fp { 2 + opc } else { 2 + (opc >> 1) };

            let imm7 = bit_range(bits, 15, 7);
            let imm = (sign_extend(imm7, 6) << scale) as i64;

            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);

            if bit_set(bits, 23) {
                if bit_set(bits, 24) {
                    Operand::Unknown("[{reg_name}, #{imm}]!")
                } else {
                    Operand::Unknown("[{reg_name}], #{imm}")
                }
            } else if imm == 0 {
                Operand::Unknown("[{reg_name}]")
            } else {
                Operand::Unknown("[{reg_name}, #{imm}]")
            }
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM9
        | InsnOperandKind::ADDR_SIMM13
        | InsnOperandKind::ADDR_OFFSET => {
            let imm9 = bit_range(bits, 12, 9);
            let scale = if kind == InsnOperandKind::ADDR_SIMM13 {
                LOG2_TAG_GRANULE
            } else {
                0
            };
            let imm = (sign_extend(imm9, 8) << scale) as i64;

            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);

            let ldst_offset_only = definition.class == InsnClass::LDST_UNPRIV
                || definition.class == InsnClass::LDST_UNSCALED;
            if ldst_offset_only {
                Operand::Unknown("[{reg_name}");
                if imm != 0 {
                    Operand::Unknown(", #{imm}");
                }
                Operand::Unknown("]")
            }

            let post_index = !bit_set(bits, 11);
            if !post_index {
                Operand::Unknown("[{reg_name}, #{imm}]!");
            } else {
                Operand::Unknown("[{reg_name}], #{imm}");
            }
            *stop = true;
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_UIMM12 => {
            let fp = bit_set(bits, 26);
            let size = bit_range(bits, 30, 2);
            let scale = if fp {
                (bit_range(bits, 23, 1) << 2) | size
            } else {
                size
            };
            let uimm12 = bit_range(bits, 10, 12) << scale;
            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);
            Operand::Unknown("[{reg_name}");
            if uimm12 != 0 {
                Operand::Unknown(", #{uimm12:#x}");
            }
            Operand::Unknown("]");
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM10 => {
            let reg_n_no = bit_range(bits, 5, 5);
            let reg_n_name = get_int_reg_name(true, reg_n_no as u8, false);

            let scale = 3;
            let pre_index = bit_set(bits, 11);
            let s = bit_range(bits, 22, 1);
            let imm10 = bit_range(bits, 12, 9) | (s << 9);
            let simm = (sign_extend(imm10, 9) << scale) as i64;

            Operand::Unknown("[{reg_n_name}");
            if pre_index {
                if simm != 0 {
                    Operand::Unknown(", #{simm}");
                }
                Operand::Unknown("]!");
            } else {
                if simm != 0 {
                    Operand::Unknown(", #{simm}");
                }
                Operand::Unknown("]");
            }
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM11 => {
            let reg_n_no = bit_range(bits, 5, 5);
            let reg_n_name = get_int_reg_name(true, reg_n_no as u8, false);
            let simm = (sign_extend(bit_range(bits, 15, 7), 6) << LOG2_TAG_GRANULE) as i64;
            match bit_range(bits, 23, 2) {
                0b01 => {
                    Operand::Unknown("[{reg_n_name}], #{simm}");
                }
                0b11 => {
                    Operand::Unknown("[{reg_n_name}, #{simm}]!");
                }
                0b10 => {
                    Operand::Unknown("[{reg_n_name}");
                    if simm != 0 {
                        Operand::Unknown(", #{simm}");
                    }
                    Operand::Unknown("]");
                }
                _ => {
                    return Operand::Unknown;
                }
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::RCPC3_ADDR_OFFSET
        | InsnOperandKind::RCPC3_ADDR_OPT_POSTIND
        | InsnOperandKind::RCPC3_ADDR_OPT_PREIND_WB
        | InsnOperandKind::RCPC3_ADDR_POSTIND
        | InsnOperandKind::RCPC3_ADDR_PREIND_WB
        | InsnOperandKind::SME_ADDR_RI_U4xVL
        | InsnOperandKind::SVE_ADDR_RI_S4x16
        | InsnOperandKind::SVE_ADDR_RI_S4x32
        | InsnOperandKind::SVE_ADDR_RI_S4xVL
        | InsnOperandKind::SVE_ADDR_RI_S4x2xVL
        | InsnOperandKind::SVE_ADDR_RI_S4x3xVL
        | InsnOperandKind::SVE_ADDR_RI_S4x4xVL
        | InsnOperandKind::SVE_ADDR_RI_S6xVL
        | InsnOperandKind::SVE_ADDR_RI_S9xVL
        | InsnOperandKind::SVE_ADDR_RI_U6
        | InsnOperandKind::SVE_ADDR_RI_U6x2
        | InsnOperandKind::SVE_ADDR_RI_U6x4
        | InsnOperandKind::SVE_ADDR_RI_U6x8 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZI_U5
        | InsnOperandKind::SVE_ADDR_ZI_U5x2
        | InsnOperandKind::SVE_ADDR_ZI_U5x4
        | InsnOperandKind::SVE_ADDR_ZI_U5x8 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZZ_LSL
        | InsnOperandKind::SVE_ADDR_ZZ_SXTW
        | InsnOperandKind::SVE_ADDR_ZZ_UXTW => Operand::Other(kind.as_ref()),

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::SYSREG | InsnOperandKind::SYSREG128 => {
            let op0 = bit_range(bits, 19, 2) as u8;
            let op1 = bit_range(bits, 16, 3) as u8;
            let crn = bit_range(bits, 12, 4) as u8;
            let crm = bit_range(bits, 8, 4) as u8;
            let op2 = bit_range(bits, 5, 3) as u8;
            if let Some(sys_reg) = get_sys_reg_name(sys_reg_number(op0, op1, crn, crm, op2)) {
                let sys_reg = sys_reg.1;
                Operand::Unknown("{sys_reg}");
            } else {
                Operand::Unknown("s{op0}_{op1}_c{crn}_c{crm}_{op2}");
            }
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::PSTATEFIELD => {
            let op2 = bit_range(bits, 5, 3);
            let crm = bit_range(bits, 8, 4);
            let op1 = bit_range(bits, 16, 3);
            let field = match op1 {
                0b000 => match op2 {
                    0b011 => "uao",
                    0b100 => "pan",
                    0b101 => "spsel",
                    _ => return Operand::Unknown("s0_{op1}_c4_{crm}_{op2}"),
                },
                0b001 => match op2 {
                    0b000 if crm & 0b1110 == 0 => "allint",
                    _ => return Operand::Unknown("s0_{op1}_c4_c{crm}_{op2}"),
                },
                0b011 => match op2 {
                    0b011 if crm & 0b1110 == 0b0010 => "svcrsm",
                    0b100 if crm & 0b1110 == 0b0100 => "svcrza",
                    0b101 if crm & 0b1110 == 0b0110 => "svcrsmza",
                    0b001 => "ssbs",
                    0b010 => "dit",
                    0b100 => "tco",
                    0b110 => "daifset",
                    0b111 => "daifclr",
                    _ => return Operand::Unknown("s0_{op1}_c4_{crm}_{op2}"),
                },
                _ => return Operand::Unknown("s0_{op1}_c4_{crm}_{op2}"),
            };
            Operand::Unknown("{field}")
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::SYSREG_AT
        | InsnOperandKind::SYSREG_DC
        | InsnOperandKind::SYSREG_IC
        | InsnOperandKind::SYSREG_TLBI
        | InsnOperandKind::SYSREG_TLBIP
        | InsnOperandKind::SYSREG_SR => Operand::Other(kind.as_ref()),

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER => {
            let barrier = bit_range(bits, 8, 4);
            let barrier = match barrier {
                0b0001 => "oshld",
                0b0010 => "oshst",
                0b0011 => "osh",
                0b0101 => "nshld",
                0b0110 => "nshst",
                0b0111 => "nsh",
                0b1001 => "ishld",
                0b1010 => "ishst",
                0b1011 => "ish",
                0b1101 => "ld",
                0b1110 => "st",
                0b1111 => "sy",
                _ => return Operand::Unknown("#{:#x}", barrier),
            };
            Operand::Unknown("{barrier}")
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_DSB_NXS => {
            let barrier = bit_range(bits, 8, 4);
            let barrier = match barrier {
                0b0010 => "oshnxs",
                0b0110 => "nshnxs",
                0b1010 => "ishnxs",
                0b1110 => "synxs",
                _ => return Operand::Unknown("#{:#x}", barrier),
            };
            Operand::Unknown("{barrier}")
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::PRFOP => {
            let typ = match bit_range(bits, 3, 2) {
                0b00 => Some("pld"),
                0b01 => Some("pli"),
                0b10 => Some("pst"),
                _ => None,
            };
            let target = match bit_range(bits, 1, 2) {
                0b00 => Some("l1"),
                0b01 => Some("l2"),
                0b10 => Some("l3"),
                _ => None,
            };

            if typ.is_some() && target.is_some() {
                let policy = if !bit_set(bits, 0) { "keep" } else { "strm" };
                let typ = typ.unwrap_or("<undefined>");
                let target = target.unwrap_or("<undefined>");
                Operand::Unknown("{typ}{target}{policy}")
            } else {
                Operand::Unknown("#{:#x}", bit_range(bits, 0, 5))
            }
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::RPRFMOP => {
            let b12 = bit_range(bits, 12, 1) << 3;
            let b13 = bit_range(bits, 13, 1) << 4;
            let b15 = bit_range(bits, 15, 1) << 5;
            let rprfmop = bit_range(bits, 0, 3) | b12 | b13 | b15;
            let op = match rprfmop {
                0b00 => Some("pldkeep"),
                0b01 => Some("pstkeep"),
                0b100 => Some("pldstrm"),
                0b101 => Some("pststrm"),
                _ => None,
            };

            if let Some(op) = op {
                Operand::Unknown("{op}")
            } else {
                Operand::Unknown("#{rprfmop:#x}")
            }
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_PSB => Operand::Other(kind.as_ref()),

        InsnOperandKind::X16 => Operand::Unknown("x16"),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0 => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0_INDEX => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0_LIST => Operand::Other(kind.as_ref()),

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_GCSB => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::BTI_TARGET => Operand::Other(kind.as_ref()),

        #[cfg(feature = "full")]
        InsnOperandKind::MOPS_ADDR_Rd
        | InsnOperandKind::MOPS_ADDR_Rs
        | InsnOperandKind::MOPS_WB_Rn => {
            *stop = true;

            let rd = bit_range(bits, 0, 5) as u8;
            let rn = bit_range(bits, 5, 5) as u8;
            let rs = bit_range(bits, 16, 5) as u8;
            let op1 = bit_range(bits, 22, 2);

            if rd == rn || rd == rs || rs == rn {
                return Operand::Unknown;
            }
            if rd == 31 || rn == 31 || (rs == 31 && op1 != 0b11) {
                return Operand::Unknown;
            }

            let rd = get_int_reg_name(true, rd, true);
            let rn = get_int_reg_name(true, rn, true);
            let rs = get_int_reg_name(true, rs, true);

            if op1 != 0b11 {
                // Memory copy forward only (cpyfm, ...)
                Operand::Unknown("[{rd}]!, [{rs}]!, {rn}!")
            } else {
                // Set memory (setp, setm, sete, ...)
                Operand::Unknown("[{rd}]!, {rn}!, {rs}")
            }
        }

        #[cfg(not(feature = "full"))]
        _ => return Operand::Unknown,
    };

    Operand::Unknown
}

pub struct InsnOperandIter<'a, O>
where
    O: InsnOpcode,
{
    pc: u64,
    opcode: &'a O,
    insn_defn: &'static Insn,
    bits: u32,
    stop: bool,
    pos_operand: core::iter::Enumerate<core::slice::Iter<'a, defn::InsnOperand>>,
}

impl<'a, O> InsnOperandIter<'a, O>
where
    O: InsnOpcode,
{
    fn new(pc: u64, opcode: &'a O) -> InsnOperandIter<'a, O> {
        let insn_defn = opcode.definition();
        let bits = opcode.bits();
        let pos_operand = insn_defn.operands.iter().enumerate();
        Self {
            pc,
            opcode,
            insn_defn,
            bits,
            stop: false,
            pos_operand,
        }
    }
}

impl<'a, O> Iterator for InsnOperandIter<'a, O>
where
    O: InsnOpcode,
{
    type Item = Operand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let (pos, operand_defn) = self.pos_operand.next()?;

        // Process hints
        #[cfg(any(feature = "full", feature = "system"))]
        if self.insn_defn.class == disarm64_defn::InsnClass::IC_SYSTEM {
            if let Some(op) = self.insn_defn.operands.first() {
                if op.kind == InsnOperandKind::UIMM7 {
                    return Some(Operand::SystemHint(operand_hint(self.bits)));
                }
            }
        }

        #[cfg(feature = "full")]
        // TODO: update json to eliminate `pos == 0`
        if pos == 0 && self.insn_defn.flags.contains(InsnFlags::IS_COND) {
            // Conditional branch or the consistent conditional branch.
            return Some(Operand::Condition(operand_condition(self.bits)));
        }

        Some(operand_get_by_class(
            pos,
            self.pc,
            self.bits,
            &operand_defn,
            &self.insn_defn,
            &mut self.stop,
        ))
    }
}

#[derive(Copy, Clone)]
pub struct InsnOperands<'a, O>(u64, &'a O)
where
    O: InsnOpcode;

impl<'a, O> IntoIterator for InsnOperands<'a, O>
where
    O: InsnOpcode,
{
    type Item = Operand;
    type IntoIter = InsnOperandIter<'a, O>;

    fn into_iter(self) -> Self::IntoIter {
        InsnOperandIter::new(self.0, self.1)
    }
}
