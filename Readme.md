# Instruction decoder generator

## About this project

This project contains tooling for generating instruction decoders based on a decision
tree from some standard description of ISA. The objective of efficient and precise
instruction decoding is a pretty common one for disassemblers, debuggers, emulators,
VMMs, and the other tools alike.

The more instructions are there to decode, the more laborious and error-prone the task
becomes. Thus one might set their eyes on the idea to _generate_ the decoder from a
machine-readable description.

The ISA description is read from a JSON file (there is an example in the repo:
[aarch64.json](./aarch64.json), more than `3,000` instructions to play with), and
the algorithms assume a fixed length 32-bit encoding. The file is produced by the tools
in the [opcodes-lab](https://github.com/kromych/opcodes-lab) repository.

## Using the generator

### Available options

```sh
cargo run -- --help
```

```txt
This tool generates an instruction decoder from a JSON description of the ISA

Usage: gen_insn_dec [OPTIONS] <DESCRIPTION_JSON>

Arguments:
  <DESCRIPTION_JSON>  A JSON file with the description of the instruction set architecture

Options:
  -f, --feature-sets <FEATURE_SETS>...  Include filter for feature sets, e.g. "v8,simd". Case-insensitive, ignored if not provided
  -c, --insn-class <INSN_CLASS>...      Include filter for instruction classes, e.g. "addsub_imm,ldst_pos,exception". Case-insensitive, ignored if not provided
  -m, --mnemonic <MNEMONIC>...          Include filter for mnemonics, e.g. "adc,ldp". Case-insensitive, ignored if not provided
  -g, --graphviz <GRAPHVIZ>             Output the decision tree to a Graphviz DOT file
  -r, --rs-file <RS_FILE>               Generate the decoder implemented in Rust
  -v...                                 Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity
  -h, --help                            Print help

```

### Instruction classes and feature sets

To learn about the classes and feature sets available in the description of the ISA,
please run

```sh
cargo run -- --bin gen_decoder ./aarch64.json -c -
```

or

```sh
cargo run -- --bin gen_decoder ./aarch64.json -f -
```

## Examples

### Generating decoder implemented in Rust

For the entire known instruction set:

```sh
cargo run --release --bin gen_decoder -- ./aarch64.json -r decoder.rs
```

If only a subset of the whole instruction set needs to de decoded, use the filter(s)
appropriately. For example, to generate a decoder for the V8 load/store instructions, do:

```sh
cargo run -- --bin gen_decoder ./aarch64.json -c ldst_imm10,ldst_imm9,ldst_pos,ldst_regoff,ldst_unpriv,ldst_unscaled,ldstexcl,ldstnapair_offs,ldstpair_indexed,ldstpair_off,loadlit -f v8 -r decoder.rs
```

### Using the decoder

The decoder can decode instructions from the command line, a flat binary file or an ELF file:

```sh
cargo run --release --bin disarm64 -- --help

Usage: disarm64 [OPTIONS] <COMMAND>

Commands:
  insn  Instructions to decode (hex 32-bit)
  bin   Flat binary file with instructions to decode
  elf   ELF file with instructions to decode
  help  Print this message or the help of the given subcommand(s)

Options:
  -v...       Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity
  -h, --help  Print help
```

To decode instructions apssed on the command line:

```sh
cargo run --release --bin disarm64 -- insn 0x1a000001,0xa,0xa,0xa,0xa
```

```log
[INFO ] Decoding instructions: [1a000001, 0000000a, 0000000a, 0000000a, 0000000a]
[INFO ] 0x1a000001: Insn { mnemonic: "adc", opcode: 1a000000, mask: 7fe0fc00, class: ADDSUB_CARRY, feature_set: V8, operands: [InsnOperand { kind: Rd, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rd, lsb: 00000000, width: 00000005 }] }, InsnOperand { kind: Rn, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rn, lsb: 00000005, width: 00000005 }] }, InsnOperand { kind: Rm, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rm, lsb: 00000010, width: 00000005 }] }] }
[INFO ] 0x00000a: Insn { mnemonic: "udf", opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }] }
[INFO ] 0x00000a: Insn { mnemonic: "udf", opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }] }
[INFO ] 0x00000a: Insn { mnemonic: "udf", opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }] }
[INFO ] 0x00000a: Insn { mnemonic: "udf", opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }] }
```

### Visualizing the decision trees

```sh
cargo run -- --bin gen_decoder ./aarch64.json -c exception -g dt-exception.dot
cargo run -- --bin gen_decoder ./aarch64.json -f v8 -g dt-v8.dot
cargo run -- --bin gen_decoder ./aarch64.json -c ic_system -g dt-system.dot
cargo run -- --bin gen_decoder ./aarch64.json -c ldst_imm10,ldst_imm9,ldst_pos,ldst_regoff,ldst_unpriv,ldst_unscaled,ldstexcl,ldstnapair_offs,ldstpair_indexed,ldstpair_off,loadlit -f v8 -g dt-ldst.dot
```

and then (assuming [Graphviz](https://graphviz.org/) tools are installed):

```sh
dot -Tpng dt-exception.dot -o dt-exception.png
dot -Tpng dt-v8.dot -o dt-v8.png
dot -Tpng dt-system.dot -o dt-system.png
dot -Tpng dt-ldst.dot -o dt-ldst.png
```

to render the [`dot`](https://graphviz.org/doc/info/lang.html) file into a `png`
image. The numbers in the circles show the bit to check; in the rectangles, there
are instructions and opcodes to check against.

Examples (Aarch64):

- exception instructions: [dt-exception.png](./img/dt-exception.png)
- V8 instructions (no SIMD, no aliases): [dt-v8.png](./img/dt-v8.png)
- system instructions: [dt-system.png](./img/dt-system.png)
- V8 load and store instructions: [dt-ldst.png](./img/dt-ldst.png)

### Debug output

```sh
cargo run -- --bin gen_decoder ./aarch64.json -c ldst_pos -m ldr -v
```

```log
[INFO ] Loading "./aarch64.json"
[INFO ] Including instructions from all feature sets
[INFO ] Including instructions from classes {LDST_POS}
[INFO ] Including instructions with mnemonics {"ldr"}
[DEBUG] instruction Insn { mnemonic: "ldr", opcode: 3d400000, mask: 3f400000, class: LDST_POS, feature_set: V8, operands: {ADDR_UIMM12: InsnOperand { class: ADDRESS, qualifiers: [S_B], bit_fields: [BitfieldSpec { bitfield: Rn, lsb: 5, width: 5 }, BitfieldSpec { bitfield: imm12, lsb: a, width: c }] }, Ft: InsnOperand { class: FP_REG, qualifiers: [S_B], bit_fields: [BitfieldSpec { bitfield: Rt, lsb: 0, width: 5 }] }}, flags: InsnFlags(0x0), index: 37d }
[DEBUG] instruction Insn { mnemonic: "ldr", opcode: b9400000, mask: bfc00000, class: LDST_POS, feature_set: V8, operands: {Rt: InsnOperand { class: INT_REG, qualifiers: [W], bit_fields: [BitfieldSpec { bitfield: Rt, lsb: 0, width: 5 }] }, ADDR_UIMM12: InsnOperand { class: ADDRESS, qualifiers: [S_S], bit_fields: [BitfieldSpec { bitfield: Rn, lsb: 5, width: 5 }, BitfieldSpec { bitfield: imm12, lsb: a, width: c }] }}, flags: InsnFlags(HAS_ADVSIMV_GPRSIZE_IN_Q), index: 382 }
[DEBUG] Classes {LDST_POS}
[DEBUG] Feature sets {V8}
[INFO ] Processed 3323 instructions, skipped 200 aliases, 1 classes, 1 feature sets filtered out 3321 instructions
[INFO ] Loaded 2 instructions
[DEBUG] Building decision tree at depth 1
[DEBUG] mask: 3f400000
[DEBUG] decision bit: 22
[DEBUG] decision mask: 400000
[DEBUG] zero: 0, one: 2
[DEBUG] mask: 3f000000
[DEBUG] decision bit: 24
[DEBUG] decision mask: 1000000
[DEBUG] zero: 0, one: 2
[DEBUG] mask: 3e000000
[DEBUG] decision bit: 25
[DEBUG] decision mask: 2000000
[DEBUG] zero: 2, one: 0
[DEBUG] mask: 3c000000
[DEBUG] decision bit: 26
[DEBUG] decision mask: 4000000
[DEBUG] zero: 1, one: 1
[DEBUG] Building decision tree at depth 2
[DEBUG] One instruction at depth 1
[DEBUG] Building decision tree at depth 2
[DEBUG] One instruction at depth 1
[DEBUG] Decision tree built at depth 0
```

## Related art

This project doesn't have any claims to fame. It uses well-known algorithms and approaches
to generating instruction decoders and disassemblers with what seems to be few pretty
minor twists: reading the ISA description from a JSON file and producing a strongly-typed Rust
decoders with no pointers, unsafe blocks and memory allocations at all. Perhaps, you'll
enjoy the ability to generate a decoder for a part of the instruction set which makes
for a smaller code size, too.

Here are other projects touching on the topic of decoding the machine instructions:

- [Capstone & its LLVM TableGen fork](https://github.com/capstone-engine)
- [LLVM & TableGen](https://github.com/llvm/llvm-project)
- [Qemu - Quick emulator](https://github.com/qemu/qemu)
- [Unicorn](https://github.com/unicorn-engine/unicorn)
- [Binutils & libopcode](https://www.gnu.org/software/binutils/)
- [Binary Ninja ARM64 plugin](https://github.com/Vector35/arch-arm64)

Those mentioned have broader scope, some offer bindings in various languages.

Not a library/API-centric, yet the one and only

- [Ghidra by NSA](https://github.com/NationalSecurityAgency/ghidra)

Although only x86_64 targeted, nonetheless an incredible one:

- [Blinkenlights](https://github.com/jart/blink)
