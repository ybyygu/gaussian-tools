// [[file:../../xo-tools.note::*imports][imports:1]]
use gut::prelude::*;
// imports:1 ends here

// [[file:../../xo-tools.note::*core][core:1]]
use gut::cli::*;
use std::path::{Path, PathBuf};
use structopt::*;

/// Generate cube file using Multiwfn for Gaussian output file
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// The path to save generated cube file
    #[structopt(short = "p")]
    exec_path: PathBuf,

    /// The input file *.fchk or *.wfn
    input_file: PathBuf,

    /// Input orbial index
    #[structopt(short = "o")]
    orbital_number: usize,
}

fn main() -> Result<()> {
    use duct::cmd;

    let args = Cli::from_args();
    args.verbosity.setup_logger();

    let mut inputs = String::new();
    writeln!(&mut inputs, "5");
    writeln!(&mut inputs, "4");
    writeln!(&mut inputs, "1");
    writeln!(&mut inputs, "3");
    writeln!(&mut inputs, "2");
    writeln!(&mut inputs, "0");
    writeln!(&mut inputs, "q");

    let o = cmd!("Multiwfn", &args.input_file)
        .stdin_bytes(inputs)
        .run()?;
    dbg!(o);
    let cube_file: &Path = "MOvalue.cub".as_ref();
    if cube_file.exists() {
        let final_cube = args.exec_path.join(args.input_file.with_extension("cub").file_name().unwrap());
        std::fs::copy(&cube_file, &final_cube)?;
        info!("final cube file wrote to: {:?}", final_cube);
    }

    Ok(())
}
// core:1 ends here
