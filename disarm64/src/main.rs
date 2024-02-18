use disarm64_defn::defn::InsnOpcode;
use clap::Parser;
use clap::Subcommand;
use clap_num::maybe_hex;
use disarm64_defn::defn::InsnOpcode;
use std::path::PathBuf;

mod decoder;
mod format_insn;

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum Command {
    /// Instructions to decode (hex 32-bit).
    Insn {
        /// Instructions delimited by commas.
        #[clap(value_parser = maybe_hex::<u32>, value_delimiter = ',', num_args = 1..)]
        dwords: Vec<u32>,
    },
    /// Flat binary file with instructions to decode.
    Bin {
        /// Path to the binary file.
        file: PathBuf,
        /// Offset in the file to start decoding.
        #[clap(short, long, value_parser = maybe_hex::<u64>, default_value = "0")]
        offset: u64,
        /// Number of instructions to decode.
        #[clap(short, long, value_parser = maybe_hex::<u64>)]
        count: Option<u64>,
    },
    /// ELF file with instructions to decode.
    Elf { file: PathBuf },
    /// Test file
    Test { file: PathBuf },
}

#[derive(Parser, Debug)]
/// This tool decodes instructions of the ARM64 architecture.
struct CommandLine {
    /// Instructions to decode (hex 32-bit).
    #[clap(subcommand)]
    command: Command,
    /// Log level/verbosity; repeat (-v, -vv, ...) to increase the verbosity.
    #[clap(short, action = clap::ArgAction::Count)]
    verbosity: u8,
}

fn init_logging(opt: &CommandLine) {
    // Maybe:
    // /// Log level.
    // #[clap(long, default_value = "info")]
    // log: log::LevelFilter,

    // From the env variable:
    // env_logger::Builder::from_env().init();

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .filter(
            None,
            match opt.verbosity {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            },
        )
        .init();
}

fn main() -> anyhow::Result<()> {
    let opt = CommandLine::parse();

    init_logging(&opt);

    match &opt.command {
        Command::Insn { dwords } => decode_insn_list(dwords.as_slice()),
        Command::Bin {
            file,
            offset,
            count,
        } => decode_bin(file.to_path_buf(), *offset, count.unwrap_or(!0)),
        Command::Elf { file } => decode_elf(file.to_path_buf()),
        Command::Test { file } => decode_test(file.to_path_buf()),
    }
}

fn decode_insn(insn: u32) -> anyhow::Result<()> {
    log::debug!("Decoding {insn:#08x}");
    if let Some(opcode) = decoder::decode(insn) {
        log::debug!("Decoded instruction: {:08x?}", opcode);
        log::info!("{insn:08x}: {opcode}");
    } else {
        log::warn!("{insn:08x}: ????");
    }
    Ok(())
}

fn decode_insn_list(dwords: &[u32]) -> anyhow::Result<()> {
    log::info!("Decoding instructions: {:08x?}", dwords);
    for insn in dwords {
        decode_insn(*insn)?;
    }
    Ok(())
}

fn decode_bin(file: PathBuf, offset: u64, count: u64) -> anyhow::Result<()> {
    log::info!("Decoding binary file {file:?} at offset {offset:#x}");
    let data = std::fs::read(file)?;
    let data = &data[offset as usize..];

    let mut pos = 0;
    let mut decoded = 0;
    while pos + 4 <= data.len() && decoded < count {
        let insn = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);

        let opcode = decoder::decode(insn);
        let current_offset = offset + pos as u64;
        if let Some(opcode) = opcode {
            log::debug!("Decoded instruction: {:08x?}", opcode);
            log::debug!("{insn:#08x}: {:08x?}", opcode.definition());

            log::info!("{current_offset:#08x}: {opcode}\t\t\t// {insn:08x}");
        } else {
            log::warn!("{current_offset:#08x}: ????\t\t\t// {insn:08x}");
        }

        pos += 4;
        decoded += 1;
    }
    Ok(())
}

fn decode_elf(file: PathBuf) -> anyhow::Result<()> {
    log::info!("Decoding ELF file {file:?}");
    let data = std::fs::read(file)?;
    let elf = goblin::elf::Elf::parse(&data)?;
    for section in &elf.section_headers {
        if section.sh_type == goblin::elf::section_header::SHT_PROGBITS {
            let data = &data[section.sh_offset as usize..][..section.sh_size as usize];
            let mut offset = 0;
            while offset + 4 <= data.len() {
                let insn = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);

                let opcode = decoder::decode(insn);
                if let Some(opcode) = opcode {
                    log::debug!("Decoded instruction: {:08x?}", opcode);
                    log::debug!("{insn:#08x}: {:08x?}", opcode.definition());

                    log::info!("{offset:#08x}: {opcode}\t\t\t// {insn:08x}");
                } else {
                    log::warn!("{offset:#08x}: ????\t\t\t// {insn:08x}");
                }

                offset += 4;
            }
        }
    }
    Ok(())
}

fn decode_test(file: PathBuf) -> anyhow::Result<()> {
    log::info!("Decoding test file {file:?}");
    let mut all_code = Vec::new();

    let data = std::fs::read_to_string(file)?;
    let lines = data.lines();
    for (i, line) in lines.enumerate() {
        let line = line.trim();
        if line.starts_with("//") {
            continue;
        }
        if line.is_empty() {
            continue;
        }

        log::info!("Decoding line {i:05}: {line}");

        // Split the line on spaces and get the first two items
        let mut split = line.split_whitespace();
        let insn_test = split.next().unwrap();
        let mnemonic_test = split.next().unwrap();

        let insn = u32::from_str_radix(insn_test, 16)?;
        all_code.extend(insn.to_le_bytes().iter());

        if let Some(opcode) = crate::decoder::decode(insn) {
            let mnemonic = opcode.definition().mnemonic;
            if mnemonic == mnemonic_test {
                log::info!("Decoded instruction: {:08x?}", opcode);
                log::info!("{insn_test}: {opcode}");
            } else {
                log::warn!("Decoded instruction: {:08x?}", opcode);
                log::warn!("{insn_test}: {opcode} (expected {mnemonic_test})");
            }
        } else {
            log::warn!("{insn_test}: ????");
        }
    }

    std::fs::write("test.bin", all_code.as_slice())?;

    Ok(())
}
