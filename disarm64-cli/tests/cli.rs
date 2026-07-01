//! End-to-end tests for the disarm64 command-line tool. These lock the CLI's
//! output format, which the library corpus does not cover.

use std::process::Command;

fn disarm64(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_disarm64"))
        .args(args)
        .output()
        .expect("run disarm64");
    assert!(
        output.status.success(),
        "disarm64 {args:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("stdout is utf8")
}

#[test]
fn insn_lists_decoded_instructions() {
    let out = disarm64(&["insn", "0x11000000,0x0b000000,0xd503201f,0xffffffff"]);
    assert_eq!(
        out,
        "0x000000: 11000000\tadd\t\tw0, w0, #0x0\n\
         0x000004: 0b000000\tadd\t\tw0, w0, w0\n\
         0x000008: d503201f\tnop\n\
         0x00000c: ffffffff\t.inst\t0xffffffff // undefined\n"
    );
}

#[test]
fn bin_decodes_a_flat_binary_with_absolute_pc_targets() {
    let path = std::env::temp_dir().join("disarm64_cli_bin_test.bin");
    let words: [u32; 4] = [0x11000000, 0x0b000000, 0xd503201f, 0x14000000];
    let mut bytes = Vec::new();
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    std::fs::write(&path, &bytes).expect("write fixture");

    let out = disarm64(&["bin", path.to_str().unwrap()]);
    // Skip the leading "// Decoding binary file: ..." line, which names the path.
    let body: Vec<&str> = out.lines().skip(1).collect();
    assert_eq!(
        body,
        [
            "0x000000: 11000000\tadd\t\tw0, w0, #0x0",
            "0x000004: 0b000000\tadd\t\tw0, w0, w0",
            "0x000008: d503201f\tnop",
            // The branch target is the absolute address (pc 0xc + offset 0).
            "0x00000c: 14000000\tb\t\t0xc",
        ]
    );

    std::fs::remove_file(&path).ok();
}
