// [[file:../../xo-tools.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../../xo-tools.note::d3295ec9][d3295ec9]]
// Setup gaussian runtime environment.
//
// create leading directories for Gaussian calculation per user.
fn init_env() -> Result<PathBuf> {
    let scrdir = std::env::var("GAUSS_SCRDIR").context("Get GAUSS_SCRDIR env var")?;
    let user = std::env::var("USER").context("Get USER env var")?;
    let scr_root_dir = Path::new(&scrdir).join(user);
    info!("Scratching root dir: {:?}", scr_root_dir);

    // create leading directories
    // FIXME: potential permission issue for other users
    std::fs::create_dir_all(&scr_root_dir).context("Create scratch directories")?;

    Ok(scr_root_dir)
}

// Return exe name of Gaussian program providing path to a rc file
//
// remove version separator dot: g09.b02.rc ==> g09
fn get_gaussian_exe_from_path(rcfile: &Path) -> Option<String> {
    // make sure there is an entension in the path
    let _ext = rcfile.extension()?;
    let gxx = rcfile.file_name()?.to_str()?.split(".").next()?;
    Some(gxx.into())
}

pub(crate) fn run_gaussian(input: &str, output_file: Option<PathBuf>, rcfile: &Path) -> Result<()> {
    let scr_root_dir = init_env()?;
    let tdir = tempfile::tempdir_in(scr_root_dir).context("Create scratching dir")?;
    let scr_dir = tdir.path();
    info!("Gaussian job scr dir: {:?}", scr_dir);

    let gxx = get_gaussian_exe_from_path(rcfile).unwrap();
    let script = format!(
        "#! /usr/bin/env bash

source \"{rcfile}\"
source \"${gxx}root/{gxx}/bsd/{gxx}.profile\"
\"${gxx}root/{gxx}/{gxx}\"

",
        rcfile = rcfile.display(),
        gxx = gxx,
    );

    info!("calling script: {:?}", script);
    let runfile = scr_dir.join("run");
    gut::fs::write_script_file(&runfile, &script)?;

    if let Some(out) = output_file {
        duct::cmd!(runfile)
            .env("GAUSS_SCRDIR", scr_dir)
            .stdin_bytes(input)
            .stdout_path(out)
            .run()?;
    } else {
        duct::cmd!(runfile)
            .env("GAUSS_SCRDIR", scr_dir)
            .stdin_bytes(input)
            .run()?;
    }

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
// d3295ec9 ends here

// [[file:../../xo-tools.note::3f24c131][3f24c131]]
/// A convenient wrapper for running Gaussian program in different version
#[derive(Debug, StructOpt)]
#[clap(author, version, about)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// Path to the Gaussian input file. If not set, it will be read from stdin.
    inp_file: Option<PathBuf>,

    /// Path to Gaussian output file. If not set, it will be determined
    /// automatically based on input file name.
    #[structopt(short = 'o')]
    out_file: Option<PathBuf>,

    /// The main Gaussian executable name: g03, g09, g16, ...
    #[structopt(short = 'x')]
    gauss_exe: String,
}

pub fn enter_main() -> Result<()> {
    let args = Cli::parse();
    args.verbosity.setup_logger();

    // The path to real executable binary file
    let real_path = std::env::current_exe().context("Failed to get exe path")?;
    // rc file is in the same directory of the real executable binary
    let rc_name = format!("{}.rc", &args.gauss_exe);
    let rc_file = real_path.with_file_name(&rc_name);

    let input = read_from_stdin_or_file(args.inp_file.as_ref())?;
    let out_file = args.out_file.or(args.inp_file.as_ref().map(|x| guess_output_file(x)));

    run_gaussian(&input, out_file, &rc_file)?;

    Ok(())
}

/// Guess an output file name from input file name
fn guess_output_file(inp_file: &Path) -> PathBuf {
    let out_file = inp_file.with_extension("log");
    assert_ne!(
        out_file.extension(),
        inp_file.extension(),
        "invalid input file name: {inp_file:?}"
    );
    out_file
}

fn read_from_stdin_or_file(f: Option<&PathBuf>) -> Result<String> {
    let input = if let Some(f) = f {
        gut::fs::read_file(f)?
    } else {
        info!("Reading Gaussian input from stdin ...");
        let mut buffer = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut buffer)?;
        buffer
    };
    let input = fix_line_endings_issue(&input);

    Ok(input)
}
// 3f24c131 ends here

// [[file:../../xo-tools.note::*test][test:1]]
#[test]
fn test_xx() {
    let p = Path::new("/share/apps/gaussian/bin/g03.rc");
    assert_eq!(get_gaussian_exe_from_path(&p), Some("g03".into()));

    let p = Path::new("/share/apps/gaussian/bin/g09.E01.rc");
    assert_eq!(get_gaussian_exe_from_path(&p), Some("g09".into()));

    let p = Path::new("/share/apps/gaussian/bin/g03");
    assert_eq!(get_gaussian_exe_from_path(&p), None);
}
// test:1 ends here
