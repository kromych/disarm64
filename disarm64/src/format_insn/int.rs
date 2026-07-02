//! Integer register and shift/extend operand formatting.

use crate::registers::get_int_reg_name;
use bitfield_struct::bitfield;
use core::fmt::Write;
use disarm64_defn::{defn, InsnClass, InsnFlags, InsnOperandKind, InsnOperandQualifier};

use super::bits::{bit_range, bit_set};
use super::qualifier::asimdins_qualifier_idx;

/// Format an integer register (32-bit or 64-bit), optionally as part of a pair.
pub(crate) fn format_int_operand_reg_pair(
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
    } else if definition.class == InsnClass::ASIMDINS
        && flags.contains(InsnFlags::HAS_ADVSIMV_GPRSIZE_IN_Q)
    {
        // SMOV/UMOV: GPR size from Q bit
        bit_set(bits, 30)
    } else if definition.class == InsnClass::ASIMDINS && !operand.qualifiers.is_empty() {
        // DUP (general): GPR size from imm5 qualifier index
        let idx = asimdins_qualifier_idx(bits).unwrap_or(0);
        matches!(operand.qualifiers.get(idx), Some(InsnOperandQualifier::X))
    } else if definition.class == InsnClass::TESTBRANCH {
        // TBZ/TBNZ: the tested bit number implies the register width; bit 31
        // is the top bit of the bit number, so bits 0-31 are in the W view.
        bit_set(bits, 31)
    } else if operand.qualifiers.is_empty()
        || operand
            .qualifiers
            .iter()
            .all(|q| matches!(q, InsnOperandQualifier::X | InsnOperandQualifier::SP))
    {
        // Only 64-bit register views are possible.
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

/// Format an integer register (32-bit or 64-bit).
pub(crate) fn format_int_operand_reg(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
    with_zr: bool,
) -> core::fmt::Result {
    format_int_operand_reg_pair(false, f, bits, operand, definition, with_zr)
}

/// Format a register with extended shift operand.
pub(crate) fn format_operand_reg_ext(f: &mut impl Write, bits: u32) -> core::fmt::Result {
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
                "lsl"
            } else {
                ""
            }
        } else {
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

/// Format a register with shift operand.
pub(crate) fn format_operand_reg_shift(f: &mut impl Write, bits: u32) -> core::fmt::Result {
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
        0b00 => "lsl",
        0b01 => "lsr",
        0b10 => "asr",
        0b11 => {
            if !bit_set(bits, 24) {
                "ror"
            } else {
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
