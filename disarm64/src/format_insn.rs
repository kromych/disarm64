use crate::decoder::Opcode;
use bitfield_struct::bitfield;
use disarm64_defn::defn::InsnOpcode;
use disarm64_defn::InsnBitField;
use disarm64_defn::InsnFlags;
use disarm64_defn::InsnOperandKind;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;

fn get_int_reg_name(is_64: bool, reg: u8, with_zr: bool) -> &'static str {
    debug_assert!(reg < 32);
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

    INT_REG[is_sp][is_64][reg as usize]
}

fn get_sve_reg_name(is_64: bool, reg: u8) -> &'static str {
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

    debug_assert!(reg < 32);
    let is_64 = is_64 as usize;

    SVE_REG[is_64][reg as usize]
}

/// Format a register (32-bit or 64-bit) to a string
fn format_operand_reg(
    f: &mut impl Write,
    bits: u32,
    operand: &disarm64_defn::defn::InsnOperand,
    definition: &disarm64_defn::defn::Insn,
    with_zr: bool,
) -> Result {
    let flags = definition.flags;
    let is_64 = if flags.contains(InsnFlags::HAS_SF_FIELD) {
        bits & 0x80000000 != 0
    } else {
        true
    };
    let bit_field_spec = operand
        .bit_fields
        .iter()
        .find(|bf| {
            bf.bitfield == InsnBitField::Rd
                || bf.bitfield == InsnBitField::Rn
                || bf.bitfield == InsnBitField::Rm
                || bf.bitfield == InsnBitField::Rt
                || bf.bitfield == InsnBitField::Rt2
                || bf.bitfield == InsnBitField::Rs
                || bf.bitfield == InsnBitField::Ra
                || bf.bitfield == InsnBitField::SVE_Rm
                || bf.bitfield == InsnBitField::LSE128_Rt
                || bf.bitfield == InsnBitField::LSE128_Rt2
        })
        .expect("must be bitfield definition present");
    let reg_no = (bits >> bit_field_spec.lsb) & ((1 << bit_field_spec.width) - 1);
    let reg_name = get_int_reg_name(is_64, reg_no as u8, with_zr);

    write!(f, "{reg_name}")
}

/// Format a register with extended shioft operand to a string.
/// This is a lot of logic and dependencies on the other registers
/// for 4 instructions and 2 aliases.
fn format_operand_reg_ext(f: &mut impl Write, bits: u32) -> Result {
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

    let reg_name = get_int_reg_name(reg_ext.sf(), reg_ext.regm() as u8, true);
    write!(f, "{reg_name}")?;
    if !extend.is_empty() {
        write!(f, ", {}", extend)?;

        if reg_ext.imm3() != 0 {
            write!(f, " #{}", reg_ext.imm3())?;
        }
    }

    Ok(())
}

/// Format an operand to a string
fn format_operand(
    f: &mut impl Write,
    bits: u32,
    operand: &disarm64_defn::defn::InsnOperand,
    definition: &disarm64_defn::defn::Insn,
) -> Result {
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
        | InsnOperandKind::PAIRREG
        | InsnOperandKind::PAIRREG_OR_XZR
        | InsnOperandKind::SVE_Rm
        | InsnOperandKind::LSE128_Rt
        | InsnOperandKind::LSE128_Rt2 => {
            let with_zr = true;
            format_operand_reg(f, bits, operand, definition, with_zr)?
        }

        InsnOperandKind::Rd_SP
        | InsnOperandKind::Rn_SP
        | InsnOperandKind::Rt_SP
        | InsnOperandKind::SVE_Rn_SP
        | InsnOperandKind::Rm_SP => {
            let with_zr = false;
            format_operand_reg(f, bits, operand, definition, with_zr)?
        }

        InsnOperandKind::Rm_EXT => format_operand_reg_ext(f, bits)?,

        _ => write!(f, "op?")?,
    };

    Ok(())
}

/// Format an instruction to a string. It does not use the aliases yet
/// and always emits the primary mnemonic.
pub fn format_insn(f: &mut impl Write, opcode: &Opcode) -> Result {
    let definition = opcode.definition();
    let bits = opcode.bits();

    write!(f, "{bits:08x}\t{}\t", definition.mnemonic)?;
    let op_count = definition.operands.len();
    for (i, operand) in definition.operands.iter().enumerate() {
        format_operand(f, bits, operand, definition)?;
        if i + 1 < op_count {
            write!(f, ", ")?;
        }
    }

    Ok(())
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        format_insn(f, self)
    }
}
