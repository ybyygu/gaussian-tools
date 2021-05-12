// [[file:../../xo-tools.note::*imports][imports:1]]
use gaussian_tools::*;
use gut::cli::*;
use gut::prelude::*;
use structopt::*;

use std::path::{Path, PathBuf};
// imports:1 ends here

// [[file:../../xo-tools.note::*core][core:1]]
// Setup gaussian runtime environment.
// 
// create leading directories for Gaussian calculation per user.
fn init_env() -> Result<PathBuf> {
    let scrdir = std::env::var("GAUSS_SCRDIR").context("Get GAUSS_SCRDIR env var")?;
    let user = std::env::var("USER").context("Get USER env var")?;
    let scr_root_dir = Path::new(&scrdir).join(user);
    info!("Scratching root dir: {:?}", scr_root_dir);

    // create leading directories
    std::fs::create_dir_all(&scr_root_dir).context("Create scratch directories")?;

    Ok(scr_root_dir)
}

fn run_gaussian(cmd: &str, input: &str, output_file: &Path, scr_root_dir: &Path) -> Result<()> {
    let dir = tempfile::tempdir_in(scr_root_dir).context("Create scratching dir")?;
    info!("Gaussian job scr dir: {:?}", dir.path());

    // cat "$GAUSS_SCRDIR/Gau-input.gjf" | ${cmd} > "$GAUSS_OUTPUT_FILENAME"
    duct::cmd!(cmd)
        .env("GAUSS_SCRDIR", dir.path())
        .stdin_bytes(input)
        .stdout_path(output_file)
        .run()?;
    info!("Gaussian job finished.");

    Ok(())
}

// Fix Windows line endings issue
fn fix_line_endings_issue(txt: &str) -> String {
    // convert to Unix line endings
    let mut txt = txt.replace("\r", "");

    // append a new line for avoiding a Gaussian bug
    txt.push_str("\n");

    txt
}
// core:1 ends here

// [[file:../../xo-tools.note::*main][main:1]]
/// A convenient wrapper for running Gaussian program
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// The Gaussian input file
    inp_file: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_logger();

    let out_file = args.inp_file.with_extension("log");
    let input = gut::fs::read_file(&args.inp_file)?;
    let input = fix_line_endings_issue(&input);

    let scr_dir = init_env()?;
    run_gaussian("g09", &input, &out_file, &scr_dir)?;

    Ok(())
}
// main:1 ends here
