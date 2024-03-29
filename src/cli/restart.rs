// [[file:../../xo-tools.note::3ae69dc7][3ae69dc7]]
use super::*;
// 3ae69dc7 ends here

// [[file:../../xo-tools.note::*log][log:1]]
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

// [[file:../../xo-tools.note::6c25fcb8][6c25fcb8]]
/// Update input with new coordinates
///
/// # Parameters
/// * coords: xyz coordinates from Gaussian log file.
/// * path: path to Gaussian input file, *.com, *.gjf
///
fn update_with_coordinates<P: AsRef<Path>>(path: P, coords: &[[f64; 3]]) -> Result<String> {
    assert!(!coords.is_empty(), "no coords");
    let path = path.as_ref();
    info!("update file {path:?} with new coords");
    let pat = r"\s+[-0-9]+\.[0-9]+\s+[-0-9]+\.[0-9]+\s+[-0-9]+\.[0-9]+";
    let re = regex::Regex::new(pat).unwrap();

    let mut lines = vec![];
    let mut coords_iter = coords.iter();
    let reader = file_reader(path)?;
    let mut ireplaced = 0;
    for line in reader.lines() {
        let line = line?;
        let line = if let Some(m) = re.find(&line) {
            let s = m.start();
            let e = m.end();
            let xyz_old = &line[s..e];
            if let Some([x, y, z]) = coords_iter.next() {
                let xyz_new = format!("{:20.8}{:20.8}{:20.8}", x, y, z);
                ireplaced += 1;
                line.replace(xyz_old, &xyz_new)
            } else {
                panic!("output coords is inconsistent with input structure.");
            }
        } else {
            line
        };

        lines.push(line);
    }
    assert_eq!(ireplaced, coords.len(), "Incorrect number of coordinates!");

    // append final blank line to avoid the bug in Gaussian.
    lines.push("".into());

    Ok(lines.join("\n"))
}
// 6c25fcb8 ends here

// [[file:../../xo-tools.note::2814494e][2814494e]]
/// Update Gaussian input file from multi-step optimization job.
#[derive(Debug, StructOpt)]
#[clap(author, version, about)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    /// The Gaussian input file as the starting template.
    #[structopt(short = 't')]
    inp_file: Option<PathBuf>,

    /// The path to save Gaussian input file for restaring.
    #[structopt(short = 'o')]
    out_file: Option<PathBuf>,

    /// The Gaussian log file containing multiple geometries, such as a geometry
    /// optimization job.
    log_file: PathBuf,
}

pub fn enter_main() -> Result<()> {
    let args = Cli::parse();
    args.verbosity.setup_logger();

    let ofile = &args.log_file;
    info!("Log file: {}", ofile.display());

    let guessed_gjf = args.log_file.with_extension("gjf");
    let guessed_com = args.log_file.with_extension("com");
    let mut ifile = args.inp_file.unwrap_or_else(|| guessed_gjf);
    if !ifile.exists() {
        ifile = guessed_com;
    }
    info!("Input file: {}", ifile.display());

    let coords = parse_gaussian_log_file(ofile)?;
    info!("Found coordinates for {} atoms.", coords.len());

    let txt = update_with_coordinates(&ifile, &coords)?;

    if let Some(ofile) = args.out_file {
        gut::fs::write_to_file(ofile, &txt)?;
    } else {
        // setup a pager like `less` cmd
        pager::Pager::with_pager("less").setup();
        print!("{}", txt);
    }

    Ok(())
}
// 2814494e ends here
