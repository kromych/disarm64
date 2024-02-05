use clap::Parser;
use insn_def::description::Insn;
use insn_def::description::InsnClass;
use insn_def::description::InsnFeatureSet;
use insn_def::description::InsnFlags;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Parser, Debug)]
/// This tool generates an instruction decoder from a JSON description of the ISA.
struct CommandLine {
    /// A JSON file with the description of the instruction set architecture.
    description_json: PathBuf,
    /// Include filter for feature sets, e.g. "v8,simd".
    /// Case-insensitive, ignored if not provided.
    #[clap(short = 'f', long, value_delimiter = ',', num_args = 1..)]
    feature_sets: Option<Vec<InsnFeatureSet>>,
    /// Include filter for instruction classes, e.g. "addsub_imm,ldst_pos,exception".
    /// Case-insensitive, ignored if not provided.
    #[clap(short = 'c', long, value_delimiter = ',', num_args = 1..)]
    insn_class: Option<Vec<InsnClass>>,
    /// Include filter for mnemonics, e.g. "adc,ldp".
    /// Case-insensitive, ignored if not provided.
    #[clap(short = 'm', long, value_delimiter = ',', num_args = 1..)]
    mnemonic: Option<Vec<String>>,
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
        .format_timestamp_millis()
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

    let filter_feature_sets = HashSet::from_iter(opt.feature_sets.unwrap_or_default());
    let filter_insn_class = HashSet::from_iter(opt.insn_class.unwrap_or_default());
    let filter_mnemonic = HashSet::from_iter(opt.mnemonic.unwrap_or_default());
    parse_insn_description(
        opt.description_json,
        filter_feature_sets,
        filter_insn_class,
        filter_mnemonic,
    )?;

    Ok(())
}

fn parse_insn_description(
    path: PathBuf,
    feature_sets_filter: HashSet<InsnFeatureSet>,
    insn_class_filter: HashSet<InsnClass>,
    mnemonic_filter: HashSet<String>,
) -> anyhow::Result<Vec<Rc<Insn>>> {
    log::info!("Loading {path:?}");

    if !feature_sets_filter.is_empty() {
        log::info!("Including instructions from feature sets {feature_sets_filter:?}");
    } else {
        log::info!("Including instructions from all feature sets");
    }
    if !insn_class_filter.is_empty() {
        log::info!("Including instructions from classes {insn_class_filter:?}");
    } else {
        log::info!("Including instructions from all classes");
    }
    if !mnemonic_filter.is_empty() {
        log::info!("Including instructions with mnemonics {mnemonic_filter:?}");
    } else {
        log::info!("Including instructions with all mnemonics");
    }

    let data = std::fs::read_to_string(path)?;
    let insns = serde_json::from_str::<Vec<Insn>>(&data)?;
    let mut filtered_insns = Vec::new();

    let mut aliases = 0;
    let mut insn_classes = HashSet::new();
    let mut insn_feature_sets = HashSet::new();
    let mut i = 0;

    loop {
        if i >= insns.len() {
            break;
        }

        let insn = &insns[i];
        let opcode = insn.opcode;
        let mask = insn.mask;

        if insn.flags.contains(InsnFlags::IS_ALIAS) {
            log::trace!("skipping alias {insn:x?}");

            aliases += 1;
        } else if (feature_sets_filter.is_empty()
            || feature_sets_filter.contains(&insn.feature_set))
            && (insn_class_filter.is_empty() || insn_class_filter.contains(&insn.class))
            && (mnemonic_filter.is_empty() || mnemonic_filter.contains(&insn.mnemonic))
        {
            log::debug!("instruction {insn:x?}");

            // If opcode == opcode & mask, then additional_dont_care == 0 and opcode & !mask == 0
            let additional_dont_care = opcode ^ (mask & opcode); // == opcode & !mask
            if additional_dont_care != 0 {
                anyhow::bail!("Invalid mask for {insn:x?}, opcode & mask != mask, extra bits: {additional_dont_care:#b}");
            }

            if mask == 0 {
                anyhow::bail!("Empty mask for {insn:x?}");
            }

            insn_classes.insert(&insn.class);
            insn_feature_sets.insert(&insn.feature_set);

            filtered_insns.push(Rc::new(insn.clone()));
        } else {
            log::trace!("Skipping {insn:x?}");
        }

        i += 1;
    }

    log::debug!("Classes {insn_classes:?}");
    log::debug!("Feature sets {insn_feature_sets:?}");

    log::info!(
        "Processed {} instructions, skipped {aliases} aliases, {} classes, {} feature sets filtered out {} instructions",
        insns.len(),
        insn_classes.len(),
        insn_feature_sets.len(),
        insns.len() - filtered_insns.len()
    );

    log::info!("Loaded {} instructions", filtered_insns.len());

    Ok(filtered_insns)
}
