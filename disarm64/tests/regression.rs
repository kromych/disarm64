//! Regression tests that lock the disassembly output byte-for-byte.
//!
//! The disassembly text is the library's contract, so any refactor must leave it
//! unchanged. Two layers guard it:
//!
//! * `smoke` — a small, always-on set of `(encoding, expected text)` pairs plus a
//!   few known-undefined encodings. It needs no external data and runs on every
//!   `cargo test`, giving fast protection for the common operand kinds.
//!
//! * `corpus` — an exhaustive differential check against the recorded per-class
//!   listings in the adjacent `disarm64_test_data` repository (about 100M
//!   encodings across 117 instruction classes). It is `#[ignore]`d because of its
//!   size; run it explicitly:
//!
//!   ```sh
//!   cargo test -p disarm64 --release -- --ignored
//!   ```
//!
//!   The data directory defaults to `../../disarm64_test_data` and can be
//!   overridden with `DISARM64_TEST_DATA`. If it is absent the test passes with a
//!   notice, so a checkout without the data still builds and tests cleanly.
//!   `DISARM64_TEST_ONLY=class1,class2` restricts the run to named classes;
//!   `DISARM64_TEST_MAX=N` caps the encodings checked per class.
//!
//! The recorded listings are produced by the full decoder, so the suite is
//! compiled only for the `full` feature.
#![cfg(feature = "full")]

use disarm64::decoder;
use disarm64::format_insn::format_insn_pc;
use disarm64::InsnOpcode;

/// The formatted operand text for an instruction, or `None` if it does not decode.
fn text(pc: u64, insn: u32) -> Option<String> {
    let opcode = decoder::decode(insn)?;
    let mut s = String::new();
    format_insn_pc(pc, &mut s, &opcode).expect("formatting to a String cannot fail");
    Some(s)
}

/// The exact stdout line the `disarm64 bin` CLI emits for one instruction, so the
/// corpus check can compare against the recorded `*-disarm64.lst` verbatim.
fn cli_line(pc: u64, insn: u32) -> String {
    match text(pc, insn) {
        Some(t) => format!("{pc:#08x}: {insn:08x}\t{t}"),
        None => format!("{pc:#08x}: {insn:08x}\t.inst\t{insn:#08x} // undefined"),
    }
}

#[test]
fn smoke() {
    // Expected text is `format_insn_pc(0, ..)` output; covers integer/fp/simd
    // registers, shifts and extends, immediates, addressing modes, pc-relative
    // targets, conditions, system encodings, an unimplemented operand stub, and an
    // operand that resolves to `<undefined>`.
    const CASES: &[(u32, &str)] = &[
        (0x11000000, "add\t\tw0, w0, #0x0"),
        (0x11c00000, "smax\t\tw0, w0, #0"),
        (0x0b200000, "add\t\tw0, w0, w0, uxtb"),
        (0x2b200000, "adds\t\tw0, w0, w0, uxtb"),
        (0x0b000000, "add\t\tw0, w0, w0"),
        (0x0a000000, "and\t\tw0, w0, w0"),
        (0x0a200000, "bic\t\tw0, w0, w0"),
        (0x12000000, "and\t\tw0, w0, #0x1"),
        (0x32000000, "orr\t\tw0, w0, #0x1"),
        (0x12800000, "movn\t\tw0, #0x0, lsl #0x0"),
        (0x52800000, "movz\t\tw0, #0x0, lsl #0x0"),
        (0x72800000, "movk\t\tw0, #0x0, lsl #0x0"),
        (0x13000000, "sbfm\t\tw0, w0, #0, #0"),
        (0x33000000, "bfm\t\tw0, w0, #0, #0"),
        (0x13800000, "extr\t\tw0, w0, w0, #0"),
        (0x1a800000, "csel\t\tw0, w0, w0, eq"),
        (0x1a800400, "csinc\t\tw0, w0, w0, eq"),
        (0x3a400800, "ccmn\t\tw0, #0x0, #0x0, eq"),
        (0x39000000, "strb\t\tw0, [x0]"),
        (0x39400000, "ldrb\t\tw0, [x0]"),
        (0x38000400, "strb\t\tw0, [x0], #0"),
        (0x38400400, "ldrb\t\tw0, [x0], #0"),
        (0x38204800, "strb\t\tw0, [x0, w0, uxtw]"),
        (0x38604800, "ldrb\t\tw0, [x0, w0, uxtw]"),
        (0x28800000, "stp\t\tw0, w0, [x0], #0"),
        (0x28c00000, "ldp\t\tw0, w0, [x0], #0"),
        (0x18000000, "ldr\t\tw0, 0x0"),
        (0x14000000, "b\t\t0x0"),
        (0x54000000, "b.eq \t\t0x0"),
        (0x34000000, "cbz\t\tw0, 0x0"),
        (0x36000000, "tbz\t\tw0, #0, 0x0"),
        (0xd61f0000, "br\t\tx0"),
        (0x10000000, "adr\t\tx0, 0x0"),
        (0x5ac00000, "rbit\t\tw0, w0"),
        (0x1ac00800, "udiv\t\tw0, w0, w0"),
        (0x1b000000, "madd\t\tw0, w0, w0, w0"),
        (0x1e284000, "frint32z\t\ts0, s0"),
        (0x1e200800, "fmul\t\ts0, s0, s0"),
        (0x1e202000, "fcmp\t\ts0, s0"),
        (0x1e201000, "fmov\t\ts0, #2"),
        (0x1e200000, "fcvtns\t\tw0, s0"),
        (0x0e200400, "shadd\t\tv0.8b, v0.8b, v0.8b"),
        (0x0f000400, "movi\t\tv0.2s, #0x0"),
        (0xd503251f, "chkfeat\t\tx16"),
        (0xd500401f, "cfinv\t\t"),
        (0xd500403f, "xaflag\t\t"),
        (0xd503201f, "nop"),
        (0xd5033fdf, "isb\t\t"),
        (0xd4000001, "svc\t\t#0x0"),
        (0xd4000002, "hvc\t\t#0x0"),
        (0x11c00008, "smax\t\tw8, w0, #0"),
        // Decoded mnemonic with an operand kind that resolves to `<undefined>`.
        (0x0b201400, "add\t\tw0, w0, <undefined>"),
        // Decoded mnemonic with a not-yet-implemented operand kind (stub form).
        (0x19400800, "ldiapp\t\tw0, w0, :RCPC3_ADDR_OPT_POSTIND:"),
    ];

    // Encodings that must not decode to any instruction.
    const UNDEFINED: &[u32] = &[0x11800000, 0xffffffff];

    let mut fails = Vec::new();
    for &(insn, want) in CASES {
        match text(0, insn) {
            Some(got) if got == want => {}
            Some(got) => fails.push(format!("{insn:#010x}: got {got:?}, want {want:?}")),
            None => fails.push(format!("{insn:#010x}: decoded to None, want {want:?}")),
        }
    }
    for &insn in UNDEFINED {
        if let Some(got) = text(0, insn) {
            fails.push(format!("{insn:#010x}: want undefined, got {got:?}"));
        }
    }
    assert!(fails.is_empty(), "smoke regressions:\n{}", fails.join("\n"));
}

#[test]
fn id_maps_to_definition() {
    // The id indexes the definition table and is matchable via InsnId.
    let op = decoder::decode(0x11000000).expect("add decodes");
    assert_eq!(op.definition().mnemonic, "add");
    assert!(format!("{:?}", op.id()).starts_with("ADD"));

    // Distinct encodings sharing a mnemonic get distinct ids.
    let add_imm = decoder::decode(0x11000000).unwrap().id();
    let add_shift = decoder::decode(0x0b000000).unwrap().id();
    assert_ne!(add_imm, add_shift);
}

#[test]
#[ignore = "exhaustive; needs disarm64_test_data; run with --release -- --ignored"]
fn corpus() {
    let root = std::env::var("DISARM64_TEST_DATA").unwrap_or_else(|_| {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../disarm64_test_data").to_string()
    });
    let classes_dir = std::path::Path::new(&root).join("test/classes");
    if !classes_dir.is_dir() {
        eprintln!(
            "skipping corpus: {} not found (set DISARM64_TEST_DATA)",
            classes_dir.display()
        );
        return;
    }

    let only: Option<std::collections::HashSet<String>> = std::env::var("DISARM64_TEST_ONLY")
        .ok()
        .map(|v| v.split(',').map(|s| s.trim().to_string()).collect());
    let max: usize = std::env::var("DISARM64_TEST_MAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let mut dirs: Vec<_> = std::fs::read_dir(&classes_dir)
        .expect("read classes dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();

    let mut checked_classes = 0usize;
    let mut checked_insns = 0u64;
    let mut failures: Vec<String> = Vec::new();

    for dir in dirs {
        let name = dir.file_name().unwrap().to_string_lossy().to_string();
        if let Some(only) = &only {
            if !only.contains(&name) {
                continue;
            }
        }
        let bin = dir.join(format!("{name}.bin"));
        let lst = dir.join(format!("{name}-disarm64.lst"));
        if !bin.exists() || !lst.exists() {
            continue;
        }

        let data = std::fs::read(&bin).expect("read bin");
        let recorded = std::fs::read_to_string(&lst).expect("read lst");

        let mut class_fails = 0usize;
        let mut i = 0usize;
        for line in recorded.lines() {
            if line.starts_with("//") || line.is_empty() {
                continue;
            }
            if max != 0 && i >= max {
                break;
            }
            let Some(bytes) = data.get(i * 4..i * 4 + 4) else {
                failures.push(format!(
                    "{name}: listing has more lines than the binary has instructions (at {i})"
                ));
                break;
            };
            let insn = u32::from_le_bytes(bytes.try_into().unwrap());
            let got = cli_line((i * 4) as u64, insn);
            if got != line {
                if class_fails < 5 {
                    failures.push(format!(
                        "{name}[{i}] {insn:#010x}:\n    got: {got:?}\n    want: {line:?}"
                    ));
                }
                class_fails += 1;
            }
            i += 1;
        }
        checked_classes += 1;
        checked_insns += i as u64;
        if class_fails > 5 {
            failures.push(format!(
                "{name}: ... and {} more mismatches",
                class_fails - 5
            ));
        }
    }

    eprintln!("corpus: checked {checked_insns} encodings across {checked_classes} classes");
    assert!(
        failures.is_empty(),
        "corpus regressions ({} classes affected):\n{}",
        failures.len(),
        failures.join("\n")
    );
}
