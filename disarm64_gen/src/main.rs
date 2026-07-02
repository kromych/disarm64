use clap::Parser;
use disarm64_defn::deser::Insn;
use disarm64_defn::InsnClass;
use disarm64_defn::InsnFeatureSet;
use disarm64_defn::InsnFlags;
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;

use crate::generate_graphviz_dot::decistion_tree_to_graphviz_dot;
use crate::generate_rust::decision_tree_to_rust;
use decision_tree::build_decision_tree;
use decision_tree::DecisionTreeIndexing;

mod decision_tree;
mod generate_graphviz_dot;
mod generate_registers;
mod generate_rust;
mod generate_test_bin;

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
enum DecoderAlgo {
    /// Conditionals
    Cond,
    /// DFS table-driven
    Dfs,
    /// BFS table-driven
    Bfs,
}

#[derive(Parser, Debug)]
/// This tool generates an instruction decoder from a JSON description of the ISA.
struct CommandLine {
    /// A JSON file with the description of the instruction set architecture.
    description_json: PathBuf,
    /// Decoder algorithm style, defaults to conditionals.
    #[clap(short = 'a', long)]
    algo: Option<DecoderAlgo>,
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
    /// Output the decision tree to a Graphviz DOT file.
    #[clap(short, long)]
    graphviz: Option<PathBuf>,
    /// Generate the decoder implemented in Rust.
    #[clap(short, long)]
    rs_file: Option<PathBuf>,
    /// Generate the mechanical register-name tables (integer, fp, SVE, SIMD).
    #[clap(long)]
    registers: Option<PathBuf>,
    /// Generate a test binary.
    #[clap(short, long)]
    test_bin: Option<PathBuf>,
    /// The size limit of the generated test binary, the default is 64MB.
    #[clap(long, default_value = "67108864")]
    test_bin_size_limit: usize,
    /// The number of test encodings to generate for each instruction, the default is 0x10_000.
    #[clap(long, default_value = "65536")]
    test_encodings_limit: usize,
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

    if let Some(registers) = &opt.registers {
        log::info!("Writing register tables to a Rust file {registers:?}");
        let mut f = std::fs::File::create(registers)?;
        generate_registers::generate_registers(&mut f)?;
    }

    let filters = Filters {
        feature_sets: HashSet::from_iter(opt.feature_sets.unwrap_or_default()),
        classes: HashSet::from_iter(opt.insn_class.unwrap_or_default()),
        mnemonics: HashSet::from_iter(opt.mnemonic.unwrap_or_default()),
    };
    let insns = parse_insn_description(opt.description_json, &filters)?;

    let decision_tree_indexing = match opt.algo {
        Some(DecoderAlgo::Cond) | None => DecisionTreeIndexing::None,
        Some(DecoderAlgo::Bfs) => DecisionTreeIndexing::BFS,
        Some(DecoderAlgo::Dfs) => DecisionTreeIndexing::DFS,
    };
    let decision_tree = build_decision_tree(insns.as_slice(), decision_tree_indexing);
    if let Some(graphviz) = opt.graphviz {
        log::info!("Writing decision tree to a Graphviz dot file {graphviz:?}");
        let mut f = std::fs::File::create(graphviz)?;
        decistion_tree_to_graphviz_dot(&decision_tree, &mut f)?;
    }

    if let Some(rust) = opt.rs_file {
        log::info!("Writing decision tree to a Rust file {rust:?}");
        let mut f = std::fs::File::create(rust)?;
        decision_tree_to_rust(&decision_tree, decision_tree_indexing, &mut f)?;
    }

    if let Some(test_bin) = opt.test_bin {
        log::info!(
            "Generating test binary {test_bin:?}, limit {} bytes, {} encodings per instruction",
            opt.test_bin_size_limit,
            opt.test_encodings_limit
        );
        let mut f = std::fs::File::create(test_bin)?;
        generate_test_bin::generate_test_bin(
            insns.as_slice(),
            &mut f,
            opt.test_bin_size_limit,
            opt.test_encodings_limit,
        )?;
    }

    Ok(())
}

/// Include filters for the instruction set; an empty set matches everything.
struct Filters {
    feature_sets: HashSet<InsnFeatureSet>,
    classes: HashSet<InsnClass>,
    mnemonics: HashSet<String>,
}

impl Filters {
    fn accepts(&self, insn: &Insn) -> bool {
        (self.feature_sets.is_empty() || self.feature_sets.contains(&insn.feature_set))
            && (self.classes.is_empty() || self.classes.contains(&insn.class))
            && (self.mnemonics.is_empty() || self.mnemonics.contains(&insn.mnemonic))
    }

    fn log(&self) {
        fn describe<T: std::fmt::Debug>(noun: &str, set: &HashSet<T>) {
            if set.is_empty() {
                log::info!("Including instructions from all {noun}");
            } else {
                log::info!("Including instructions from {noun} {set:?}");
            }
        }
        describe("feature sets", &self.feature_sets);
        describe("classes", &self.classes);
        describe("mnemonics", &self.mnemonics);
    }
}

fn parse_insn_description(path: PathBuf, filters: &Filters) -> anyhow::Result<Vec<Rc<Insn>>> {
    log::info!("Loading {path:?}");
    filters.log();

    let data = std::fs::read_to_string(path)?;
    let insns = serde_json::from_str::<Vec<Insn>>(&data)?;

    let mut aliases = 0;
    let mut classes = HashSet::new();
    let mut feature_sets = HashSet::new();
    let mut filtered = Vec::new();

    for insn in &insns {
        if insn.flags.contains(InsnFlags::IS_ALIAS) {
            log::trace!("skipping alias {insn:x?}");
            aliases += 1;
            continue;
        }
        if !filters.accepts(insn) {
            log::trace!("Skipping {insn:x?}");
            continue;
        }

        log::debug!("instruction {insn:x?}");

        let extra_bits = insn.opcode & !insn.mask;
        if extra_bits != 0 {
            anyhow::bail!(
                "Invalid mask for {insn:x?}, opcode & mask != mask, extra bits: {extra_bits:#b}"
            );
        }
        if insn.mask == 0 {
            anyhow::bail!("Empty mask for {insn:x?}");
        }

        classes.insert(&insn.class);
        feature_sets.insert(&insn.feature_set);
        filtered.push(Rc::new(insn.clone()));
    }

    log::debug!("Classes {classes:?}");
    log::debug!("Feature sets {feature_sets:?}");

    log::info!(
        "Processed {} instructions, skipped {aliases} aliases, {} classes, {} feature sets filtered out {} instructions",
        insns.len(),
        classes.len(),
        feature_sets.len(),
        insns.len() - filtered.len()
    );

    log::info!("Loaded {} instructions", filtered.len());

    Ok(filtered)
}
