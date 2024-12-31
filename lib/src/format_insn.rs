//! Formatting instructions to human-readable strings
//!
//! Currently, the decoded instruction is converted straight to a string.
//! Once the full known set of instructions is implemented, this will be
//! supplemented with a more structured approach while retaining the
//! current string formatting instructions.

#![cfg_attr(
    not(feature = "full"),
    allow(unused_variables, dead_code, unused_imports)
)]

use crate::registers::get_fp_reg_name;
use crate::registers::get_int_reg_name;
use crate::registers::get_simd_reg_name;
use crate::registers::get_sys_reg_name;
use crate::registers::sys_reg_number;
use crate::registers::FpRegSize;
use crate::registers::SimdRegArrangement;
use bitfield_struct::bitfield;
use core::fmt::Write;
use defn::InsnOpcode;
use disarm64_defn::defn;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;

const LOG2_TAG_GRANULE: u32 = 4;

fn cond_name(cond: u32) -> &'static str {
    const COND: [&str; 16] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al",
        "nv",
    ];
    COND[(cond & 0b1111) as usize]
}

fn bit_set(bits: u32, bit: u32) -> bool {
    bits & (1 << bit) != 0
}

fn bit_range(bits: u32, start: u32, width: u32) -> u32 {
    (bits >> start) & ((1 << width) - 1)
}

fn sign_extend(v: u32, n: u8) -> u64 {
    debug_assert!(n < 32);

    let v = v as u64;
    let mask = 1u64 << n;

    // Sign-extend by utilizing the fact that shifting into the sign bit replicates the bit.
    // Also can do arithmetic shift right by 64 - n bits and then shift left by 64 - n bits.
    (v ^ mask).wrapping_sub(mask)
}

/// Decode a logical immediate value, N:immr:imms.
fn decode_limm(byte_count: u32, n: u32, mut immr: u32, mut imms: u32) -> Option<u64> {
    let mut imm;
    let mask;
    let bit_count: u32;

    if n != 0 {
        bit_count = 64;
        mask = !0;
    } else {
        bit_count = match imms {
            0x00..=0x1f => 32,
            0x20..=0x2f => {
                imms &= 0xf;
                16
            }
            0x30..=0x37 => {
                imms &= 0x7;
                8
            }
            0x38..=0x3b => {
                imms &= 0x3;
                4
            }
            0x3c..=0x3d => {
                imms &= 0x1;
                2
            }
            _ => return None,
        };
        mask = (1u64 << bit_count) - 1;
        immr &= bit_count - 1;
    }

    if bit_count > byte_count * 8 {
        return None;
    }

    if imms == bit_count - 1 {
        return None;
    }

    imm = (1u64 << (imms + 1)) - 1;
    if immr != 0 {
        imm = ((imm << (bit_count - immr)) & mask) | (imm >> immr);
    }

    let replicate: &[u64] = match bit_count {
        2 => &[2, 4, 8, 16, 32],
        4 => &[4, 8, 16, 32],
        8 => &[8, 16, 32],
        16 => &[16, 32],
        32 => &[32],
        64 => &[],
        _ => return None,
    };
    for &r in replicate {
        imm |= imm << r;
    }

    let limm = !0 << (byte_count * 4) << (byte_count * 4);
    let limm = imm & !limm;
    Some(limm)
}

/// Format a floating-point register to a string
fn format_fp_reg(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
) -> core::fmt::Result {
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
        InsnClass::ASIMDALL => {
            let size = bit_range(bits, 22, 2);
            let cross_lane = bit_set(bits, 15);
            if cross_lane {
                match size {
                    0b00 => get_fp_reg_name(FpRegSize::B8, reg_no as usize),
                    0b01 => get_fp_reg_name(FpRegSize::H16, reg_no as usize),
                    0b10 => get_fp_reg_name(FpRegSize::S32, reg_no as usize),
                    0b11 => return write!(f, "<undefined>"),
                    _ => unreachable!(),
                }
            } else {
                match size {
                    0b00 => get_fp_reg_name(FpRegSize::H16, reg_no as usize),
                    0b01 => get_fp_reg_name(FpRegSize::S32, reg_no as usize),
                    0b10 => get_fp_reg_name(FpRegSize::D64, reg_no as usize),
                    0b11 => return write!(f, "<undefined>"),
                    _ => unreachable!(),
                }
            }
        }

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

/// Follows `bits(N) VFPExpandImm(bits(8) imm8, integer N)` in the A64 reference.
fn fp_expand_imm(size: i32, imm8: u32) -> Option<f64> {
    let imm8_7 = (imm8 >> 7) & 0x01; // imm8<7>
    let imm8_6_0 = imm8 & 0x7f; // imm8<6:0>
    let imm8_6 = imm8_6_0 >> 6; // imm8<6>
    let imm8_6_repl4 = (imm8_6 << 3) | (imm8_6 << 2) | (imm8_6 << 1) | imm8_6; // Replicate(imm8<6>,4)

    match size {
        8 => {
            // Double-precision
            let imm: u64 = ((imm8_7 as u64) << (63-32))    // imm8<7>
                | (((imm8_6 ^ 1) as u64) << (62-32)) // NOT(imm8<6>)
                | ((imm8_6_repl4 as u64) << (58-32))
                | ((imm8_6 as u64) << (57-32))
                | ((imm8_6 as u64) << (56-32))
                | ((imm8_6 as u64) << (55-32))      // Replicate(imm8<6>,7)
                | ((imm8_6_0 as u64) << (48-32)); // imm8<6>:imm8<5:0>
            Some(f64::from_bits(imm << 32))
        }
        4 | 2 => {
            // Single precision | Half-precision
            let imm = ((imm8_7 as u64) << 31)    // imm8<7>
                | (((imm8_6 ^ 1) as u64) << 30) // NOT(imm8<6>)
                | ((imm8_6_repl4 as u64) << 26) // Replicate(imm8<6>,4)
                | ((imm8_6_0 as u64) << 19); // imm8<6>:imm8<5:0>
            Some(f32::from_bits(imm as u32) as f64)
        }
        _ => None,
    }
}

/// Format an integer register (32-bit or 64-bit) to a string
fn format_int_operand_reg_pair(
    pair: bool,
    f: &mut impl Write,
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

/// Format an integer register (32-bit or 64-bit) to a string
fn format_int_operand_reg(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    with_zr: bool,
) -> core::fmt::Result {
    format_int_operand_reg_pair(false, f, bits, operand, definition, with_zr)
}

/// Format a register with extended shift operand to a string.
/// This is a lot of logic and dependencies on the other registers
/// for 4 instructions and 2 aliases.
fn format_operand_reg_ext(f: &mut impl Write, bits: u32) -> core::fmt::Result {
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

/// Format a register with shift operand to a string.
/// An example: https://developer.arm.com/documentation/ddi0602/2023-12/Base-Instructions/ADD--shifted-register---Add--shifted-register--
fn format_operand_reg_shift(f: &mut impl Write, bits: u32) -> core::fmt::Result {
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

/// Format a SIMD register to a string
fn format_simd_reg(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
) -> core::fmt::Result {
    let kind = operand.kind;

    let reg_no = if let Some(bit_filed) = operand.bit_fields.first() {
        bit_range(bits, bit_filed.lsb.into(), bit_filed.width.into())
    } else {
        return write!(f, ":{kind:?}:");
    } as u8;

    let simd_reg_arrangement = match definition.class {
        InsnClass::ASIMDALL => {
            let size = bit_range(bits, 22, 2);
            let q = bit_set(bits, 30) as u32;
            match size << 1 | q {
                0b000 => SimdRegArrangement::Vector8B,
                0b001 => SimdRegArrangement::Vector16B,
                0b010 => SimdRegArrangement::Vector4H,
                0b011 => SimdRegArrangement::Vector8H,
                0b101 => SimdRegArrangement::Vector4S,
                _ => return write!(f, "<undefined>"),
            }
        }

        _ => {
            if let Some(qual) = operand.qualifiers.first() {
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
                        InsnOperandQualifier::V_1D | InsnOperandQualifier::V_2D => {
                            SimdRegArrangement::Vector2D
                        }
                        _ => {
                            return write!(f, "<undefined>");
                        }
                    }
                }
            } else {
                return write!(f, "<undefined>");
            }
        }
    };

    let simd_reg_name = get_simd_reg_name(reg_no, simd_reg_arrangement);

    write!(f, "{simd_reg_name}")
}

/// Format an operand to a string
fn format_operand(
    pos: usize,
    pc: u64,
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    stop: &mut bool,
) -> core::fmt::Result {
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
            format_int_operand_reg(f, bits, operand, definition, with_zr)?
        }

        InsnOperandKind::PAIRREG | InsnOperandKind::PAIRREG_OR_XZR => {
            if pos == 0 {
                return write!(f, "<undefined>");
            }

            let prev_operand = &definition.operands[pos - 1];
            let pair = true;
            format_int_operand_reg_pair(pair, f, bits, prev_operand, definition, true)?;
        }

        InsnOperandKind::Rd_SP
        | InsnOperandKind::Rn_SP
        | InsnOperandKind::Rt_SP
        | InsnOperandKind::SVE_Rn_SP
        | InsnOperandKind::Rm_SP => {
            let with_zr = false;
            format_int_operand_reg(f, bits, operand, definition, with_zr)?
        }

        InsnOperandKind::Rm_EXT => format_operand_reg_ext(f, bits)?,

        InsnOperandKind::Rm_SFT => format_operand_reg_shift(f, bits)?,

        InsnOperandKind::Fd
        | InsnOperandKind::Fn
        | InsnOperandKind::Fm
        | InsnOperandKind::Fa
        | InsnOperandKind::Ft
        | InsnOperandKind::Ft2
        | InsnOperandKind::Sd
        | InsnOperandKind::Sn
        | InsnOperandKind::Sm => format_fp_reg(f, bits, operand, definition)?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_VZn => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Vd
        | InsnOperandKind::SVE_Vm
        | InsnOperandKind::SVE_Vn
        | InsnOperandKind::Va
        | InsnOperandKind::Vd
        | InsnOperandKind::Vn
        | InsnOperandKind::Vm => format_simd_reg(f, bits, operand, definition)?,

        #[cfg(feature = "full")]
        InsnOperandKind::Ed | InsnOperandKind::En | InsnOperandKind::Em | InsnOperandKind::Em16 => {
            write!(f, ":{kind:?}:")?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::VdD1 | InsnOperandKind::VnD1 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::LVn
        | InsnOperandKind::LVt
        | InsnOperandKind::LVt_AL
        | InsnOperandKind::LEt => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Pd
        | InsnOperandKind::SVE_Pg3
        | InsnOperandKind::SVE_Pg4_5
        | InsnOperandKind::SVE_Pg4_10
        | InsnOperandKind::SVE_Pg4_16
        | InsnOperandKind::SVE_Pm
        | InsnOperandKind::SVE_Pn
        | InsnOperandKind::SVE_Pt
        | InsnOperandKind::SME_Pm => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PNd
        | InsnOperandKind::SVE_PNg4_10
        | InsnOperandKind::SVE_PNn
        | InsnOperandKind::SVE_PNt
        | InsnOperandKind::SME_PNd3
        | InsnOperandKind::SME_PNg3
        | InsnOperandKind::SME_PNn => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_Pdx2 | InsnOperandKind::SME_PdxN => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_PNn3_INDEX1 | InsnOperandKind::SME_PNn3_INDEX2 => {
            write!(f, ":{kind:?}:")?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_Za_5
        | InsnOperandKind::SVE_Za_16
        | InsnOperandKind::SVE_Zd
        | InsnOperandKind::SVE_Zm_5
        | InsnOperandKind::SVE_Zm_16
        | InsnOperandKind::SVE_Zn
        | InsnOperandKind::SVE_Zt
        | InsnOperandKind::SME_Zm => write!(f, ":{kind:?}:")?,

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
        | InsnOperandKind::SME_Zt4 => write!(f, ":{kind:?}:")?,

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
        | InsnOperandKind::SVE_Zm_imm4 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZAda_2b | InsnOperandKind::SME_ZAda_3b => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_HV_idx_src
        | InsnOperandKind::SME_ZA_HV_idx_srcxN
        | InsnOperandKind::SME_ZA_HV_idx_dest
        | InsnOperandKind::SME_ZA_HV_idx_destxN
        | InsnOperandKind::SME_ZA_HV_idx_ldstr => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_list_of_64bit_tiles => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_array_off1x4
        | InsnOperandKind::SME_ZA_array_off2x2
        | InsnOperandKind::SME_ZA_array_off2x4
        | InsnOperandKind::SME_ZA_array_off3_0
        | InsnOperandKind::SME_ZA_array_off3_5
        | InsnOperandKind::SME_ZA_array_off3x2
        | InsnOperandKind::SME_ZA_array_off4 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZA_array_vrsb_1
        | InsnOperandKind::SME_ZA_array_vrsh_1
        | InsnOperandKind::SME_ZA_array_vrss_1
        | InsnOperandKind::SME_ZA_array_vrsd_1
        | InsnOperandKind::SME_ZA_array_vrsb_2
        | InsnOperandKind::SME_ZA_array_vrsh_2
        | InsnOperandKind::SME_ZA_array_vrss_2
        | InsnOperandKind::SME_ZA_array_vrsd_2 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_SM_ZA => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_PnT_Wm_imm => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_VLxN_10 | InsnOperandKind::SME_VLxN_13 => write!(f, ":{kind:?}:")?,

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::CRn => write!(f, "c{}", bit_range(bits, 12, 4))?,
        InsnOperandKind::CRm => write!(f, "c{}", bit_range(bits, 8, 4))?,

        #[cfg(feature = "full")]
        InsnOperandKind::IMMR => {
            let immr = bit_range(bits, 16, 6);
            let n = bit_set(bits, 22);
            let sf = bit_set(bits, 31);

            if (sf && !n) || (!sf && (n || bit_set(immr, 5))) {
                return write!(f, "<undefined>");
            }
            write!(f, "#{}", immr)?;
        }
        #[cfg(feature = "full")]
        InsnOperandKind::IMMS => {
            let imms = bit_range(bits, 10, 6);
            let n = bit_set(bits, 22);
            let sf = bit_set(bits, 31);

            if (sf && !n) || (!sf && (n || bit_set(imms, 5))) {
                return write!(f, "<undefined>");
            }
            write!(f, "#{}", imms)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::BIT_NUM => {
            let b5 = bit_range(bits, 31, 1);
            let b40 = bit_range(bits, 19, 5);
            let bit_num = (b5 << 5) | b40;
            write!(f, "#{}", bit_num)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::FBITS => {
            let ftype = bit_range(bits, 22, 2);
            if ftype == 0b10 {
                return write!(f, "<undefined>");
            }
            let sf = bit_set(bits, 31);
            let scale = 64 - bit_range(bits, 10, 6);
            if !sf && scale > 32 {
                return write!(f, "<undefined>");
            }
            write!(f, "#{scale}")?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IMM0 => write!(f, "#0")?,
        #[cfg(feature = "full")]
        InsnOperandKind::TME_UIMM16 => write!(f, "#{}", bit_range(bits, 5, 16))?,

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
        | InsnOperandKind::SVE_IMM_ROT3 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_SIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as i8)?,
        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_UIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as u8)?,
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM4_ADDG => write!(f, "#{}", bit_range(bits, 10, 4))?,
        // The 10 bit size comes from 6 bit in the instruction and the shift of 4.
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM10 => write!(f, "#{}", bit_range(bits, 16, 6) << LOG2_TAG_GRANULE)?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_I1_HALF_ONE
        | InsnOperandKind::SVE_I1_HALF_TWO
        | InsnOperandKind::SVE_I1_ZERO_ONE => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PATTERN => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PATTERN_SCALED => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_PRFOP => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::IMM_MOV => write!(f, ":{kind:?}:")?,
        #[cfg(feature = "full")]
        InsnOperandKind::FPIMM0 => write!(f, "#{:.1}", 0.0)?,

        #[cfg(feature = "full")]
        InsnOperandKind::AIMM => {
            let shift = bit_set(bits, 22);
            let imm12 = bit_range(bits, 10, 12);
            write!(f, "#{imm12:#x}")?;

            if shift {
                return write!(f, ", lsl #12");
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::HALF => {
            let hw = bit_range(bits, 21, 2);
            if !bit_set(bits, 31) && bit_set(hw, 1) {
                return write!(f, "<undefined>");
            }

            let imm16 = bit_range(bits, 5, 16);
            let shift = hw << 4;

            write!(f, "#{imm16:#x}, lsl #{shift:#x}")?
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::LIMM => {
            let imms = bit_range(bits, 10, 6);
            let immr = bit_range(bits, 16, 6);
            let n = bit_range(bits, 22, 1);
            let is_64bit = bit_set(bits, 31);
            let byte_count = if is_64bit { 8 } else { 4 };
            if let Some(imm) = decode_limm(byte_count, n, immr, imms) {
                write!(f, "#{imm:#x}")?;
            } else {
                write!(f, "<undefined>")?;
            }
        }
        #[cfg(feature = "full")]
        InsnOperandKind::SVE_INV_LIMM
        | InsnOperandKind::SVE_LIMM
        | InsnOperandKind::SVE_LIMM_MOV => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_IMM => {
            let imm8 = (bit_range(bits, 16, 3) << 5) | bit_range(bits, 5, 5);
            let mut imm = 0u64;
            for i in 0..8 {
                let byte = if imm8 & (1 << i) != 0 { 0xff } else { 0x00 };
                imm |= byte << (i * 8);
            }

            write!(f, "#{imm:#x}")?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_IMM_SFT => {
            let imm8 = (bit_range(bits, 16, 3) << 5) | bit_range(bits, 5, 5);
            let cmode = bit_range(bits, 12, 4);

            *stop = true;
            // 32-bit shifting ones (op == 0 && cmode == 110x),
            // e.g. MOVI <Vd>.<T>, #<imm8>, MSL #<amount>
            if cmode >> 1 == 0b110 {
                let msl = if bit_set(cmode, 0) { 16 } else { 8 };
                return write!(f, "#{imm8:#x}, MSL #{msl}");
            }
            // 32-bit shifted immediate (op == 0 && cmode == 0xx0),
            // e.g. MOVI <Vd>.<T>, #<imm8>{, LSL #<amount>}
            if cmode & 0b1001 == 0 || cmode & 0b1001 == 0b0001 {
                let lsl = bit_range(cmode, 1, 2) * 8;
                if lsl != 0 {
                    return write!(f, "#{imm8:#x}, LSL #{lsl}");
                } else {
                    return write!(f, "#{imm8:#x}");
                }
            }
            // 16-bit shifted immediate (op == 0 && cmode == 10x0),
            // e.g. MOVI <Vd>.<T>, #<imm8>{, LSL #<amount>}
            if cmode & 0b1101 == 0b1000 || cmode & 0b1101 == 0b1001 {
                let lsl = if bit_set(cmode, 1) { 8 } else { 0 };
                if lsl != 0 {
                    return write!(f, "#{imm8:#x}, LSL #{lsl}");
                } else {
                    return write!(f, "#{imm8:#x}");
                }
            }
            // 8-bit (op == 0 && cmode == 111x),
            // e.g. MOVI <Vd>.<T>, #<imm8>{, LSL #0}
            if cmode >> 1 == 0b111 {
                return write!(f, "#{imm8:#x}");
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_FPIMM => {
            let imm8 = (bit_range(bits, 16, 3) << 5) | bit_range(bits, 5, 5);
            if let Some(imm) = fp_expand_imm(8, imm8) {
                write!(f, "#{}", imm)?
            } else {
                write!(f, "<undefined>")?
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::FPIMM => {
            let fp_type = bit_range(bits, 22, 2);
            let size = match fp_type {
                0b00 => 4,
                0b01 => 8,
                0b10 => return write!(f, "<undefined>"),
                0b11 => 2,
                _ => unreachable!(),
            };
            let imm8 = bit_range(bits, 13, 8);
            if let Some(imm) = fp_expand_imm(size, imm8) {
                write!(f, "#{}", imm)?
            } else {
                write!(f, "<undefined>")?
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_AIMM | InsnOperandKind::SVE_ASIMM | InsnOperandKind::SVE_FPIMM8 => {
            write!(f, ":{kind:?}:")?
        }

        #[cfg(any(feature = "full", feature = "exception"))]
        InsnOperandKind::EXCEPTION => {
            let imm16 = bit_range(bits, 5, 16);
            write!(f, "#{imm16:#x}")?
        }
        #[cfg(any(feature = "full", feature = "exception"))]
        InsnOperandKind::UNDEFINED => {
            let imm16 = bit_range(bits, 0, 16);
            write!(f, "#{imm16:#x}")?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::CCMP_IMM => {
            let imm5 = bit_range(bits, 16, 5);
            write!(f, "#{imm5:#x}")?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::NZCV => {
            let imm4 = bit_range(bits, 0, 4);
            write!(f, "#{imm4:#x}")?
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::IMM_2 => {
            let imm = bit_range(bits, 15, 6);
            write!(f, "#{}", imm)?;
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::MASK => {
            let mask = bit_range(bits, 0, 4);
            write!(f, "#{}", mask)?;
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM3_OP1 => {
            let imm = bit_range(bits, 16, 3);
            write!(f, "#{}", imm)?;
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM3_OP2 => {
            let imm = bit_range(bits, 5, 3);
            write!(f, "#{}", imm)?;
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM4 => {
            let imm4 = bit_range(bits, 8, 4);
            write!(f, "#{imm4:#x}")?
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_ISB => {
            let imm4 = bit_range(bits, 8, 4);
            if imm4 != 0xf {
                write!(f, "#{imm4:#x}")?
            }
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM7 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::COND | InsnOperandKind::COND1 => {
            let cond = bit_range(bits, 12, 4);
            write!(f, "{}", cond_name(cond))?
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL14 => {
            let offset = bit_range(bits, 5, 14);
            let offset = sign_extend(offset, 13) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL19 => {
            let offset = bit_range(bits, 5, 19);
            let offset = sign_extend(offset, 18) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL21 => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20);
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_ADRP => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20) << 12;
            let pc = pc & !((1 << 12) - 1);
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_PCREL26 => {
            let offset = bit_range(bits, 0, 26);
            let offset = sign_extend(offset, 25) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMPLE | InsnOperandKind::SIMD_ADDR_SIMPLE => {
            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);
            write!(f, "[{reg_name}]")?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SIMD_ADDR_POST => write!(f, ":{kind:?}:")?,

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
            write!(f, "[{rn_reg_name}, {rm_reg_name}")?;
            if option == 0b011 {
                if shift {
                    write!(f, ", lsl #{scale}")?;
                }
            } else if option != 0b011 {
                write!(f, ", {extend}")?;
                if shift {
                    write!(f, " #{scale}")?;
                }
            }
            write!(f, "]")?;

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
        | InsnOperandKind::SVE_ADDR_RX_LSL3 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZX => write!(f, ":{kind:?}:")?,

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
        | InsnOperandKind::SVE_ADDR_RZ_XTW3_22 => write!(f, ":{kind:?}:")?,

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM7 => {
            let opc = bit_range(bits, 30, 2);
            if opc == 0b11 {
                return write!(f, "<undefined>");
            }

            let fp = bit_set(bits, 26);
            let scale = if fp { 2 + opc } else { 2 + (opc >> 1) };

            let imm7 = bit_range(bits, 15, 7);
            let imm = (sign_extend(imm7, 6) << scale) as i64;

            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);

            if bit_set(bits, 23) {
                if bit_set(bits, 24) {
                    write!(f, "[{reg_name}, #{imm}]!")?;
                } else {
                    write!(f, "[{reg_name}], #{imm}")?;
                }
            } else if imm == 0 {
                write!(f, "[{reg_name}]")?;
            } else {
                write!(f, "[{reg_name}, #{imm}]")?;
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
                write!(f, "[{reg_name}")?;
                if imm != 0 {
                    write!(f, ", #{imm}")?;
                }
                return write!(f, "]");
            }

            let post_index = !bit_set(bits, 11);
            if !post_index {
                write!(f, "[{reg_name}, #{imm}]!")?;
            } else {
                write!(f, "[{reg_name}], #{imm}")?;
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
            write!(f, "[{reg_name}")?;
            if uimm12 != 0 {
                write!(f, ", #{uimm12:#x}")?;
            }
            write!(f, "]")?;
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

            write!(f, "[{reg_n_name}")?;
            if pre_index {
                if simm != 0 {
                    write!(f, ", #{simm}")?;
                }
                write!(f, "]!")?;
            } else {
                if simm != 0 {
                    write!(f, ", #{simm}")?;
                }
                write!(f, "]")?;
            }
        }

        #[cfg(any(feature = "full", feature = "load_store"))]
        InsnOperandKind::ADDR_SIMM11 => {
            let reg_n_no = bit_range(bits, 5, 5);
            let reg_n_name = get_int_reg_name(true, reg_n_no as u8, false);
            let simm = (sign_extend(bit_range(bits, 15, 7), 6) << LOG2_TAG_GRANULE) as i64;
            match bit_range(bits, 23, 2) {
                0b01 => {
                    write!(f, "[{reg_n_name}], #{simm}")?;
                }
                0b11 => {
                    write!(f, "[{reg_n_name}, #{simm}]!")?;
                }
                0b10 => {
                    write!(f, "[{reg_n_name}")?;
                    if simm != 0 {
                        write!(f, ", #{simm}")?;
                    }
                    write!(f, "]")?;
                }
                _ => {
                    write!(f, "<undefined>")?;
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
        | InsnOperandKind::SVE_ADDR_RI_U6x8 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZI_U5
        | InsnOperandKind::SVE_ADDR_ZI_U5x2
        | InsnOperandKind::SVE_ADDR_ZI_U5x4
        | InsnOperandKind::SVE_ADDR_ZI_U5x8 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SVE_ADDR_ZZ_LSL
        | InsnOperandKind::SVE_ADDR_ZZ_SXTW
        | InsnOperandKind::SVE_ADDR_ZZ_UXTW => write!(f, ":{kind:?}:")?,

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::SYSREG | InsnOperandKind::SYSREG128 => {
            let op0 = bit_range(bits, 19, 2) as u8;
            let op1 = bit_range(bits, 16, 3) as u8;
            let crn = bit_range(bits, 12, 4) as u8;
            let crm = bit_range(bits, 8, 4) as u8;
            let op2 = bit_range(bits, 5, 3) as u8;
            if let Some(sys_reg) = get_sys_reg_name(sys_reg_number(op0, op1, crn, crm, op2)) {
                let sys_reg = sys_reg.1;
                write!(f, "{sys_reg}")?;
            } else {
                write!(f, "s{op0}_{op1}_c{crn}_c{crm}_{op2}")?;
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
                    _ => return write!(f, "s0_{op1}_c4_{crm}_{op2}"),
                },
                0b001 => match op2 {
                    0b000 if crm & 0b1110 == 0 => "allint",
                    _ => return write!(f, "s0_{op1}_c4_c{crm}_{op2}"),
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
                    _ => return write!(f, "s0_{op1}_c4_{crm}_{op2}"),
                },
                _ => return write!(f, "s0_{op1}_c4_{crm}_{op2}"),
            };
            write!(f, "{field}")?
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::SYSREG_AT
        | InsnOperandKind::SYSREG_DC
        | InsnOperandKind::SYSREG_IC
        | InsnOperandKind::SYSREG_TLBI
        | InsnOperandKind::SYSREG_TLBIP
        | InsnOperandKind::SYSREG_SR => write!(f, ":{kind:?}:")?,

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
                _ => return write!(f, "#{:#x}", barrier),
            };
            write!(f, "{barrier}")?
        }
        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_DSB_NXS => {
            let barrier = bit_range(bits, 8, 4);
            let barrier = match barrier {
                0b0010 => "oshnxs",
                0b0110 => "nshnxs",
                0b1010 => "ishnxs",
                0b1110 => "synxs",
                _ => return write!(f, "#{:#x}", barrier),
            };
            write!(f, "{barrier}")?
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
                write!(f, "{typ}{target}{policy}")?
            } else {
                write!(f, "#{:#x}", bit_range(bits, 0, 5))?
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
                write!(f, "{op}")?
            } else {
                write!(f, "#{rprfmop:#x}")?
            }
        }

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_PSB => write!(f, ":{kind:?}:")?,

        InsnOperandKind::X16 => write!(f, "x16")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0 => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0_INDEX => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::SME_ZT0_LIST => write!(f, ":{kind:?}:")?,

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::BARRIER_GCSB => write!(f, ":{kind:?}:")?,

        #[cfg(feature = "full")]
        InsnOperandKind::BTI_TARGET => write!(f, ":{kind:?}:")?,

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
                return write!(f, "<undefined>");
            }
            if rd == 31 || rn == 31 || (rs == 31 && op1 != 0b11) {
                return write!(f, "<undefined>");
            }

            let rd = get_int_reg_name(true, rd, true);
            let rn = get_int_reg_name(true, rn, true);
            let rs = get_int_reg_name(true, rs, true);

            if op1 != 0b11 {
                // Memory copy forward only (cpyfm, ...)
                write!(f, "[{rd}]!, [{rs}]!, {rn}!")?
            } else {
                // Set memory (setp, setm, sete, ...)
                write!(f, "[{rd}]!, {rn}!, {rs}")?
            }
        }

        #[cfg(not(feature = "full"))]
        _ => write!(f, "<unknown>")?,
    };

    Ok(())
}

#[cfg(any(feature = "full", feature = "system"))]
fn format_hint(f: &mut impl Write, bits: u32) -> core::fmt::Result {
    let hint = bit_range(bits, 5, 7);

    #[allow(clippy::unusual_byte_groupings)]
    let hint = match hint {
        0b0000_000 => "nop",
        0b0000_001 => "yield",
        0b0000_010 => "wfe",
        0b0000_011 => "wfi",
        0b0000_100 => "sev",
        0b0000_101 => "sevl",
        0b0000_110 => "dgh",
        0b0000_111 => "xpaclri",
        0b0001_000 => "pacia1716",
        0b0001_010 => "pacib1716",
        0b0001_100 => "autia1716",
        0b0001_110 => "autib1716",
        0b0010_000 => "esb",
        0b0010_001 => "psb\t\tcsync",
        0b0010_010 => "tsb\t\tcsync",
        0b0010_100 => "csdb",
        0b0010_110 => "clrbhb",
        0b0010_011 => "gcsb\t\tdsync",
        0b0011_000 => "paciaz",
        0b0011_001 => "paciasp",
        0b0011_010 => "pacibz",
        0b0011_011 => "pacibsp",
        0b0011_100 => "autiaz",
        0b0011_101 => "autiasp",
        0b0011_110 => "autibz",
        0b0011_111 => "autibsp",
        0b0100_000 => "bti",
        0b0100_010 => "bti\t\tc",
        0b0100_100 => "bti\t\tj",
        0b0100_110 => "bti\t\tjc",
        _ => return write!(f, "hint\t\t#{:#x}", hint),
    };

    write!(f, "{}", hint)
}

/// Format an instruction to a string. It does not use the aliases yet
/// and always emits the primary mnemonic.
/// The program counter is useful for the PC-relative addressing to
/// emit the target address in the disassembly rather than the offset.
pub fn format_insn_pc<O: InsnOpcode>(pc: u64, f: &mut impl Write, opcode: &O) -> core::fmt::Result {
    let definition = opcode.definition();
    let bits = opcode.bits();

    // Process hints
    #[cfg(any(feature = "full", feature = "system"))]
    if definition.class == disarm64_defn::InsnClass::IC_SYSTEM {
        if let Some(op) = definition.operands.first() {
            if op.kind == InsnOperandKind::UIMM7 {
                return format_hint(f, bits);
            }
        }
    }

    write!(f, "{}", definition.mnemonic)?;

    #[cfg(feature = "full")]
    if definition.flags.contains(InsnFlags::IS_COND) {
        // Conditional branch or the consistent conditional branch.
        let cond = bit_range(bits, 0, 4);
        let cond = cond_name(cond);
        write!(f, "{} ", cond)?;
    }
    write!(f, "\t\t")?;

    let op_count = definition.operands.len();
    for (i, operand) in definition.operands.iter().enumerate() {
        let mut stop = false;
        format_operand(i, pc, f, bits, operand, definition, &mut stop)?;
        if !stop && i + 1 < op_count {
            write!(f, ", ")?;
        }
        if stop {
            break;
        }
    }

    Ok(())
}
