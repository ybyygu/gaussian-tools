// [[file:../../xo-tools.note::9b77d0e3][9b77d0e3]]
use super::*;
// 9b77d0e3 ends here

// [[file:../../xo-tools.note::8fbf13aa][8fbf13aa]]
/// Generate cube file using Multiwfn from Gaussian output file
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    verbosity: Verbosity,

    /// The path to save generated cube file
    #[clap(short = 'p')]
    exec_path: PathBuf,

    /// The input file *.fchk or *.wfn
    input_file: PathBuf,

    /// Input orbial index
    #[clap(short = 'o')]
    orbital_number: usize,
}

pub fn enter_main() -> Result<()> {
    use duct::cmd;

    let args = Cli::parse();
    args.verbosity.setup_logger();

    let mut inputs = String::new();
    writeln!(&mut inputs, "5");
    writeln!(&mut inputs, "4");
    writeln!(&mut inputs, "1");
    writeln!(&mut inputs, "3");
    writeln!(&mut inputs, "2");
    writeln!(&mut inputs, "0");
    writeln!(&mut inputs, "q");

    let o = cmd!("Multiwfn", &args.input_file).stdin_bytes(inputs).run()?;
    dbg!(o);
    let cube_file: &Path = "MOvalue.cub".as_ref();
    if cube_file.exists() {
        let final_cube = args
            .exec_path
            .join(args.input_file.with_extension("cub").file_name().unwrap());
        std::fs::copy(&cube_file, &final_cube)?;
        info!("final cube file wrote to: {:?}", final_cube);
    }

    Ok(())
}
// 8fbf13aa ends here
