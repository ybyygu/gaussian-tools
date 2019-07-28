// bin/log-restart.rs
// :PROPERTIES:
// :header-args: :tangle src/bin/log-restart.rs
// :END:

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*bin/log-restart.rs][bin/log-restart.rs:1]]
use std::path::PathBuf;

use quicli::prelude::*;
use structopt::*;

/// Update Gaussian input file from geometry optimization log file.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// edit .gjf/.com file in place, and no new file will be generated.
    #[structopt(long="in-place", short="i")]
    in_place: bool,

    /// specify the restarted .gjf/.com file name.
    #[structopt(parse(from_os_str), short="o")]
    out_file: Option<PathBuf>,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    Ok(())
}
// bin/log-restart.rs:1 ends here
