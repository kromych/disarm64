//! This is a library for decoding ARM64 instructions.
//!
//! The main entry point is the `decode` function, which decodes a single instruction:
//!
//! ```
//! use disarm64::decoder;
//! use disarm64::{InsnDisplay, InsnOpcode};
//!
//! let insn = decoder::decode(0x11000000).unwrap();
//!
//! println!("Instruction: {insn:?}");
//! println!("Formatted: {insn}");
//! println!("At 0x1000: {}", insn.display_at(0x1000));
//! println!("Definition: {:?}", insn.definition());
//! ```
//!

#![no_std]

// The instruction model lives in disarm64_defn; re-export the types a consumer
// needs so decoding and inspecting instructions needs only this crate.
pub use disarm64_defn::defn::Insn;
pub use disarm64_defn::defn::InsnOpcode;
pub use disarm64_defn::defn::InsnOperand;
pub use format_insn::DisplayAt;
pub use format_insn::InsnDisplay;

/// Give a decoder's `Opcode` alias a `Display` that formats the instruction
/// without a program counter. The module declaration and re-export are kept out
/// of the macro so rustfmt can still discover the generated module files.
macro_rules! impl_opcode_display {
    ($feat:literal, $alias:ident) => {
        #[cfg(feature = $feat)]
        impl core::fmt::Display for $alias {
            /// The program counter is not used for PC-relative addressing here, so
            /// the default formatting emits the offset rather than the target
            /// address.
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                crate::format_insn::format_insn_pc(0, f, self)
            }
        }
    };
}

#[cfg(feature = "full")]
pub mod decoder_full;
#[cfg(feature = "full")]
pub use decoder_full as decoder;
#[cfg(feature = "full")]
pub use decoder_full::Opcode;
impl_opcode_display!("full", Opcode);

#[cfg(feature = "exception")]
pub mod decoder_exception;
#[cfg(feature = "exception")]
pub use decoder_exception::Opcode as ExceptionOpcode;
impl_opcode_display!("exception", ExceptionOpcode);

#[cfg(feature = "load_store")]
pub mod decoder_load_store;
#[cfg(feature = "load_store")]
pub use decoder_load_store::Opcode as LoadStoreOpcode;
impl_opcode_display!("load_store", LoadStoreOpcode);

#[cfg(feature = "system")]
pub mod decoder_system;
#[cfg(feature = "system")]
pub use decoder_system::Opcode as SystemOpcode;
impl_opcode_display!("system", SystemOpcode);

pub mod format_insn;
mod register_tables;
pub mod registers;
