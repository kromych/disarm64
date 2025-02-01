# Instruction decoder generator

## About this project

This project contains a library and a tooling for generating instruction decoders
based on a decision tree from some standard description of the ISA. The objective of
efficient and precise instruction decoding is a pretty common one for disassemblers,
debuggers, emulators, VMMs, and the other tools alike.

The more instructions are there to decode, the more laborious and error-prone the task
becomes. Thus one might set their eyes on the idea to _generate_ the decoder from a
machine-readable description.

The ISA description is read from a JSON file (there is an example in the repo:
[aarch64.json](./aarch64.json), more than `3,000` instructions to play with), and
the algorithms assume a fixed length 32-bit encoding. The file is produced by the tools
from the [opcodes-lab](https://github.com/kromych/opcodes-lab) repository. The generated
decoder can be driven by conditionals statements (the default), the DFS table, or the
BFS table. There are trade-offs between size and speed, one can find what's best for them.

The default options deliver at least 250 MiB/sec of the decoding speed on a laptop that
is used for development.

After the instruction is recognized (decoded), one might need to format it as some string.
Adding instruction formatting resembling what disassemblers use is in progress, expected
in version `0.2.0`. The detailed progress is summarized [here](https://github.com/kromych/disarm64_test_data).
The high-level status is that the formatting matching LLVM and binutils works for
the instructions of the following groups:

- [x] Arithmetic
- [x] Data processing (some aliases might not be used just yet, the base instruction form is used)
- [x] Logical
- [x] Bit manipulation (some aliases might not be used just yet, the base instruction form is used)
- [x] Branches (uncoditional, conditional, test bit & branch)
- [x] Conditionals
- [x] Exceptions
- [x] Load/store
- [x] Moves
- [x] PC-relative
- [x] System instructions (hints, system registers)
- [x] Floating point
- [ ] SIMD, SVE
- [ ] SME

The current effort is directed at providing the ability to perform
structural matching against the operands of the instruction.

To install the tools:

```sh
cargo install disarm64-cli
```

To use as a library:

```sh
cargo add disarm64
cargo add disarm64_defn
```

and somewhere in your cool code could start with a snippet like this

```rust
use disarm64::decoder;
use disarm64_defn::defn::InsnOpcode;

fn neato() {
    let insn = decoder::decode(0x11000000).unwrap();

    println!("Instruction: {insn:?}");
    println!("Formatted: {insn}");
    println!("Definition: {:?}", insn.definition());
}

```

By default, the library is built to support the full known instruction set. There are also
few subsets available as the crate features to save on the compilation time and the size of
the code:

```toml
# Other features are `exception` and `system` 
disarm64 = { version = "0.1", default_features = false, features = ["load_store"] }
```

## Using the tools

### The decoder

The decoder can decode ARM64 instructions from the command line, a flat binary file,
an ELF file, a PE file, or a Mach-O file:

```sh
disarm64 --help
```

```txt
Usage: disarm64 [OPTIONS] <COMMAND>

Commands:
  insn    Instructions to decode (hex 32-bit), can specify multiple instructions separated by commas
  bin     Flat binary file with instructions to decode, can specify offset and count
  elf     ELF file with instructions to decode
  mach-o  Mach-O file with instructions to decode
  pe      PE file with instructions to decode
  help    Print this message or the help of the given subcommand(s)

Options:
  -v...       Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity
  --benchmark <BENCHMARK>  Benchmark mode: measure the time to decode the instructions.
                           This is not a synthetic benchmark, it provides an estimate
                           of the real-world performance
                           [possible values: decode, format]
  -h, --help  Print help
```

### The generator

This tool generates an instruction decoder from a JSON description of the ISA.

```sh
disarm64_gen --help
```

```txt
Usage: disarm64_gen [OPTIONS] <DESCRIPTION_JSON>

Arguments:
  <DESCRIPTION_JSON>
          A JSON file with the description of the instruction set architecture

Options:
  -a, --algo <ALGO>
          Decoder algorithm style, defaults to conditionals

          Possible values:
          - cond: Conditionals
          - dfs:  DFS table-driven
          - bfs:  BFS table-driven

  -f, --feature-sets <FEATURE_SETS>...
          Include filter for feature sets, e.g. "v8,simd". Case-insensitive, ignored if not provided

  -c, --insn-class <INSN_CLASS>...
          Include filter for instruction classes, e.g. "addsub_imm,ldst_pos,exception". Case-insensitive, ignored if not provided

  -m, --mnemonic <MNEMONIC>...
          Include filter for mnemonics, e.g. "adc,ldp". Case-insensitive, ignored if not provided

  -g, --graphviz <GRAPHVIZ>
          Output the decision tree to a Graphviz DOT file

  -r, --rs-file <RS_FILE>
          Generate the decoder implemented in Rust

  -t, --test-bin <TEST_BIN>
          Generate a test binary

      --test-bin-size-limit <TEST_BIN_SIZE_LIMIT>
          The size limit of the generated test binary, the default is 64MB
          
          [default: 67108864]

      --test-encodings-limit <TEST_ENCODINGS_LIMIT>
          The number of test encodings to generate for each instruction, the default is 0x10_000
          
          [default: 65536]

  -v...
          Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity

  -h, --help
          Print help (see a summary with '-h')
```

### Instruction classes and feature sets

To learn about the classes and feature sets available in the description of the ISA,
please run

```sh
disarm64_gen ./aarch64.json -c -
```

or

```sh
disarm64_gen ./aarch64.json -f -
```

## Examples

### Generating decoder implemented in Rust

For the entire known instruction set:

```sh
disarm64_gen ./aarch64.json -r decoder.rs
```

If only a subset of the whole instruction set needs to de decoded, use the filter(s)
appropriately. For example, to generate a decoder for the V8 load/store instructions, do:

```sh
disarm64_gen ./aarch64.json -c ldst_imm10,ldst_imm9,ldst_pos,ldst_regoff,ldst_unpriv,ldst_unscaled,ldstexcl,ldstnapair_offs,ldstpair_indexed,ldstpair_off,loadlit -f v8 -r decoder.rs
```

### Using the decoder

To decode instructions passed on the command line:

```sh
disarm64 -v insn 0x1a000001,0xa,0xa,0xa,0xa
```

```log
[DEBUG] Decoded instruction: ADDSUB_CARRY(ADC_Rd_Rn_Rm(ADC_Rd_Rn_Rm { rd: 00000001, rn: 00000000, rm: 00000000 }))
[DEBUG] 0x1a000001: Insn { mnemonic: "adc", aliases: [], opcode: 1a000000, mask: 7fe0fc00, class: ADDSUB_CARRY, feature_set: V8, operands: [InsnOperand { kind: Rd, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rd, lsb: 00000000, width: 00000005 }] }, InsnOperand { kind: Rn, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rn, lsb: 00000005, width: 00000005 }] }, InsnOperand { kind: Rm, class: INT_REG, qualifiers: [W, X], bit_fields: [BitfieldSpec { bitfield: Rm, lsb: 00000010, width: 00000005 }] }], flags: InsnFlags(HAS_SF_FIELD) }
[INFO ] 0x000000: 1a000001      adc     w1, w0, w0
[DEBUG] Decoded instruction: EXCEPTION(UDF_UNDEFINED(UDF_UNDEFINED { imm16_0: 0000000a }))
[DEBUG] 0x00000a: Insn { mnemonic: "udf", aliases: [], opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }], flags: InsnFlags(0x0) }
[INFO ] 0x000004: 0000000a      udf     #0xa
[DEBUG] Decoded instruction: EXCEPTION(UDF_UNDEFINED(UDF_UNDEFINED { imm16_0: 0000000a }))
[DEBUG] 0x00000a: Insn { mnemonic: "udf", aliases: [], opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }], flags: InsnFlags(0x0) }
[INFO ] 0x000008: 0000000a      udf     #0xa
[DEBUG] Decoded instruction: EXCEPTION(UDF_UNDEFINED(UDF_UNDEFINED { imm16_0: 0000000a }))
[DEBUG] 0x00000a: Insn { mnemonic: "udf", aliases: [], opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }], flags: InsnFlags(0x0) }
[INFO ] 0x00000c: 0000000a      udf     #0xa
[DEBUG] Decoded instruction: EXCEPTION(UDF_UNDEFINED(UDF_UNDEFINED { imm16_0: 0000000a }))
[DEBUG] 0x00000a: Insn { mnemonic: "udf", aliases: [], opcode: 00000000, mask: ffff0000, class: EXCEPTION, feature_set: V8, operands: [InsnOperand { kind: UNDEFINED, class: IMMEDIATE, qualifiers: [], bit_fields: [BitfieldSpec { bitfield: imm16_0, lsb: 00000000, width: 00000010 }] }], flags: InsnFlags(0x0) }
[INFO ] 0x000010: 0000000a      udf     #0xa
```

### Visualizing the decision trees

```sh
disarm64_gen ./aarch64.json -c exception -g dt-exception.dot
disarm64_gen ./aarch64.json -f v8 -g dt-v8.dot
disarm64_gen ./aarch64.json -c ic_system -g dt-system.dot
disarm64_gen ./aarch64.json -c ldst_imm10,ldst_imm9,ldst_pos,ldst_regoff,ldst_unpriv,ldst_unscaled,ldstexcl,ldstnapair_offs,ldstpair_indexed,ldstpair_off,loadlit -f v8 -g dt-ldst.dot
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

- exception instructions: [dt-exception.png](https://github.com/kromych/disarm64/blob/master/img/dt-exception.png)
- V8 instructions (no SIMD, no aliases): [dt-v8.png](https://github.com/kromych/disarm64/blob/master/img/dt-v8.png)
- system instructions: [dt-system.png](https://github.com/kromych/disarm64/blob/master/img/dt-system.png)
- V8 load and store instructions: [dt-ldst.png](https://github.com/kromych/disarm64/blob/master/img/dt-ldst.png)

### Debug output

```sh
disarm64_gen ./aarch64.json -c ldst_pos -m ldr -v
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

## Testing

For the test collateral and disassembly delta's between disarm64 & LLVM and diasrm64 & binutils,
please refer to [disarm64_test_data](https://github.com/kromych/disarm64_test_data).

## Reference materials for the ARM A64 ISA

- [HTML](https://developer.arm.com/documentation/ddi0602)
- [PDF](https://developer.arm.com/documentation/ddi0487)
- [Expolation Tools](https://developer.arm.com/downloads/-/exploration-tools)

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
- [Radare2](https://github.com/radareorg/radare2)
- [Rizin](https://rizin.re/)
- [Binary Ninja ARM64 plugin](https://github.com/Vector35/arch-arm64)

Those mentioned have broader scope, some offer bindings in various languages.

Not a library/API-centric, yet the one and only

- [Ghidra by NSA](https://github.com/NationalSecurityAgency/ghidra)

Although only x86_64 targeted, nonetheless an incredible one:

- [Blinkenlights](https://github.com/jart/blink)

## Some papers on this topic and the closely connected ones

- C.S. Collberg. "Reverse interpretation + mutation analysis = automatic retargeting",
  Proc. of the ACM SIG- PLAN 1997 Conference on Programming Language Design and Implementation", 1997, pp. 57–70.
- C.S. Collberg. "Automatic derivation of compiler machine descriptions",
  ACM Trans. Program. Lang. Syst., 2002, vol. 24, no. 4, pp. 369–408.
- W.C. Hsieh, D.R. Engler, G. Back. "Reverse-engineering instruction encodings",
  Proc. of the General Track: 2002 USENIX Annual Technical Conference, 2001, pp. 133–145.
- R. Krishna, T. Austin. "Efficient Software Decoder Design",
  IEEE Computer Society Technical Committee on Computer Architecture Newsletter, 2001.
- H. Theiling. "Generating decision trees for decoding binaries",
  Proc. of the ACM SIGPLAN Workshop on Languages, Compilers and Tools for Embedded Systems, 2001, pp. 112–120.
- W. Qin, S. Malik. "Automated synthesis of efficient binary decoders for retargetable software toolkits",
  Proc. of the 40th Annual Design Automation Conference, 2003, pp. 764–769.

## Legal

All trademarks are property of their respective owners. The content of this project is available under the terms
of the [MIT License](./LICENSE).
