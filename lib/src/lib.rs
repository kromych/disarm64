//! This is a library for decoding ARM64 instructions.
//!
//! The main entry point is the `decode` function, which decodes a single instruction:
//!
//! ```
//! use disarm64::decoder;
//! use disarm64_defn::defn::InsnOpcode;
//!
//! let insn = decoder::decode(0x11000000).unwrap();
//!
//! println!("Instruction: {insn:?}");
//! println!("Formatted: {insn}");
//! println!("Definition: {:?}", insn.definition());
//! ```
//!

#![no_std]

#[cfg(all(
    feature = "full",
    any(feature = "load_store", feature = "system", feature = "exception")
))]
compile_error!("Cannot use the 'full' feature with any of 'load_store', 'system', 'exception'");

use core::fmt::Display;
use core::fmt::Formatter;

#[cfg(feature = "full")]
pub mod decoder_full;
#[cfg(feature = "full")]
pub use decoder_full as decoder;
#[cfg(feature = "full")]
pub use decoder_full::Opcode;
#[cfg(feature = "full")]
impl Display for Opcode {
    /// The program counter is not used for the PC-relative addressing here.
    /// Thus this default formattikng is not able to emit the target address
    /// in the disassembly so it emits the offset.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        crate::format_insn::format_insn_pc(0, f, self)
    }
}

#[cfg(feature = "exception")]
pub mod decoder_exception;
#[cfg(feature = "exception")]
pub use decoder_exception::Opcode as ExceptionOpcode;
#[cfg(feature = "exception")]
impl Display for ExceptionOpcode {
    /// The program counter is not used for the PC-relative addressing here.
    /// Thus this default formattikng is not able to emit the target address
    /// in the disassembly so it emits the offset.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        crate::format_insn::format_insn_pc(0, f, self)
    }
}

#[cfg(feature = "load_store")]
pub mod decoder_load_store;
#[cfg(feature = "load_store")]
pub use decoder_load_store::Opcode as LoadStoreOpcode;
#[cfg(feature = "load_store")]
impl Display for LoadStoreOpcode {
    /// The program counter is not used for the PC-relative addressing here.
    /// Thus this default formattikng is not able to emit the target address
    /// in the disassembly so it emits the offset.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        crate::format_insn::format_insn_pc(0, f, self)
    }
}

#[cfg(feature = "system")]
pub mod decoder_system;
#[cfg(feature = "system")]
pub use decoder_system::Opcode as SystemOpcode;
#[cfg(feature = "system")]
impl Display for SystemOpcode {
    /// The program counter is not used for the PC-relative addressing here.
    /// Thus this default formattikng is not able to emit the target address
    /// in the disassembly so it emits the offset.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        crate::format_insn::format_insn_pc(0, f, self)
    }
}

pub mod format_insn;
pub mod operand;
pub mod registers;

fn bit_range(bits: u32, start: u32, width: u32) -> u32 {
    (bits >> start) & ((1 << width) - 1)
}

fn bit_set(bits: u32, bit: u32) -> bool {
    bits & (1 << bit) != 0
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

const LOG2_TAG_GRANULE: u32 = 4;
