// [[file:../../xo-tools.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../../xo-tools.note::1c08ebe1][1c08ebe1]]
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
            debug!("{}", line);
        } else if line.starts_with(" Stoichiometry") {
            debug!("{}", line);
        } else if line.starts_with(" Standard basis:") {
            debug!("{}", line);
        } else if line.starts_with(" General basis") {
            debug!("{}", line);
        } else if line.starts_with(" Framework group") {
            debug!("{}", line);
        } else if line.starts_with(" Deg. of freedom") {
            info!("{}", line);
            banner!();
        } else if line.contains("Standard basis") {
            debug!("{}", line);
        } else if line.contains("basis functions") {
            debug!("{}", line);
        } else if line.contains("(Enter ") {
            debug!("{}", line);
        } else if line.contains("Leave Link ") {
            debug!("{}", line);
        } else if line.contains("Number of steps in this run=") {
            info!("{}", line);
        } else if line.contains("Convergence criterion not met") {
            info!("{}", line);
            // # print SCF information and the next two lines
        } else if line.starts_with(" SCF Done: ") {
            warn!("{line}");
            print_next_line!(lines);
            banner!();
        } else if line.contains("Step number") {
            info!("{}", line);
        } else if line.contains("exceeded") {
            info!("{}", line);
        } else if line.contains("energy=") {
            debug!("{}", line);
        } else if line.contains("Counterpoise:") {
            info!("{}", line);
        } else if line.starts_with(" Energy=") {
            info!("{}", line);
        } else if line.starts_with(" Cycle ") {
            trace!("{}", line);
        } else if line.starts_with(" E=") {
            trace!("{}", line);
        } else if line.contains("ONIOM: generating point") {
            info!("{}", line);
        } else if line.contains("ONIOM: extrapolated energy") {
            info!("{}", line);
        } else if line.contains("ONIOM: Dipole moment") {
            info!("{}", line);
            print_next_line!(lines);
        } else if line.contains("Eigenvalues ---") {
            info!("{}", line);
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
            info!("{}", line);
            for _ in 0..7 {
                print_next_line!(lines);
            }
            banner!();
        } else if line.contains("WARNING") {
            warn!("{}", line);
        } else if line.contains("Warning") {
            warn!("{}", line);
        } else if line.contains("Frequencies --") {
            if first_time {
                info!("{}", line);
            }
            first_time = false;
        } else if line.contains("Zero-point correction=") {
            info!("{}", line);
        } else if line.contains("Thermal correction to") {
            info!("{}", line);
        } else if line.contains("Sum of electronic and") {
            info!("{}", line);
            if line.contains("thermal Free Energies") {
                banner!();
            }
        } else if line.contains("termination") {
            info!("{}", line);
        } else if line.contains("Job cpu time:") {
            info!("{}", line);
        }
    }

    Ok(())
}
// 1c08ebe1 ends here

// [[file:../../xo-tools.note::6df1e54a][6df1e54a]]
/// Print important lines found in a Gaussian output file.
#[derive(Debug, StructOpt)]
#[clap(author, version, about)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// Summarize all optimization steps of Gaussian log file.
    #[structopt(long = "all", short = 'a')]
    show_all: bool,

    /// Path to Gaussian log file to be summarized.
    log_file: PathBuf,
}

pub fn enter_main() -> Result<()> {
    let args = Cli::parse();
    gut::logger::setup_plain_logger();

    // setup a pager like `less` cmd
    pager::Pager::with_pager("less").setup();

    let reader = file_reader(args.log_file)?;
    let _ = summarize_gauss_log(reader)?;

    Ok(())
}
// 6df1e54a ends here
