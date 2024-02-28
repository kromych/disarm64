use clap::Parser;
use clap::Subcommand;
use clap_num::maybe_hex;
use disarm64::decoder;
use disarm64::format_insn;
use disarm64_defn::defn::InsnOpcode;
use memmap2::Mmap;
use std::fs::File;
use std::io::BufReader;
use std::io::IsTerminal;
use std::io::Read;
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
enum Command {
    /// Instructions to decode (hex 32-bit), can specify multiple instructions separated by commas.
    Insn {
        /// Instructions delimited by commas.
        #[clap(value_parser = maybe_hex::<u32>, value_delimiter = ',', num_args = 1..)]
        dwords: Vec<u32>,
    },
    /// Flat binary file with instructions to decode, can specify offset and count.
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
    /// Mach-O file with instructions to decode.
    MachO { file: PathBuf },
    /// PE file with instructions to decode.
    Pe { file: PathBuf },
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

    let mut builder = env_logger::builder();
    let mut builder = builder
        .target(env_logger::Target::Stdout)
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
        );

    if !std::io::stdout().is_terminal() {
        builder = builder
            .write_style(env_logger::WriteStyle::Never)
            .format_level(false);
    }
    builder.init();
}

fn main() -> anyhow::Result<()> {
    let opt = CommandLine::parse();

    init_logging(&opt);

    let decode_file =
        |file: &PathBuf, reader: fn(data: &[u8]) -> anyhow::Result<()>| -> anyhow::Result<()> {
            let file = File::open(file)?;
            let mmap = unsafe { Mmap::map(&file)? };

            reader(&mmap)?;
            Ok(())
        };

    match &opt.command {
        Command::Insn { dwords } => decode_insn_list(dwords.as_slice()),
        Command::Bin {
            file,
            offset,
            count,
        } => {
            log::info!("// Decoding binary file: {file:?}, offset {offset:#x}, count {count:#x?}");

            let file = File::open(file)?;
            let mmap = unsafe { Mmap::map(&file)? };
            let data = &mmap[*offset as usize..];
            let data = if let Some(count) = count {
                &data[..*count as usize]
            } else {
                data
            };
            decode_bin(data, *offset)
        }
        Command::Elf { file } => {
            log::info!("// Decoding ELF file: {file:?}");
            decode_file(file, decode_elf)
        }
        Command::MachO { file } => {
            log::info!("// Decoding Mach-O file: {file:?}");
            decode_file(file, decode_mach)
        }
        Command::Pe { file } => {
            log::info!("// Decoding PE file: {file:?}");
            decode_file(file, decode_pe)
        }
    }
}

fn print_insn(pc: u64, insn: u32, buffer: &mut String) -> anyhow::Result<()> {
    let opcode = decoder::decode(insn);
    if let Some(opcode) = opcode {
        log::debug!("Decoded instruction: {:08x?}", opcode);
        log::debug!("{insn:#08x}: {:08x?}", opcode.definition());

        // For the PC-relative instructions, we need to know the current offset
        // to calculate the target address. If that is not relevant, you can use the
        // Display implementation and remove the buffer.
        buffer.clear();
        format_insn::format_insn_pc(pc, buffer, &opcode)?;

        log::info!("{pc:#08x}: {insn:08x}\t{buffer}");
    } else {
        log::warn!("{pc:#08x}: {insn:08x}\t.inst\t{insn:#08x} // undefined");
    }

    Ok(())
}

fn process_bytes(data: &[u8], offset: u64, buffer: &mut String) -> anyhow::Result<()> {
    let mut pos = 0;
    let mut reader = BufReader::new(data);
    while pos + 4 <= data.len() {
        let mut insn = [0; 4];
        reader.read_exact(&mut insn)?;
        let insn = u32::from_le_bytes(insn);
        let current_offset = offset + pos as u64;

        print_insn(current_offset, insn, buffer)?;
        pos += 4;
    }
    Ok(())
}

fn decode_insn_list(dwords: &[u32]) -> anyhow::Result<()> {
    let mut buffer = String::new();
    for (i, insn) in dwords.iter().enumerate() {
        print_insn(i as u64 * 4, *insn, &mut buffer)?;
    }
    Ok(())
}

fn decode_bin(data: &[u8], offset: u64) -> anyhow::Result<()> {
    let mut buffer = String::new();
    process_bytes(data, offset, &mut buffer)
}

fn decode_elf(data: &[u8]) -> anyhow::Result<()> {
    let elf = goblin::elf::Elf::parse(data)?;
    if elf.header.e_machine != goblin::elf::header::EM_AARCH64 {
        log::error!("Not an ARM64 ELF file");
        return Err(anyhow::anyhow!("Not an ARM64 ELF file"));
    }

    let mut buffer = String::new();
    for section in &elf.section_headers {
        if section.sh_flags & goblin::elf::section_header::SHF_EXECINSTR as u64 == 0 {
            continue;
        }

        let section_name = elf
            .shdr_strtab
            .get_at(section.sh_name)
            .unwrap_or("<unknown>");
        log::info!(
            "// Decoding section {section_name:?} @ {:#x}",
            section.sh_addr
        );

        let data = &data[section.sh_offset as usize..][..section.sh_size as usize];
        process_bytes(data, section.sh_addr, &mut buffer)?;
    }

    Ok(())
}

fn decode_mach(data: &[u8]) -> anyhow::Result<()> {
    let mach = goblin::mach::Mach::parse(data)?;
    let mach = match mach {
        goblin::mach::Mach::Binary(macho) => {
            if macho.header.cputype != goblin::mach::cputype::CPU_TYPE_ARM64 {
                log::error!("Not an ARM64 Mach-O file");
                return Err(anyhow::anyhow!("Not an ARM64 Mach-O file"));
            }
            macho
        }
        goblin::mach::Mach::Fat(_) => {
            log::error!("Fat Mach-O files are not supported");
            return Err(anyhow::anyhow!("Fat Mach-O files are not supported"));
        }
    };

    let mut buffer = String::new();
    for segment in &mach.segments {
        if let Ok(sections) = segment.sections() {
            for (section, data) in sections {
                if section.flags & 0x80000000 != 0 {
                    log::info!(
                        "// Decoding section {:?} @ {:#x}",
                        section.name().unwrap_or("<unknwon>"),
                        section.addr
                    );

                    process_bytes(data, section.addr, &mut buffer)?;
                }
            }
        } else {
            log::warn!("Failed to read sections for segment {:?}", segment.name());
        }
    }
    Ok(())
}

fn decode_pe(data: &[u8]) -> anyhow::Result<()> {
    let pe = goblin::pe::PE::parse(data)?;
    if pe.header.coff_header.machine != goblin::pe::header::COFF_MACHINE_ARM64 {
        log::error!("Not an ARM64 PE file");
        return Err(anyhow::anyhow!("Not an ARM64 PE file"));
    }

    let mut buffer = String::new();
    for section in &pe.sections {
        if section.characteristics & 0x20000000 != 0 {
            log::info!(
                "// Decoding section {:?} @ {:#x}",
                std::ffi::CStr::from_bytes_until_nul(&section.name).unwrap_or(
                    std::ffi::CStr::from_bytes_until_nul(b"<unknown>\0").expect("a valid C string")
                ),
                section.virtual_address
            );

            let data =
                &data[section.pointer_to_raw_data as usize..][..section.size_of_raw_data as usize];
            process_bytes(data, section.virtual_address as u64, &mut buffer)?;
        }
    }
    Ok(())
}
