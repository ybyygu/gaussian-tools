// main

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*main][main:1]]
use std::path::PathBuf;

use quicli::prelude::*;
use structopt::*;

/// Update Gaussian input file from geometry optimization log file.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// Summarize all optimization steps of Gaussian log file.
    #[structopt(long="all", short="a")]
    show_all: bool,

    /// Summarize information about bond, angle etc, e..g bond length: 32,25.
    query: Option<String>,

    /// Path to Gaussian log file to be summarized.
    #[structopt(parse(from_os_str), short="o")]
    out_file: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    Ok(())
}
// main:1 ends here
