// [[file:../../xo-tools.note::3b069e29][3b069e29]]
#![allow(non_camel_case_types)]

use super::*;
// 3b069e29 ends here

// [[file:../../xo-tools.note::d3061f91][d3061f91]]
#[derive(Subcommand, Debug)]
enum Action {
    /// Rewrite Gaussian input file to make it suitable for XYG3 type calculations
    Rewrite(RewriteInput),
    /// Obtain energy of XYG3 type functional from relevant Gaussian output file
    Obtain(ObtainFrom),
}

#[derive(Args, Debug)]
struct RewriteInput {
    /// Path to Gaussian input file containing "XYG3" or other xDH functional keywords
    inpfile: PathBuf,
}

#[derive(Args, Debug)]
struct ObtainFrom {
    /// Path to Gaussian output file relevant for XYG3 type calculations
    outfile: PathBuf,
}

/// Obtain XYG3-type doubly hybrid (xDH) results from Gaussian output
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    verbosity: Verbosity,

    #[clap(subcommand)]
    action: Action,
}

pub fn enter_main() -> Result<()> {
    use crate::xdh::*;
    use duct::cmd;

    let args = Cli::parse();
    args.verbosity.setup_logger();

    match args.action {
        Action::Rewrite(rewrite) => {
            xDH::rewrite_gaussian_input(&rewrite.inpfile)?;
        }
        Action::Obtain(obtain) => {
            let xdh = xDH::collect_from_gaussian(&obtain.outfile)?;
            let energy_xyg3 = xdh.energy(Functional::XYG3);
            eprintln!("  E(XYG3)    =  {energy_xyg3:16.8} A.U.");
            println!("@model_properties_format_version 0.1");
            println!("# XYG3 energy: convert from a.u. to eV");
            println!("@energy unit_factor=27.211386024367243");
            println!("{energy_xyg3:16.8}");
        }
    }

    Ok(())
}
// d3061f91 ends here
