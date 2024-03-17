//! This is a library for decoding ARM64 instructions.
//!
//! The main entry point is the `decode` function, which decodes a single instruction:
//!
//! ```
//! use disarm64::decoder;
//! use disarm64_defn::defn::InsnOpcode;
//!
//! let insn = decoder::decode(0x94000000).unwrap();
//!
//! println!("Instruction: {insn:?}");
//! println!("Definition: {:?}", insn.definition());
//! ```
//!

#![no_std]

pub mod decoder;
pub mod format_insn;
pub mod registers;
