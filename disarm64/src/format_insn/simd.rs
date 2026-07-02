//! SIMD register, element, register list, and address formatting.

use crate::registers::{get_int_reg_name, get_simd_reg_name, SimdRegArrangement};
use core::fmt::Write;
use disarm64_defn::{defn, InsnClass, InsnFlags, InsnOperandKind, InsnOperandQualifier};

use super::bits::{bit_range, bit_set};
use super::qualifier::{
    asimdins_qualifier_idx, decode_imm5_element, qualifier_element_size, qualifier_to_simd_reg,
    resolve_em_qualifier_idx, resolve_sizeq_arrangement,
};

/// Format a SIMD register operand with vector arrangement.
pub(crate) fn format_simd_reg(
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

        InsnClass::ASIMDDIFF => {
            let size = bit_range(bits, 22, 2) as usize;
            match kind {
                InsnOperandKind::Vd | InsnOperandKind::Vn | InsnOperandKind::Vm => {
                    let len = operand.qualifiers.len();
                    let idx = if len >= 3 {
                        size
                    } else if len == 2 {
                        if size == 0 {
                            return write!(f, "<undefined>");
                        }
                        size - 1
                    } else {
                        0
                    };
                    match operand.qualifiers.get(idx).and_then(qualifier_to_simd_reg) {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                }
                _ => return write!(f, "<undefined>"),
            }
        }

        InsnClass::ASIMDSHF => {
            let immh = bit_range(bits, 19, 4);
            let q = bit_set(bits, 30);
            let q_idx = q as usize;
            let immh_level: usize = if immh >= 8 {
                3
            } else if immh >= 4 {
                2
            } else if immh >= 2 {
                1
            } else if immh >= 1 {
                0
            } else {
                return write!(f, "<undefined>");
            };

            let quals = operand.qualifiers;
            let idx = match (quals.len(), quals.first()) {
                (7, Some(InsnOperandQualifier::V_8B)) => {
                    if immh_level == 3 {
                        if !q {
                            return write!(f, "<undefined>");
                        }
                        6
                    } else {
                        immh_level * 2 + q_idx
                    }
                }
                (2, _) => {
                    let arrangement = match (immh_level, q) {
                        (0, false) => SimdRegArrangement::Vector8B,
                        (0, true) => SimdRegArrangement::Vector16B,
                        (1, false) => SimdRegArrangement::Vector4H,
                        (1, true) => SimdRegArrangement::Vector8H,
                        (2, false) => SimdRegArrangement::Vector2S,
                        (2, true) => SimdRegArrangement::Vector4S,
                        (3, true) => SimdRegArrangement::Vector2D,
                        _ => return write!(f, "<undefined>"),
                    };
                    return write!(f, "{}", get_simd_reg_name(reg_no, arrangement));
                }
                (3, Some(InsnOperandQualifier::V_2S)) => {
                    if immh_level >= 3 {
                        if !q {
                            return write!(f, "<undefined>");
                        }
                        2
                    } else if immh_level == 2 {
                        q_idx
                    } else {
                        return write!(f, "<undefined>");
                    }
                }
                (3, _) => {
                    if immh_level >= 3 {
                        return write!(f, "<undefined>");
                    }
                    immh_level
                }
                _ => return write!(f, "<undefined>"),
            };
            match quals.get(idx).and_then(qualifier_to_simd_reg) {
                Some(arr) => arr,
                None => return write!(f, "<undefined>"),
            }
        }

        InsnClass::ASIMDINS => {
            let idx = match asimdins_qualifier_idx(bits) {
                Some(i) => i,
                None => return write!(f, "<undefined>"),
            };
            match operand.qualifiers.get(idx).and_then(qualifier_to_simd_reg) {
                Some(arr) => arr,
                None => return write!(f, "<undefined>"),
            }
        }

        _ => {
            if let Some(qual) = operand.qualifiers.first() {
                if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD)
                    && matches!(qual, InsnOperandQualifier::V_8B)
                    && operand.qualifiers.len() >= 4
                {
                    // Qualifier list starting with V_8B (4, 6, or 7 elements):
                    // [8B, 16B, 4H, 8H, (2S, 4S, (2D))]
                    let size = bit_range(bits, 22, 2) as usize;
                    let q = bit_set(bits, 30);
                    let idx = if size < 3 {
                        size * 2 + q as usize
                    } else if q {
                        6
                    } else {
                        return write!(f, "<undefined>");
                    };
                    match operand.qualifiers.get(idx).and_then(qualifier_to_simd_reg) {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                } else if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD)
                    && matches!(qual, InsnOperandQualifier::V_2S)
                    && operand.qualifiers.len() == 3
                {
                    // FP [V_2S, V_4S, V_2D]: bit22 = 0 (single) / 1 (double), Q = width
                    let fp_double = bit_set(bits, 22);
                    let q = bit_set(bits, 30);
                    match (fp_double, q) {
                        (false, false) => SimdRegArrangement::Vector2S,
                        (false, true) => SimdRegArrangement::Vector4S,
                        (true, true) => SimdRegArrangement::Vector2D,
                        _ => return write!(f, "<undefined>"),
                    }
                } else if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD)
                    && matches!(qual, InsnOperandQualifier::V_4H)
                    && operand.qualifiers.len() >= 3
                {
                    // V_4H-starting lists
                    let size = bit_range(bits, 22, 2) as usize;
                    let q = bit_set(bits, 30) as usize;
                    let len = operand.qualifiers.len();
                    let idx = if len >= 6 {
                        if size > 2 {
                            return write!(f, "<undefined>");
                        }
                        size * 2 + q
                    } else if len >= 4 {
                        if !(1..=2).contains(&size) {
                            return write!(f, "<undefined>");
                        }
                        (size - 1) * 2 + q
                    } else {
                        if size == 0 {
                            return write!(f, "<undefined>");
                        }
                        (size - 1) * 2 + q
                    };
                    match operand.qualifiers.get(idx).and_then(qualifier_to_simd_reg) {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                } else if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD)
                    && operand.qualifiers.len() == 3
                {
                    // 3-element size-indexed lists (narrowing/widening)
                    let size = bit_range(bits, 22, 2) as usize;
                    match operand.qualifiers.get(size).and_then(qualifier_to_simd_reg) {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                } else if definition.flags.contains(InsnFlags::HAS_SPEC_DECODE_RULES)
                    && operand.qualifiers.len() == 2
                {
                    // FP narrowing/widening with bit22 selecting qualifier
                    let fp_double = bit_set(bits, 22) as usize;
                    match operand
                        .qualifiers
                        .get(fp_double)
                        .and_then(qualifier_to_simd_reg)
                    {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                } else if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD)
                    && operand.qualifiers.len() == 2
                {
                    // 2-element HAS_SIZEQ_FIELD:
                    // Same element type (e.g. V_2S/V_4S): Q selects variant
                    // Different element type (e.g. V_4S/V_2D): size selects
                    let same_element = qualifier_element_size(&operand.qualifiers[0])
                        == qualifier_element_size(&operand.qualifiers[1]);
                    let q = bit_set(bits, 30) as usize;
                    let idx = if same_element {
                        q
                    } else {
                        let size = bit_range(bits, 22, 2) as usize;
                        if size > 0 {
                            size - 1
                        } else {
                            0
                        }
                    };
                    match operand.qualifiers.get(idx).and_then(qualifier_to_simd_reg) {
                        Some(arr) => arr,
                        None => return write!(f, "<undefined>"),
                    }
                } else {
                    // Fixed or compact qualifier list: Q bit selects width variant
                    let double = if definition.flags.contains(InsnFlags::HAS_SIZEQ_FIELD) {
                        bit_set(bits, 30)
                    } else {
                        false
                    };
                    if !double {
                        match qualifier_to_simd_reg(qual) {
                            Some(arr) => arr,
                            None => return write!(f, "<undefined>"),
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
                            _ => return write!(f, "<undefined>"),
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

/// Format Ed/En SIMD element operands for ASIMDINS instructions.
pub(crate) fn format_simd_element_ed_en(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
) -> core::fmt::Result {
    let reg_no = if let Some(bf) = operand.bit_fields.first() {
        bit_range(bits, bf.lsb.into(), bf.width.into())
    } else {
        return write!(f, "<undefined>");
    };

    let imm5 = bit_range(bits, 16, 5);
    let Some((suffix, _)) = decode_imm5_element(imm5) else {
        return write!(f, "<undefined>");
    };

    let index_from_imm5 = |s: char, imm: u32| -> u32 {
        match s {
            'b' => imm >> 1,
            'h' => imm >> 2,
            's' => imm >> 3,
            'd' => imm >> 4,
            _ => unreachable!(),
        }
    };

    let index = match operand.kind {
        InsnOperandKind::Ed => index_from_imm5(suffix, imm5),
        InsnOperandKind::En => {
            let has_ed = definition
                .operands
                .iter()
                .any(|op| op.kind == InsnOperandKind::Ed);
            if has_ed {
                let imm4 = bit_range(bits, 11, 4);
                match suffix {
                    'b' => imm4,
                    'h' => imm4 >> 1,
                    's' => imm4 >> 2,
                    'd' => imm4 >> 3,
                    _ => unreachable!(),
                }
            } else {
                index_from_imm5(suffix, imm5)
            }
        }
        _ => unreachable!(),
    };

    write!(f, "v{reg_no}.{suffix}[{index}]")
}

/// Format Em/Em16 SIMD element operands.
pub(crate) fn format_simd_element_em(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
) -> core::fmt::Result {
    let reg_no = if let Some(bf) = operand.bit_fields.first() {
        bit_range(bits, bf.lsb.into(), bf.width.into())
    } else {
        return write!(f, "<undefined>");
    };

    let qual_idx =
        resolve_em_qualifier_idx(operand.qualifiers, definition.flags, bits, definition.mask);
    let qual = operand.qualifiers.get(qual_idx);

    match qual {
        Some(InsnOperandQualifier::S_H) if operand.kind == InsnOperandKind::Em16 => {
            // Em16 16-bit: register V0-V15, index = H:L:M
            let h = bit_range(bits, 11, 1);
            let l = bit_range(bits, 21, 1);
            let m = bit_range(bits, 20, 1);
            let index = (h << 2) | (l << 1) | m;
            let reg_no = reg_no & 0xF;
            write!(f, "v{reg_no}.h[{index}]")
        }
        Some(InsnOperandQualifier::S_H) => {
            // Em 16-bit (FCMLA): full register, index = H:L
            let h = bit_range(bits, 11, 1);
            let l = bit_range(bits, 21, 1);
            let index = (h << 1) | l;
            write!(f, "v{reg_no}.h[{index}]")
        }
        Some(InsnOperandQualifier::S_S) => {
            // 32-bit element: index = imm2[13:12] for CRYPTOSM3, H:L otherwise
            let index = if definition.class == InsnClass::CRYPTOSM3 {
                bit_range(bits, 12, 2)
            } else {
                let h = bit_range(bits, 11, 1);
                let l = bit_range(bits, 21, 1);
                (h << 1) | l
            };
            write!(f, "v{reg_no}.s[{index}]")
        }
        Some(InsnOperandQualifier::S_D) => {
            let index = bit_range(bits, 11, 1);
            write!(f, "v{reg_no}.d[{index}]")
        }
        Some(InsnOperandQualifier::S_4B) => {
            let h = bit_range(bits, 11, 1);
            let l = bit_range(bits, 21, 1);
            let index = (h << 1) | l;
            let reg_no = reg_no & 0xF;
            write!(f, "v{reg_no}.4b[{index}]")
        }
        Some(InsnOperandQualifier::S_2H) => {
            let h = bit_range(bits, 11, 1);
            let l = bit_range(bits, 21, 1);
            let index = (h << 1) | l;
            let reg_no = reg_no & 0xF;
            write!(f, "v{reg_no}.2h[{index}]")
        }
        _ => write!(f, "<undefined>"),
    }
}

/// Write a register list `{ v{n}.{arr}, v{n+1}.{arr}, ... }`.
pub(crate) fn write_reglist(
    f: &mut impl Write,
    first_reg: u32,
    count: u32,
    arrangement: &str,
) -> core::fmt::Result {
    write!(f, "{{ ")?;
    for i in 0..count {
        if i > 0 {
            write!(f, ", ")?;
        }
        let reg = (first_reg + i) & 31;
        write!(f, "v{reg}.{arrangement}")?;
    }
    write!(f, " }}")
}

/// Write an element register list `{ v{n}.{suffix}, ... }[index]`.
pub(crate) fn write_elemlist(
    f: &mut impl Write,
    first_reg: u32,
    count: u32,
    suffix: char,
    index: u32,
) -> core::fmt::Result {
    write!(f, "{{ ")?;
    for i in 0..count {
        if i > 0 {
            write!(f, ", ")?;
        }
        let reg = (first_reg + i) & 31;
        write!(f, "v{reg}.{suffix}")?;
    }
    write!(f, " }}[{index}]")
}

/// Format LVn register list for ASIMDTBL (tbl/tbx).
pub(crate) fn format_simd_reglist_lvn(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
) -> core::fmt::Result {
    let first_reg = if let Some(bf) = operand.bit_fields.first() {
        bit_range(bits, bf.lsb.into(), bf.width.into())
    } else {
        return write!(f, "<undefined>");
    };
    let len = bit_range(bits, 13, 2);
    let count = len + 1;
    write_reglist(f, first_reg, count, "16b")
}

/// Format LVt/LVt_AL register list for ASISDLSE/ASISDLSO.
pub(crate) fn format_simd_reglist_lvt(
    f: &mut impl Write,
    bits: u32,
    operand: &defn::InsnOperand,
    definition: &defn::Insn,
) -> core::fmt::Result {
    let rt = bit_range(bits, 0, 5);

    let count = match definition.class {
        InsnClass::ASISDLSE | InsnClass::ASISDLSEP => {
            let opcode_field = bit_range(bits, 12, 4);
            match opcode_field {
                0b0000 => 4, // st4/ld4
                0b0010 => 4, // st1x4/ld1x4
                0b0100 => 3, // st3/ld3
                0b0110 => 3, // st1x3/ld1x3
                0b0111 => 1, // st1x1/ld1x1
                0b1000 => 2, // st2/ld2
                0b1010 => 2, // st1x2/ld1x2
                _ => return write!(f, "<undefined>"),
            }
        }
        InsnClass::ASISDLSO | InsnClass::ASISDLSOP => {
            let r = bit_range(bits, 21, 1);
            let b13 = bit_range(bits, 13, 1);
            match (r, b13) {
                (0, 0) => 1,
                (0, 1) => 3,
                (1, 0) => 2,
                (1, 1) => 4,
                _ => unreachable!(),
            }
        }
        _ => return write!(f, "<undefined>"),
    };

    let size = bit_range(bits, 10, 2);
    let q = bit_set(bits, 30);
    let arrangement = resolve_sizeq_arrangement(operand.qualifiers, size, q);
    if let Some(arr) = arrangement {
        write_reglist(f, rt, count, arr)
    } else {
        write!(f, "<undefined>")
    }
}

/// Format LEt element register list for ASISDLSO.
pub(crate) fn format_simd_elemlist(
    f: &mut impl Write,
    bits: u32,
    _definition: &defn::Insn,
) -> core::fmt::Result {
    let rt = bit_range(bits, 0, 5);

    let r = bit_range(bits, 21, 1);
    let b13 = bit_range(bits, 13, 1);
    let count = match (r, b13) {
        (0, 0) => 1,
        (0, 1) => 3,
        (1, 0) => 2,
        (1, 1) => 4,
        _ => unreachable!(),
    };

    let opcode_upper = bit_range(bits, 14, 2);
    let q = bit_range(bits, 30, 1);
    let s = bit_range(bits, 12, 1);
    let size_field = bit_range(bits, 10, 2);

    match opcode_upper {
        0b00 => {
            let index = (q << 3) | (s << 2) | size_field;
            write_elemlist(f, rt, count, 'b', index)
        }
        0b01 => {
            if size_field & 1 != 0 {
                return write!(f, "<undefined>");
            }
            let index = (q << 2) | (s << 1) | (size_field >> 1);
            write_elemlist(f, rt, count, 'h', index)
        }
        0b10 => {
            if size_field & 1 == 0 {
                let index = (q << 1) | s;
                write_elemlist(f, rt, count, 's', index)
            } else if s == 0 {
                write_elemlist(f, rt, count, 'd', q)
            } else {
                write!(f, "<undefined>")
            }
        }
        _ => write!(f, "<undefined>"),
    }
}

/// Format SIMD_ADDR_POST: post-indexed addressing for SIMD load/store.
pub(crate) fn format_simd_addr_post(
    f: &mut impl Write,
    bits: u32,
    definition: &defn::Insn,
) -> core::fmt::Result {
    let rn = bit_range(bits, 5, 5);
    let rm = bit_range(bits, 16, 5);
    let rn_name = get_int_reg_name(true, rn as u8, false);

    if rm == 31 {
        let imm = match definition.class {
            InsnClass::ASISDLSEP => {
                let opcode_field = bit_range(bits, 12, 4);
                let num_regs = match opcode_field {
                    0b0000 => 4u32,
                    0b0010 => 4,
                    0b0100 => 3,
                    0b0110 => 3,
                    0b0111 => 1,
                    0b1000 => 2,
                    0b1010 => 2,
                    _ => return write!(f, "<undefined>"),
                };
                let q = bit_set(bits, 30);
                num_regs * if q { 16 } else { 8 }
            }
            InsnClass::ASISDLSOP => {
                let r = bit_range(bits, 21, 1);
                let b13 = bit_range(bits, 13, 1);
                let num_regs = match (r, b13) {
                    (0, 0) => 1u32,
                    (0, 1) => 3,
                    (1, 0) => 2,
                    (1, 1) => 4,
                    _ => unreachable!(),
                };
                let opcode_upper = bit_range(bits, 14, 2);
                let size_field = bit_range(bits, 10, 2);
                let s = bit_range(bits, 12, 1);
                let elem_size = match opcode_upper {
                    0b00 => 1u32,
                    0b01 => 2,
                    0b10 if size_field & 1 == 0 => 4,
                    0b10 if s == 0 => 8,
                    0b11 => 1u32 << size_field,
                    _ => return write!(f, "<undefined>"),
                };
                num_regs * elem_size
            }
            _ => return write!(f, "<undefined>"),
        };
        write!(f, "[{rn_name}], #{imm}")
    } else {
        let rm_name = get_int_reg_name(true, rm as u8, true);
        write!(f, "[{rn_name}], {rm_name}")
    }
}
