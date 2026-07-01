//! Formatting instructions to human-readable strings.
//!
//! The module is organized as:
//! - `bits`: Bit manipulation utilities
//! - `qualifier`: Qualifier mapping and indexing helpers
//! - `fp`: Floating-point register and immediate formatting
//! - `int`: Integer register formatting
//! - `simd`: SIMD register, element, and register list formatting

#![cfg_attr(
    not(feature = "full"),
    allow(unused_variables, dead_code, unused_imports)
)]

pub(crate) mod bits;
pub(crate) mod fp;
pub(crate) mod int;
pub(crate) mod qualifier;
pub(crate) mod simd;

use crate::registers::get_int_reg_name;
use crate::registers::get_sys_reg_name;
use crate::registers::sys_reg_number;
use core::fmt::Write;
use defn::InsnOpcode;
use disarm64_defn::defn;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandKind;

use bits::{bit_range, bit_set, cond_name, decode_limm, sign_extend};
use fp::{format_fp_reg, fp_expand_imm};
use int::{
    format_int_operand_reg, format_int_operand_reg_pair, format_operand_reg_ext,
    format_operand_reg_shift,
};
use simd::{
    format_simd_addr_post, format_simd_element_ed_en, format_simd_element_em, format_simd_elemlist,
    format_simd_reg, format_simd_reglist_lvn, format_simd_reglist_lvt,
};

const LOG2_TAG_GRANULE: u32 = 4;

/// Format an operand to a string.
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
        InsnOperandKind::SVE_Vd
        | InsnOperandKind::SVE_Vm
        | InsnOperandKind::SVE_Vn
        | InsnOperandKind::Va
        | InsnOperandKind::Vd
        | InsnOperandKind::Vn
        | InsnOperandKind::Vm => format_simd_reg(f, bits, operand, definition)?,

        #[cfg(feature = "full")]
        InsnOperandKind::Ed | InsnOperandKind::En => {
            format_simd_element_ed_en(f, bits, operand, definition)?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::Em | InsnOperandKind::Em16 => {
            format_simd_element_em(f, bits, operand, definition)?
        }

        #[cfg(feature = "full")]
        InsnOperandKind::VdD1 | InsnOperandKind::VnD1 => {
            if let Some(bf) = operand.bit_fields.first() {
                let reg_no = bit_range(bits, bf.lsb.into(), bf.width.into());
                write!(f, "v{reg_no}.d[1]")?;
            } else {
                write!(f, ":{kind:?}:")?;
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::LVn => {
            format_simd_reglist_lvn(f, bits, operand)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::LVt | InsnOperandKind::LVt_AL => {
            format_simd_reglist_lvt(f, bits, operand, definition)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::LEt => {
            format_simd_elemlist(f, bits, definition)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SME_PNn3_INDEX1 | InsnOperandKind::SME_PNn3_INDEX2 => {
            write!(f, ":{kind:?}:")?
        }

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
        InsnOperandKind::IMM_VLSR => {
            let immh = bit_range(bits, 19, 4);
            let immb = bit_range(bits, 16, 3);
            let esize = if immh >= 8 {
                64u32
            } else if immh >= 4 {
                32
            } else if immh >= 2 {
                16
            } else {
                8
            };
            let shift = 2 * esize - ((immh << 3) | immb);
            write!(f, "#{shift}")?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IMM_VLSL => {
            let immh = bit_range(bits, 19, 4);
            let immb = bit_range(bits, 16, 3);
            let esize = if immh >= 8 {
                64u32
            } else if immh >= 4 {
                32
            } else if immh >= 2 {
                16
            } else {
                8
            };
            let shift = ((immh << 3) | immb) - esize;
            write!(f, "#{shift}")?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::SHLL_IMM => {
            let size = bit_range(bits, 22, 2);
            write!(f, "#{}", 8u32 << size)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IMM_ROT1 | InsnOperandKind::IMM_ROT2 => {
            let rot = if let Some(bf) = operand.bit_fields.first() {
                bit_range(bits, bf.lsb.into(), bf.width.into())
            } else {
                0
            };
            write!(f, "#{}", rot * 90)?;
        }
        #[cfg(feature = "full")]
        InsnOperandKind::IMM_ROT3 => {
            let rot = if let Some(bf) = operand.bit_fields.first() {
                bit_range(bits, bf.lsb.into(), bf.width.into())
            } else {
                0
            };
            write!(f, "#{}", rot * 180 + 90)?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IDX => {
            let imm4 = bit_range(bits, 11, 4);
            let q = bit_set(bits, 30);
            if !q && imm4 > 7 {
                write!(f, "<undefined>")?;
            } else {
                write!(f, "#{imm4}")?;
            }
        }

        #[cfg(feature = "full")]
        InsnOperandKind::IMM => {
            let imm6 = bit_range(bits, 10, 6);
            write!(f, "#{imm6:#x}")?;
        }

        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_SIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as i8)?,
        #[cfg(feature = "full")]
        InsnOperandKind::CSSC_UIMM8 => write!(f, "#{}", bit_range(bits, 10, 8) as u8)?,
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM4_ADDG => write!(f, "#{}", bit_range(bits, 10, 4))?,
        #[cfg(feature = "full")]
        InsnOperandKind::UIMM10 => write!(f, "#{}", bit_range(bits, 16, 6) << LOG2_TAG_GRANULE)?,

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
            if cmode >> 1 == 0b110 {
                let msl = if bit_set(cmode, 0) { 16 } else { 8 };
                return write!(f, "#{imm8:#x}, MSL #{msl}");
            }
            if cmode & 0b1001 == 0 || cmode & 0b1001 == 0b0001 {
                let lsl = bit_range(cmode, 1, 2) * 8;
                if lsl != 0 {
                    return write!(f, "#{imm8:#x}, LSL #{lsl}");
                } else {
                    return write!(f, "#{imm8:#x}");
                }
            }
            if cmode & 0b1101 == 0b1000 || cmode & 0b1101 == 0b1001 {
                let lsl = if bit_set(cmode, 1) { 8 } else { 0 };
                if lsl != 0 {
                    return write!(f, "#{imm8:#x}, LSL #{lsl}");
                } else {
                    return write!(f, "#{imm8:#x}");
                }
            }
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
        InsnOperandKind::SIMD_ADDR_POST => {
            format_simd_addr_post(f, bits, definition)?;
            *stop = true;
        }

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

            let ldst_offset_only = definition.class == disarm64_defn::InsnClass::LDST_UNPRIV
                || definition.class == disarm64_defn::InsnClass::LDST_UNSCALED;
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

        InsnOperandKind::X16 => write!(f, "x16")?,

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
                write!(f, "[{rd}]!, [{rs}]!, {rn}!")?
            } else {
                write!(f, "[{rd}]!, {rn}!, {rs}")?
            }
        }

        // Operand kinds not yet given a dedicated formatter print their name.
        #[cfg(feature = "full")]
        InsnOperandKind::SVE_VZn
        | InsnOperandKind::SVE_Pd
        | InsnOperandKind::SVE_Pg3
        | InsnOperandKind::SVE_Pg4_5
        | InsnOperandKind::SVE_Pg4_10
        | InsnOperandKind::SVE_Pg4_16
        | InsnOperandKind::SVE_Pm
        | InsnOperandKind::SVE_Pn
        | InsnOperandKind::SVE_Pt
        | InsnOperandKind::SME_Pm
        | InsnOperandKind::SVE_PNd
        | InsnOperandKind::SVE_PNg4_10
        | InsnOperandKind::SVE_PNn
        | InsnOperandKind::SVE_PNt
        | InsnOperandKind::SME_PNd3
        | InsnOperandKind::SME_PNg3
        | InsnOperandKind::SME_PNn
        | InsnOperandKind::SME_Pdx2
        | InsnOperandKind::SME_PdxN
        | InsnOperandKind::SVE_Za_5
        | InsnOperandKind::SVE_Za_16
        | InsnOperandKind::SVE_Zd
        | InsnOperandKind::SVE_Zm_5
        | InsnOperandKind::SVE_Zm_16
        | InsnOperandKind::SVE_Zn
        | InsnOperandKind::SVE_Zt
        | InsnOperandKind::SME_Zm
        | InsnOperandKind::SVE_ZnxN
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
        | InsnOperandKind::SME_Zt4
        | InsnOperandKind::SVE_Zm3_INDEX
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
        | InsnOperandKind::SVE_Zm_imm4
        | InsnOperandKind::SME_ZAda_2b
        | InsnOperandKind::SME_ZAda_3b
        | InsnOperandKind::SME_ZA_HV_idx_src
        | InsnOperandKind::SME_ZA_HV_idx_srcxN
        | InsnOperandKind::SME_ZA_HV_idx_dest
        | InsnOperandKind::SME_ZA_HV_idx_destxN
        | InsnOperandKind::SME_ZA_HV_idx_ldstr
        | InsnOperandKind::SME_list_of_64bit_tiles
        | InsnOperandKind::SME_ZA_array_off1x4
        | InsnOperandKind::SME_ZA_array_off2x2
        | InsnOperandKind::SME_ZA_array_off2x4
        | InsnOperandKind::SME_ZA_array_off3_0
        | InsnOperandKind::SME_ZA_array_off3_5
        | InsnOperandKind::SME_ZA_array_off3x2
        | InsnOperandKind::SME_ZA_array_off4
        | InsnOperandKind::SME_ZA_array_vrsb_1
        | InsnOperandKind::SME_ZA_array_vrsh_1
        | InsnOperandKind::SME_ZA_array_vrss_1
        | InsnOperandKind::SME_ZA_array_vrsd_1
        | InsnOperandKind::SME_ZA_array_vrsb_2
        | InsnOperandKind::SME_ZA_array_vrsh_2
        | InsnOperandKind::SME_ZA_array_vrss_2
        | InsnOperandKind::SME_ZA_array_vrsd_2
        | InsnOperandKind::SME_SM_ZA
        | InsnOperandKind::SME_PnT_Wm_imm
        | InsnOperandKind::SME_VLxN_10
        | InsnOperandKind::SME_VLxN_13
        | InsnOperandKind::WIDTH
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
        | InsnOperandKind::SVE_IMM_ROT1
        | InsnOperandKind::SVE_IMM_ROT2
        | InsnOperandKind::SVE_IMM_ROT3
        | InsnOperandKind::SVE_I1_HALF_ONE
        | InsnOperandKind::SVE_I1_HALF_TWO
        | InsnOperandKind::SVE_I1_ZERO_ONE
        | InsnOperandKind::SVE_PATTERN
        | InsnOperandKind::SVE_PATTERN_SCALED
        | InsnOperandKind::SVE_PRFOP
        | InsnOperandKind::IMM_MOV
        | InsnOperandKind::SVE_INV_LIMM
        | InsnOperandKind::SVE_LIMM
        | InsnOperandKind::SVE_LIMM_MOV
        | InsnOperandKind::SVE_ADDR_R
        | InsnOperandKind::SVE_ADDR_RR
        | InsnOperandKind::SVE_ADDR_RR_LSL1
        | InsnOperandKind::SVE_ADDR_RR_LSL2
        | InsnOperandKind::SVE_ADDR_RR_LSL3
        | InsnOperandKind::SVE_ADDR_RR_LSL4
        | InsnOperandKind::SVE_ADDR_RX
        | InsnOperandKind::SVE_ADDR_RX_LSL1
        | InsnOperandKind::SVE_ADDR_RX_LSL2
        | InsnOperandKind::SVE_ADDR_RX_LSL3
        | InsnOperandKind::SVE_ADDR_ZX
        | InsnOperandKind::SVE_ADDR_RZ
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
        | InsnOperandKind::SVE_ADDR_RZ_XTW3_22
        | InsnOperandKind::RCPC3_ADDR_OFFSET
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
        | InsnOperandKind::SVE_ADDR_RI_U6x8
        | InsnOperandKind::SVE_ADDR_ZI_U5
        | InsnOperandKind::SVE_ADDR_ZI_U5x2
        | InsnOperandKind::SVE_ADDR_ZI_U5x4
        | InsnOperandKind::SVE_ADDR_ZI_U5x8
        | InsnOperandKind::SVE_ADDR_ZZ_LSL
        | InsnOperandKind::SVE_ADDR_ZZ_SXTW
        | InsnOperandKind::SVE_ADDR_ZZ_UXTW
        | InsnOperandKind::SME_ZT0
        | InsnOperandKind::SME_ZT0_INDEX
        | InsnOperandKind::SME_ZT0_LIST
        | InsnOperandKind::BTI_TARGET => write!(f, ":{kind:?}:")?,

        #[cfg(any(feature = "full", feature = "system"))]
        InsnOperandKind::UIMM7
        | InsnOperandKind::SYSREG_AT
        | InsnOperandKind::SYSREG_DC
        | InsnOperandKind::SYSREG_IC
        | InsnOperandKind::SYSREG_TLBI
        | InsnOperandKind::SYSREG_TLBIP
        | InsnOperandKind::SYSREG_SR
        | InsnOperandKind::BARRIER_PSB
        | InsnOperandKind::BARRIER_GCSB => write!(f, ":{kind:?}:")?,
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

/// An opcode paired with a program counter so that `Display` renders PC-relative
/// operands as absolute target addresses. Created by [`InsnDisplay::display_at`].
pub struct DisplayAt<'a, O: InsnOpcode> {
    pc: u64,
    opcode: &'a O,
}

impl<O: InsnOpcode> core::fmt::Display for DisplayAt<'_, O> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        format_insn_pc(self.pc, f, self.opcode)
    }
}

/// Formatting helpers for any decoded opcode.
pub trait InsnDisplay: InsnOpcode + Sized {
    /// Format the instruction with `pc` as the address of the instruction itself,
    /// so PC-relative operands render as absolute targets rather than offsets.
    fn display_at(&self, pc: u64) -> DisplayAt<'_, Self> {
        DisplayAt { pc, opcode: self }
    }
}

impl<O: InsnOpcode> InsnDisplay for O {}
