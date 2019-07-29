// imports

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*imports][imports:1]]
use std::io::BufRead;

use quicli::prelude::*;

type Result<T> = ::std::result::Result<T, Error>;
// imports:1 ends here

// core

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*core][core:1]]
macro_rules! banner {
    () => {
        println!(" {:-^72}", "");
    };
}

macro_rules! print_next_line {
    ($lines:ident) => {
        let line = $lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        println!("{}", line);
    };
}

/// Walk the flog and print the essential information.
fn summarize_gauss_log<R: BufRead>(flog: R) -> Result<()> {
    let mut lines = flog.lines();

    let mut first_time = true;
    while let Some(line) = lines.next() {
        let line = line?;
        if line.contains("Revision") {
            println!("{}", line);
        } else if line.starts_with(" Stoichiometry") {
            println!("{}", line);
        } else if line.starts_with(" Standard basis:") {
            println!("{}", line);
        } else if line.starts_with(" General basis") {
            println!("{}", line);
        } else if line.starts_with(" Framework group") {
            println!("{}", line);
        } else if line.starts_with(" Deg. of freedom") {
            println!("{}", line);
            banner!();
        } else if line.contains("Standard basis") {
            println!("{}", line);
        } else if line.contains("basis functions") {
            println!("{}", line);
        } else if line.contains("(Enter ") {
            println!("{}", line);
        } else if line.contains("Leave Link ") {
            println!("{}", line);
        } else if line.contains("Number of steps in this run=") {
            println!("{}", line);
        // # print SCF information and the next two lines
        } else if line.contains("SCF Done") {
            print_next_line!(lines);
            print_next_line!(lines);
            banner!();
        } else if line.contains("Step number") {
            println!("{}", line);
        } else if line.contains("exceeded") {
            println!("{}", line);
        } else if line.contains("energy=") {
            println!("{}", line);
        } else if line.contains("Counterpoise:") {
            println!("{}", line);
        } else if line.starts_with(" Energy=") {
            println!("{}", line);
        } else if line.starts_with(" Cycle ") {
            println!("{}", line);
        } else if line.starts_with(" E=") {
            println!("{}", line);
        } else if line.contains("ONIOM: generating point") {
            println!("{}", line);
        } else if line.contains("ONIOM: extrapolated energy") {
            println!("{}", line);
        } else if line.contains("ONIOM: Dipole moment") {
            println!("{}", line);
            print_next_line!(lines);
        } else if line.contains("Eigenvalues ---") {
            println!("{}", line);
            // skip other Eigenvalues lines
            while let Some(line) = lines.next() {
                let line = line?;
                if !line.contains("Eigenvalues ---") {
                    break;
                }
            }
        }
        // # print converged information
        else if line.contains("Converged?") {
            banner!();
            println!("{}", line);
            for _ in 0..7 {
                print_next_line!(lines);
            }
            banner!();
        } else if line.contains("WARNING") {
            println!("{}", line);
        } else if line.contains("Warning") {
            println!("{}", line);
        } else if line.contains("Frequencies --") {
            if first_time {
                println!("{}", line);
            }
            first_time = false;
        } else if line.contains("Zero-point correction=") {
            println!("{}", line);
        } else if line.contains("Thermal correction to") {
            println!("{}", line);
        } else if line.contains("Sum of electronic and") {
            println!("{}", line);
            if line.contains("thermal Free Energies") {
                banner!();
            }
        } else if line.contains("termination") {
            println!("{}", line);
        } else if line.contains("Job cpu time:") {
            println!("{}", line);
        }
    }

    Ok(())
}
// core:1 ends here

// main

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*main][main:1]]
use std::path::PathBuf;

use structopt::*;

/// Print important lines found in a Gaussian output file.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// Summarize all optimization steps of Gaussian log file.
    #[structopt(long = "all", short = "a")]
    show_all: bool,

    /// Path to Gaussian log file to be summarized.
    #[structopt(parse(from_os_str))]
    log_file: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    // setup a pager like `less` cmd
    pager::Pager::with_pager("less -r").setup();
    let f = std::fs::File::open(&args.log_file)?;
    let f = std::io::BufReader::new(f);
    let _ = summarize_gauss_log(f)?;

    Ok(())
}
// main:1 ends here
