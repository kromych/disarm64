use clap::Parser;
use clap_num::maybe_hex;

mod decoder;

#[derive(Parser, Debug)]
/// This tool decodes an instruction
struct CommandLine {
    /// An instruction to decode (hex 32-bit).
    #[clap(short, long, value_parser = maybe_hex::< u32 >)]
    insn: u32,
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

    let opcode = opt.insn;
    log::info!("Decoding {opcode:#08x}");
    let insn = decoder::decode(opcode);
    log::info!("Decoded instruction: {:x?}", insn);

    Ok(())
}
