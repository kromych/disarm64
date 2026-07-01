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

/// Declare a decoder module for one feature: bring in the generated module,
/// re-export its `Opcode` under the given alias, and give the alias a `Display`
/// that formats the instruction without a program counter.
macro_rules! decoder_module {
    ($feat:literal, $module:ident, $alias:ident) => {
        #[cfg(feature = $feat)]
        pub mod $module;
        #[cfg(feature = $feat)]
        pub use $module::Opcode as $alias;
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

decoder_module!("full", decoder_full, Opcode);
#[cfg(feature = "full")]
pub use decoder_full as decoder;

decoder_module!("exception", decoder_exception, ExceptionOpcode);
decoder_module!("load_store", decoder_load_store, LoadStoreOpcode);
decoder_module!("system", decoder_system, SystemOpcode);

pub mod format_insn;
pub mod registers;
