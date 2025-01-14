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
pub mod registers;
