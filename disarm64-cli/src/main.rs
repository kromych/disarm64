use clap::Parser;
use clap::Subcommand;
use clap_num::maybe_hex;
use disarm64::format_insn;
use disarm64_defn::defn::InsnOpcode;
use memmap2::Mmap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::IsTerminal;
use std::io::Read;
use std::path::PathBuf;

#[cfg(feature = "full")]
use disarm64::decoder;

#[cfg(feature = "exception")]
use disarm64::decoder_exception as decoder;
#[cfg(feature = "load_store")]
use disarm64::decoder_load_store as decoder;
#[cfg(feature = "system")]
use disarm64::decoder_system as decoder;

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
        #[clap(short, long, value_parser = maybe_hex::<usize>, default_value = "0")]
        offset: usize,
        /// Number of instructions to decode.
        #[clap(short, long, value_parser = maybe_hex::<usize>)]
        count: Option<usize>,
    },
    /// ELF file with instructions to decode.
    Elf { file: PathBuf },
    /// Mach-O file with instructions to decode.
    MachO { file: PathBuf },
    /// PE file with instructions to decode.
    Pe { file: PathBuf },
}

#[derive(Parser, Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
enum BenchmarkMode {
    Decode,
    Format,
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
    /// Benchmark mode: measure the time to decode the instructions. This is not
    /// a synthetic benchmark, it provides an estimate of the real-world
    /// performance.
    #[clap(long)]
    benchmark: Option<BenchmarkMode>,
}

#[derive(Debug, Clone, Copy, Default)]
struct ProcessingStats {
    elapsed: std::time::Duration,
    processed_size: usize,
}

impl ProcessingStats {
    fn add(&mut self, other: &Self) {
        self.elapsed += other.elapsed;
        self.processed_size += other.processed_size;
    }

    fn mib_per_second(&self) -> u64 {
        if self.elapsed.as_secs_f64() == 0.0 {
            return 0;
        }
        (self.processed_size as f64 / self.elapsed.as_secs_f64()).floor() as u64 >> 20
    }
}

fn init_logging(opt: &CommandLine) {
    // Maybe:
    // /// Log level.
    // #[clap(long, default_value = "info")]
    // log: log::LevelFilter,

    // From the env variable:
    // env_logger::Builder::from_env().init();

    if opt.benchmark.is_some() {
        eprintln!("Benchmark mode: logging disabled");
        return;
    }

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

    #[cfg(not(feature = "full"))]
    log::warn!("Decoder wasn't built with the support for the full instruction set");

    let decode_file = |file: &PathBuf,
                       reader: fn(
        data: &[u8],
        benchmark: Option<BenchmarkMode>,
    ) -> anyhow::Result<ProcessingStats>,
                       offset: usize,
                       count: Option<usize>|
     -> anyhow::Result<ProcessingStats> {
        if opt.benchmark.is_none() {
            let file = File::open(file)?;
            let mmap = unsafe { Mmap::map(&file)? };

            let data = &mmap[offset..];
            let data = if let Some(count) = count {
                &data[..count]
            } else {
                data
            };

            reader(data, opt.benchmark)
        } else {
            let data = fs::read(file)?;
            let data = &data[offset..];
            let data = if let Some(count) = count {
                &data[..count]
            } else {
                data
            };

            reader(data, opt.benchmark)
        }
    };

    let stats = match &opt.command {
        Command::Insn { dwords } => decode_insn_list(dwords.as_slice(), opt.benchmark),
        Command::Bin {
            file,
            offset,
            count,
        } => {
            log::info!("// Decoding binary file: {file:?}, offset {offset:#x}, count {count:#x?}");

            decode_file(file, decode_bin, *offset, *count)
        }
        Command::Elf { file } => {
            log::info!("// Decoding ELF file: {file:?}");
            decode_file(file, decode_elf, 0, None)
        }
        Command::MachO { file } => {
            log::info!("// Decoding Mach-O file: {file:?}");
            decode_file(file, decode_mach, 0, None)
        }
        Command::Pe { file } => {
            log::info!("// Decoding PE file: {file:?}");
            decode_file(file, decode_pe, 0, None)
        }
    }?;

    if let Some(benchmark) = opt.benchmark {
        eprintln!(
            "Benchmark mode {benchmark:?}. Processing has taken {elapsed:?} for {processed_size} bytes, {mib_per_second} MiB/s",
            elapsed = stats.elapsed,
            processed_size = stats.processed_size,
            mib_per_second = stats.mib_per_second()
        );
    }

    Ok(())
}

#[inline]
fn print_insn(
    pc: u64,
    insn: u32,
    buffer: &mut String,
    benchmark: Option<BenchmarkMode>,
) -> anyhow::Result<()> {
    let opcode = decoder::decode(insn);
    if benchmark == Some(BenchmarkMode::Decode) {
        return Ok(());
    }

    if let Some(opcode) = opcode {
        // For the PC-relative instructions, we need to know the current offset
        // to calculate the target address. If that is not relevant, you can use the
        // Display implementation and remove the buffer.
        buffer.clear();
        format_insn::format_insn_pc(pc, buffer, &opcode)?;

        if benchmark == Some(BenchmarkMode::Format) {
            return Ok(());
        }

        log::debug!("Decoded instruction: {:08x?}", opcode);
        log::debug!("{insn:#08x}: {:08x?}", opcode.definition());
        log::info!("{pc:#08x}: {insn:08x}\t{buffer}");
    } else {
        if benchmark == Some(BenchmarkMode::Format) {
            return Ok(());
        }

        log::warn!("{pc:#08x}: {insn:08x}\t.inst\t{insn:#08x} // undefined");
    }

    Ok(())
}

#[inline]
fn process_bytes(
    data: &[u8],
    offset: u64,
    buffer: &mut String,
    benchmark: Option<BenchmarkMode>,
) -> anyhow::Result<ProcessingStats> {
    let mut pos = 0;
    let mut reader = BufReader::new(data);
    let start = std::time::Instant::now();
    while pos + 4 <= data.len() {
        let mut insn = [0; 4];
        reader.read_exact(&mut insn)?;
        let insn = u32::from_le_bytes(insn);
        let current_offset = offset + pos as u64;

        print_insn(current_offset, insn, buffer, benchmark)?;
        pos += 4;
    }
    let elapsed = start.elapsed();

    Ok(ProcessingStats {
        elapsed,
        processed_size: pos,
    })
}

fn decode_insn_list(
    dwords: &[u32],
    benchmark: Option<BenchmarkMode>,
) -> anyhow::Result<ProcessingStats> {
    let mut buffer = String::new();
    let start = std::time::Instant::now();
    for (i, insn) in dwords.iter().enumerate() {
        print_insn(i as u64 * 4, *insn, &mut buffer, benchmark)?;
    }
    let elapsed = start.elapsed();

    Ok(ProcessingStats {
        elapsed,
        processed_size: dwords.len() * 4,
    })
}

fn decode_bin(data: &[u8], benchmark: Option<BenchmarkMode>) -> anyhow::Result<ProcessingStats> {
    let mut buffer = String::new();
    let stats = process_bytes(data, 0, &mut buffer, benchmark)?;

    Ok(stats)
}

fn decode_elf(data: &[u8], benchmark: Option<BenchmarkMode>) -> anyhow::Result<ProcessingStats> {
    let mut proc_stats = ProcessingStats::default();

    let elf = goblin::elf::Elf::parse(data)?;
    if elf.header.e_machine != goblin::elf::header::EM_AARCH64
        || !elf
            .header
            .endianness()
            .expect("endiannes is known")
            .is_little()
    {
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
        let stats = process_bytes(data, section.sh_addr, &mut buffer, benchmark)?;
        proc_stats.add(&stats);
    }

    Ok(proc_stats)
}

fn decode_mach(data: &[u8], benchmark: Option<BenchmarkMode>) -> anyhow::Result<ProcessingStats> {
    let mut proc_stats = ProcessingStats::default();

    let mach = goblin::mach::Mach::parse(data)?;
    let mach = match mach {
        goblin::mach::Mach::Binary(macho) => {
            if macho.header.cputype != goblin::mach::cputype::CPU_TYPE_ARM64 || !macho.little_endian
            {
                log::error!("Not an ARM64 little-endian Mach-O file");
                return Err(anyhow::anyhow!("Not an ARM64 Mach-O little-endian file"));
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

                    let stats = process_bytes(data, section.addr, &mut buffer, benchmark)?;
                    proc_stats.add(&stats);
                }
            }
        } else {
            log::warn!("Failed to read sections for segment {:?}", segment.name());
        }
    }

    Ok(proc_stats)
}

fn decode_pe(data: &[u8], benchmark: Option<BenchmarkMode>) -> anyhow::Result<ProcessingStats> {
    let pe = goblin::pe::PE::parse(data)?;
    if pe.header.coff_header.machine != goblin::pe::header::COFF_MACHINE_ARM64 {
        log::error!("Not an ARM64 PE file");
        return Err(anyhow::anyhow!("Not an ARM64 PE file"));
    }

    let mut proc_stats = ProcessingStats::default();
    let mut buffer = String::new();
    for section in &pe.sections {
        if section.characteristics & 0x20000000 != 0 {
            let vbase = pe.image_base + section.virtual_address as usize;
            log::info!(
                "// Decoding section {:?} @ {vbase:#x}",
                std::ffi::CStr::from_bytes_until_nul(&section.name).unwrap_or(
                    std::ffi::CStr::from_bytes_until_nul(b"<unknown>\0").expect("a valid C string")
                ),
            );

            let data =
                &data[section.pointer_to_raw_data as usize..][..section.size_of_raw_data as usize];
            let stats = process_bytes(data, vbase as u64, &mut buffer, benchmark)?;
            proc_stats.add(&stats);
        }
    }

    Ok(proc_stats)
}
