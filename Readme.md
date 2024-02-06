# Instruction decoder tree generator

## About this project

This project contains tooling for generating instruction decoders based on a decision
tree from some standard description of ISA. The objective of efficient and precise
instruction decoding is a pretty common one for disassemblers, debuggers and emulators,
and the other tools alike.

The more instructions are there to decode, the more laborious and error-prone the task
becomes. Thus one might set their eyes on the idea to _generate_ the decoder from a
machine-readable description.

The ISA description is read from a JSON file (there is an example in the repo:
[aarch64.json](./aarch64.json), more than `3,000` instructions to play with), and
the algorithms assume a fixed length 32-bit encoding.

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
  -v...                                 Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity
  -h, --help                            Print help

```

### Instruction classes and feature sets

To learn about the classes and feature sets available in the description of the ISA,
please run

```sh
cargo run -- ./aarch64.json -c -
```

or

```sh
cargo run -- ./aarch64.json -f -
```

## Examples

```sh
cargo run -- ./aarch64.json -c ldst_pos -m ldr -v
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
