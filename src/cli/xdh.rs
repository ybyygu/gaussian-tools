// [[file:../../xo-tools.note::3b069e29][3b069e29]]
#![allow(non_camel_case_types)]

use super::*;
// 3b069e29 ends here

// [[file:../../xo-tools.note::d3061f91][d3061f91]]
/// Obtain XYG3-type doubly hybrid (xDH) results from Gaussian output
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    verbosity: Verbosity,

    /// Path to Gaussian output file
    outfile: PathBuf,
}

pub fn enter_main() -> Result<()> {
    use crate::xdh::*;
    use duct::cmd;

    let args = Cli::parse();
    args.verbosity.setup_logger();

    let xdh = xDH::collect_from_gaussian(&args.outfile)?;
    let energy_xyg3 = xdh.energy(Functional::XYG3);
    eprintln!("  E(XYG3)    =  {energy_xyg3:16.8} A.U.");
    println!("@model_properties_format_version 0.1");
    println!("# XYG3 energy: convert from a.u. to eV");
    println!("@energy unit_factor=27.211386024367243");
    println!("{energy_xyg3:16.8}");

    Ok(())
}
// d3061f91 ends here
