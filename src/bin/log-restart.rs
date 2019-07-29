// imports

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*imports][imports:1]]
use std::io::BufRead;
use std::path::Path;

use xo_tools::*;
// imports:1 ends here

// log

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*log][log:1]]
/// Parse xyz coordinates from gaussian log file.
fn parse_gaussian_log_file<P: AsRef<Path>>(path: P) -> Result<Vec<[f64; 3]>> {
    let reader = file_reader(&path)?;

    // the keyword indicating lines for coordinates information
    let key = " orientation:";

    // record line number containing `orientation` line.
    let mut line_numbers = vec![];

    // record number of total atoms
    let mut natoms = None;

    // record current line number
    let mut nline = 0;
    for line in reader.lines() {
        let line = line?;
        if natoms.is_none() {
            // Sample line:
            // ------------
            //  NAtoms=     60 NQM=       60 NQMF=       0 NMMI=      0 NMMIF=      0
            //
            let k = " NAtoms=";
            if line.starts_with(k) {
                let parts: Vec<_> = line[k.len()..].trim_start().splitn(2, " ").collect();
                let n: usize = parts[0].parse()?;
                natoms = Some(n);
            }
        }

        if line.contains(key) {
            line_numbers.push(nline);
        }
        nline += 1;
    }

    // save the total number of lines.
    let nlines = nline;

    // take the last good record of coordinates during multiple-step optimization.
    assert!(
        line_numbers.len() >= 1,
        "incomplete coordinates records: {:?}",
        line_numbers
    );

    let natoms = natoms.expect("Failed to get total number of atoms.");
    let nstart = match line_numbers.len() {
        0 => panic!("incomplete coordinates records: {:?}", line_numbers),
        1 => line_numbers[0],
        _ => {
            let x = line_numbers.pop().unwrap();
            if natoms + x < nlines {
                line_numbers.pop().unwrap()
            } else {
                x
            }
        }
    };

    info!("Start line number of coordinates: {}", nstart);
    let reader = file_reader(&path)?;

    // Sample record
    //
    // Standard orientation:
    // ---------------------------------------------------------------------
    //     Center     Atomic      Atomic             Coordinates (Angstroms)
    //     Number     Number       Type             X           Y           Z
    //     ---------------------------------------------------------------------
    //     1          6           0        1.196105    0.388638    3.377408
    //     2          6           0        0.739233   -1.017467    3.377408

    let mut coords: Vec<_> = vec![];
    for (line, _) in reader.lines().skip(nstart + 5).zip(0..natoms) {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();
        let x: f64 = parts[3].parse()?;
        let y: f64 = parts[4].parse()?;
        let z: f64 = parts[5].parse()?;
        coords.push([x, y, z]);
    }
    assert_eq!(coords.len(), natoms);

    Ok(coords)
}

#[test]
#[ignore]
fn test_parse_log() -> Result<()> {
    let fname = "/share/apps/gaussian/g09/tests/amd64/test0333.log";
    let coords = parse_gaussian_log_file(fname)?;

    Ok(())
}
// log:1 ends here

// update

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*update][update:1]]
/// Update input with new coordinates
fn update_with_coordinates<P: AsRef<Path>>(path: P, coords: &[[f64; 3]]) -> Result<String> {
    let pat = r"\s+[-0-9]+\.[0-9]+\s+[-0-9]+\.[0-9]+\s+[-0-9]+\.[0-9]+";
    let re = regex::Regex::new(pat).unwrap();

    let mut lines = vec![];
    let mut coords_iter = coords.iter();
    let reader = file_reader(path)?;
    for line in reader.lines() {
        let line = line?;
        let line = if let Some(m) = re.find(&line) {
            let s = m.start();
            let e = m.end();
            let xyz_old = &line[s..e];
            if let Some([x, y, z]) = coords_iter.next() {
                let xyz_new = format!("{:20.8}{:20.8}{:20.8}", x, y, z);
                line.replace(xyz_old, &xyz_new)
            } else {
                panic!("coords is inconsistent with input");
            }
        } else {
            line
        };

        lines.push(line);
    }
    // append final blank line to avoid the bug in Gaussian.
    lines.push("".into());

    Ok(lines.join("\n"))
}
// update:1 ends here

// main

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*main][main:1]]
use std::path::PathBuf;

use quicli::prelude::*;
use structopt::*;

/// Update Gaussian input file from multi-step optimization job.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// The Gaussian input file as the starting template.
    #[structopt(parse(from_os_str), short = "-t")]
    inp_file: Option<PathBuf>,

    /// The Gaussian log file containing multiple geometries, such as a geometry
    /// optimization job.
    #[structopt(parse(from_os_str))]
    out_file: PathBuf,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("log-restart")?;

    let ofile = &args.out_file;
    info!("Log file: {}", ofile.display());

    let guessed = args.out_file.with_extension("gjf");
    let ifile = args.inp_file.unwrap_or_else(|| guessed);
    info!("Input file: {}", ifile.display());

    let coords = parse_gaussian_log_file(ofile)?;
    info!("Found coordinates for {} atoms.", coords.len());

    let txt = update_with_coordinates(&ifile, &coords)?;

    // setup a pager like `less` cmd
    pager::Pager::with_pager("less").setup();

    print!("{}", txt);

    Ok(())
}
// main:1 ends here
