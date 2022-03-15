// [[file:../../xo-tools.note::3b069e29][3b069e29]]
#![allow(non_camel_case_types)]

use super::*;
// 3b069e29 ends here

// [[file:../../xo-tools.note::d3061f91][d3061f91]]
#[derive(Debug, Parser)]
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
    println!("  E(XYG3)    =  {energy_xyg3:16.8}");

    Ok(())
}
// d3061f91 ends here
