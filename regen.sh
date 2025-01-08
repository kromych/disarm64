#!/bin/sh

disarm64_gen ./aarch64.json -r disarm64/src/decoder_full.rs
disarm64_gen ./aarch64.json -c ic_system -r disarm64/src/decoder_system.rs
disarm64_gen ./aarch64.json -c exception -r disarm64/src/decoder_exception.rs
disarm64_gen ./aarch64.json -c ldst_imm10,ldst_imm9,ldst_pos,ldst_regoff,ldst_unpriv,ldst_unscaled,ldstexcl,ldstnapair_offs,ldstpair_indexed,ldstpair_off,loadlit,log_imm,log_shift,lse128_atomic,lse_atomic -r disarm64/src/decoder_load_store.rs

cargo fmt
cargo clippy
