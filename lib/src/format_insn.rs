//! Formatting instructions to human-readable strings
//!
//! Currently, the decoded instruction is converted straight to a string.
//! Once the full known set of instructions is implemented, this will be
//! supplemented with a more structured approach while retaining the
//! current string formatting instructions.

use crate::decoder::Opcode;
use bitfield_struct::bitfield;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Write;
use defn::InsnOpcode;
use disarm64_defn::defn;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandKind;
use disarm64_defn::InsnOperandQualifier;

const LOG2_TAG_GRANULE: u32 = 4;

fn get_int_reg_name(is_64: bool, reg: u8, with_zr: bool) -> &'static str {
    const INT_REG: [[[&str; 32]; 2]; 2] = [
        [
            [
                "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "w10", "w11", "w12",
                "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22", "w23", "w24",
                "w25", "w26", "w27", "w28", "w29", "w30", "wsp",
            ],
            [
                "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12",
                "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
                "x25", "x26", "x27", "x28", "x29", "x30", "sp",
            ],
        ],
        [
            [
                "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "w10", "w11", "w12",
                "w13", "w14", "w15", "w16", "w17", "w18", "w19", "w20", "w21", "w22", "w23", "w24",
                "w25", "w26", "w27", "w28", "w29", "w30", "wzr",
            ],
            [
                "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12",
                "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
                "x25", "x26", "x27", "x28", "x29", "x30", "xzr",
            ],
        ],
    ];

    let is_sp = with_zr as usize;
    let is_64 = is_64 as usize;

    if reg >= 32 {
        return "<undefined>";
    }

    INT_REG[is_sp][is_64][reg as usize]
}

#[repr(usize)]
enum FpRegSize {
    B8,
    H16,
    S32,
    D64,
    Q128,
}

fn get_fp_reg_name(size: FpRegSize, reg_no: usize) -> &'static str {
    const FP_REG: [[&str; 32]; 5] = [
        [
            "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "b10", "b11", "b12", "b13",
            "b14", "b15", "b16", "b17", "b18", "b19", "b20", "b21", "b22", "b23", "b24", "b25",
            "b26", "b27", "b28", "b29", "b30", "b31",
        ],
        [
            "h0", "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "h10", "h11", "h12", "h13",
            "h14", "h15", "h16", "h17", "h18", "h19", "h20", "h21", "h22", "h23", "h24", "h25",
            "h26", "h27", "h28", "h29", "h30", "h31",
        ],
        [
            "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "s12", "s13",
            "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25",
            "s26", "s27", "s28", "s29", "s30", "s31",
        ],
        [
            "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "d10", "d11", "d12", "d13",
            "d14", "d15", "d16", "d17", "d18", "d19", "d20", "d21", "d22", "d23", "d24", "d25",
            "d26", "d27", "d28", "d29", "d30", "d31",
        ],
        [
            "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "q10", "q11", "q12", "q13",
            "q14", "q15", "q16", "q17", "q18", "q19", "q20", "q21", "q22", "q23", "q24", "q25",
            "q26", "q27", "q28", "q29", "q30", "q31",
        ],
    ];

    if reg_no >= 32 {
        return "<undefined>";
    }

    FP_REG[size as usize][reg_no]
}

fn _get_sve_reg_name(is_64: bool, reg: u8) -> &'static str {
    const SVE_REG: [[&str; 32]; 2] = [
        [
            "z0.s", "z1.s", "z2.s", "z3.s", "z4.s", "z5.s", "z6.s", "z7.s", "z8.s", "z9.s",
            "z10.s", "z11.s", "z12.s", "z13.s", "z14.s", "z15.s", "z16.s", "z17.s", "z18.s",
            "z19.s", "z20.s", "z21.s", "z22.s", "z23.s", "z24.s", "z25.s", "z26.s", "z27.s",
            "z28.s", "z29.s", "z30.s", "z31.s",
        ],
        [
            "z0.d", "z1.d", "z2.d", "z3.d", "z4.d", "z5.d", "z6.d", "z7.d", "z8.d", "z9.d",
            "z10.d", "z11.d", "z12.d", "z13.d", "z14.d", "z15.d", "z16.d", "z17.d", "z18.d",
            "z19.d", "z20.d", "z21.d", "z22.d", "z23.d", "z24.d", "z25.d", "z26.d", "z27.d",
            "z28.d", "z29.d", "z30.d", "z31.d",
        ],
    ];

    if reg >= 32 {
        return "<undefined>";
    }

    let is_64 = is_64 as usize;
    SVE_REG[is_64][reg as usize]
}

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
fn decode_limm(is_64bit: bool, n: u32, mut immr: u32, mut imms: u32) -> Option<u64> {
    let byte_count = if is_64bit { 8 } else { 4 };
    let mut imm;
    let mask;
    let simd_size: u32;

    if n != 0 {
        simd_size = 64;
        mask = 0xffffffffffffffffu64;
    } else {
        simd_size = match imms {
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
        mask = (1u64 << simd_size) - 1;
        immr &= simd_size - 1;
    }

    if simd_size > byte_count as u32 * 8 {
        return None;
    }

    if imms == simd_size - 1 {
        return None;
    }

    imm = (1u64 << (imms + 1)) - 1;
    if immr != 0 {
        imm = ((imm << (simd_size - immr)) & mask) | (imm >> immr);
    }

    imm = match simd_size {
        2 => (imm << 2) | imm,
        4 => {
            imm = (imm << 2) | imm;
            (imm << 4) | imm
        }
        8 => {
            imm = (imm << 4) | imm;
            (imm << 8) | imm
        }
        16 => {
            imm = (imm << 8) | imm;
            (imm << 16) | imm
        }
        32 => {
            imm = (imm << 16) | imm;
            (imm << 32) | imm
        }
        64 => imm,
        _ => return None,
    };

    Some(imm & !(!0 << (byte_count * 4) << (byte_count * 4)))
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

        _ => return write!(f, ":{kind:?}:"),
    };

    write!(f, "{fp_reg_name}")
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
        | InsnOperandKind::Ft2 => format_fp_reg(f, bits, operand, definition)?,

        InsnOperandKind::Sd
        | InsnOperandKind::Sn
        | InsnOperandKind::Sm
        | InsnOperandKind::SVE_VZn
        | InsnOperandKind::SVE_Vd
        | InsnOperandKind::SVE_Vm
        | InsnOperandKind::SVE_Vn => write!(f, ":{kind:?}:")?,

        InsnOperandKind::Va | InsnOperandKind::Vd | InsnOperandKind::Vn | InsnOperandKind::Vm => {
            write!(f, ":{kind:?}:")?
        }

        InsnOperandKind::Ed | InsnOperandKind::En | InsnOperandKind::Em | InsnOperandKind::Em16 => {
            write!(f, ":{kind:?}:")?
        }

        InsnOperandKind::VdD1 | InsnOperandKind::VnD1 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::LVn
        | InsnOperandKind::LVt
        | InsnOperandKind::LVt_AL
        | InsnOperandKind::LEt => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_Pd
        | InsnOperandKind::SVE_Pg3
        | InsnOperandKind::SVE_Pg4_5
        | InsnOperandKind::SVE_Pg4_10
        | InsnOperandKind::SVE_Pg4_16
        | InsnOperandKind::SVE_Pm
        | InsnOperandKind::SVE_Pn
        | InsnOperandKind::SVE_Pt
        | InsnOperandKind::SME_Pm => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_PNd
        | InsnOperandKind::SVE_PNg4_10
        | InsnOperandKind::SVE_PNn
        | InsnOperandKind::SVE_PNt
        | InsnOperandKind::SME_PNd3
        | InsnOperandKind::SME_PNg3
        | InsnOperandKind::SME_PNn => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_Pdx2 | InsnOperandKind::SME_PdxN => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_PNn3_INDEX1 | InsnOperandKind::SME_PNn3_INDEX2 => {
            write!(f, ":{kind:?}:")?
        }

        InsnOperandKind::SVE_Za_5
        | InsnOperandKind::SVE_Za_16
        | InsnOperandKind::SVE_Zd
        | InsnOperandKind::SVE_Zm_5
        | InsnOperandKind::SVE_Zm_16
        | InsnOperandKind::SVE_Zn
        | InsnOperandKind::SVE_Zt
        | InsnOperandKind::SME_Zm => write!(f, ":{kind:?}:")?,

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

        InsnOperandKind::SME_ZAda_2b | InsnOperandKind::SME_ZAda_3b => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_ZA_HV_idx_src
        | InsnOperandKind::SME_ZA_HV_idx_srcxN
        | InsnOperandKind::SME_ZA_HV_idx_dest
        | InsnOperandKind::SME_ZA_HV_idx_destxN
        | InsnOperandKind::SME_ZA_HV_idx_ldstr => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_list_of_64bit_tiles => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_ZA_array_off1x4
        | InsnOperandKind::SME_ZA_array_off2x2
        | InsnOperandKind::SME_ZA_array_off2x4
        | InsnOperandKind::SME_ZA_array_off3_0
        | InsnOperandKind::SME_ZA_array_off3_5
        | InsnOperandKind::SME_ZA_array_off3x2
        | InsnOperandKind::SME_ZA_array_off4 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_ZA_array_vrsb_1
        | InsnOperandKind::SME_ZA_array_vrsh_1
        | InsnOperandKind::SME_ZA_array_vrss_1
        | InsnOperandKind::SME_ZA_array_vrsd_1
        | InsnOperandKind::SME_ZA_array_vrsb_2
        | InsnOperandKind::SME_ZA_array_vrsh_2
        | InsnOperandKind::SME_ZA_array_vrss_2
        | InsnOperandKind::SME_ZA_array_vrsd_2 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_SM_ZA => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_PnT_Wm_imm => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_VLxN_10 | InsnOperandKind::SME_VLxN_13 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::CRn | InsnOperandKind::CRm => write!(f, ":{kind:?}:")?,

        InsnOperandKind::IDX
        | InsnOperandKind::MASK
        | InsnOperandKind::IMM
        | InsnOperandKind::IMM_2
        | InsnOperandKind::WIDTH
        | InsnOperandKind::UIMM3_OP1
        | InsnOperandKind::UIMM3_OP2
        | InsnOperandKind::BIT_NUM
        | InsnOperandKind::IMM_VLSL
        | InsnOperandKind::IMM_VLSR
        | InsnOperandKind::SHLL_IMM
        | InsnOperandKind::IMM0
        | InsnOperandKind::IMMR
        | InsnOperandKind::IMMS
        | InsnOperandKind::FBITS
        | InsnOperandKind::TME_UIMM16
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

        InsnOperandKind::CSSC_SIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as i8)?,
        InsnOperandKind::CSSC_UIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as u8)?,
        InsnOperandKind::UIMM4_ADDG => write!(f, "#{}", bit_range(bits, 10, 4))?,
        // The 10 bit size comes from 6 bit in the instruction and the shift of 4.
        InsnOperandKind::UIMM10 => write!(f, "#{}", bit_range(bits, 16, 6) << LOG2_TAG_GRANULE)?,

        InsnOperandKind::SVE_I1_HALF_ONE
        | InsnOperandKind::SVE_I1_HALF_TWO
        | InsnOperandKind::SVE_I1_ZERO_ONE => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_PATTERN => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_PATTERN_SCALED => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_PRFOP => write!(f, ":{kind:?}:")?,

        InsnOperandKind::IMM_MOV => write!(f, ":{kind:?}:")?,

        InsnOperandKind::FPIMM0 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::AIMM => {
            let shift = bit_set(bits, 22);
            let imm12 = bit_range(bits, 10, 12);
            write!(f, "#{imm12:#x}")?;

            if shift {
                return write!(f, ", lsl #12");
            }
        }

        InsnOperandKind::HALF => {
            let hw = bit_range(bits, 21, 2);
            if !bit_set(bits, 31) && bit_set(hw, 1) {
                return write!(f, "<undefined>");
            }

            let imm16 = bit_range(bits, 5, 16);
            let shift = hw << 4;

            write!(f, "#{imm16:#x}, lsl #{shift:#x}")?
        }

        InsnOperandKind::LIMM
        | InsnOperandKind::SVE_INV_LIMM
        | InsnOperandKind::SVE_LIMM
        | InsnOperandKind::SVE_LIMM_MOV => {
            let imms = bit_range(bits, 10, 6);
            let immr = bit_range(imms, 16, 6);
            let n = bit_range(imms, 22, 1);
            let is_64bit = bit_set(bits, 31);
            if let Some(imm) = decode_limm(is_64bit, n, immr, imms) {
                write!(f, "#{imm:#x}")?;
            } else {
                write!(f, "<undefined>")?;
            }
        }

        InsnOperandKind::SIMD_IMM | InsnOperandKind::SIMD_IMM_SFT => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_AIMM | InsnOperandKind::SVE_ASIMM => write!(f, ":{kind:?}:")?,

        InsnOperandKind::FPIMM | InsnOperandKind::SIMD_FPIMM | InsnOperandKind::SVE_FPIMM8 => {
            write!(f, ":{kind:?}:")?
        }

        InsnOperandKind::EXCEPTION => {
            let imm16 = bit_range(bits, 5, 16);
            write!(f, "#{imm16:#x}")?
        }
        InsnOperandKind::UNDEFINED => {
            let imm16 = bit_range(bits, 0, 16);
            write!(f, "#{imm16:#x}")?;
        }

        InsnOperandKind::CCMP_IMM
        | InsnOperandKind::NZCV
        | InsnOperandKind::UIMM4
        | InsnOperandKind::UIMM7 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::COND | InsnOperandKind::COND1 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::ADDR_PCREL14 => {
            let offset = bit_range(bits, 5, 14);
            let offset = sign_extend(offset, 13) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        InsnOperandKind::ADDR_PCREL19 => {
            let offset = bit_range(bits, 5, 19);
            let offset = sign_extend(offset, 18) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        InsnOperandKind::ADDR_PCREL21 => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20);
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        InsnOperandKind::ADDR_ADRP => {
            let offset = (bit_range(bits, 5, 19) << 2) | bit_range(bits, 29, 2);
            let offset = sign_extend(offset, 20) << 12;
            let pc = pc & !((1 << 12) - 1);
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }
        InsnOperandKind::ADDR_PCREL26 => {
            let offset = bit_range(bits, 0, 26);
            let offset = sign_extend(offset, 25) << 2;
            write!(f, "{:#x}", pc.wrapping_add(offset))?
        }

        InsnOperandKind::ADDR_SIMPLE | InsnOperandKind::SIMD_ADDR_SIMPLE => {
            let reg_no = bit_range(bits, 5, 5);
            let reg_name = get_int_reg_name(true, reg_no as u8, false);
            write!(f, "[{reg_name}]")?
        }
        InsnOperandKind::SIMD_ADDR_POST => write!(f, ":{kind:?}:")?,

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

        InsnOperandKind::SVE_ADDR_ZX => write!(f, ":{kind:?}:")?,

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

        InsnOperandKind::SVE_ADDR_ZI_U5
        | InsnOperandKind::SVE_ADDR_ZI_U5x2
        | InsnOperandKind::SVE_ADDR_ZI_U5x4
        | InsnOperandKind::SVE_ADDR_ZI_U5x8 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SVE_ADDR_ZZ_LSL
        | InsnOperandKind::SVE_ADDR_ZZ_SXTW
        | InsnOperandKind::SVE_ADDR_ZZ_UXTW => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SYSREG | InsnOperandKind::SYSREG128 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::PSTATEFIELD => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SYSREG_AT
        | InsnOperandKind::SYSREG_DC
        | InsnOperandKind::SYSREG_IC
        | InsnOperandKind::SYSREG_TLBI
        | InsnOperandKind::SYSREG_TLBIP
        | InsnOperandKind::SYSREG_SR => write!(f, ":{kind:?}:")?,

        InsnOperandKind::BARRIER | InsnOperandKind::BARRIER_DSB_NXS => write!(f, ":{kind:?}:")?,

        InsnOperandKind::BARRIER_ISB => write!(f, ":{kind:?}:")?,

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

        InsnOperandKind::BARRIER_PSB => write!(f, ":{kind:?}:")?,

        InsnOperandKind::X16 => write!(f, "x16")?,

        InsnOperandKind::SME_ZT0 => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_ZT0_INDEX => write!(f, ":{kind:?}:")?,

        InsnOperandKind::SME_ZT0_LIST => write!(f, ":{kind:?}:")?,

        InsnOperandKind::BARRIER_GCSB => write!(f, ":{kind:?}:")?,

        InsnOperandKind::BTI_TARGET => write!(f, ":{kind:?}:")?,

        InsnOperandKind::MOPS_ADDR_Rd | InsnOperandKind::MOPS_ADDR_Rs => write!(f, ":{kind:?}:")?,

        InsnOperandKind::MOPS_WB_Rn => write!(f, ":{kind:?}:")?,
    };

    Ok(())
}

/// Format an instruction to a string. It does not use the aliases yet
/// and always emits the primary mnemonic.
/// The program counter is useful for the PC-relative addressing to
/// emit the target address in the disassembly rather than the offset.
pub fn format_insn_pc(pc: u64, f: &mut impl Write, opcode: &Opcode) -> core::fmt::Result {
    let definition = opcode.definition();
    let bits = opcode.bits();

    write!(f, "{}", definition.mnemonic)?;
    if definition.flags.contains(InsnFlags::IS_COND) {
        // Conditional branch or the consistent conditional branch.
        let cond = bit_range(bits, 0, 4);
        let cond = cond_name(cond);
        write!(f, "{} ", cond)?;
    }
    write!(f, "\t")?;

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

impl Display for Opcode {
    /// The program counter is not used for the PC-relative addressing here.
    /// Thus this default formattikng is not able to emit the target address
    /// in the disassembly so it emits the offset.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        format_insn_pc(0, f, self)
    }
}
