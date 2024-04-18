// [[file:../../xo-tools.note::146c5546][146c5546]]
use super::*;
// 146c5546 ends here

// [[file:../../xo-tools.note::c11365a3][c11365a3]]
/// Read all relevant lines for XYG3 from Gaussian generated log file
fn extract_relevant_lines(f: &Path) -> Result<Vec<String>> {
    let f = file_reader(f)?;
    extract_relevant_lines_from(f)
}

/// Read all relevant lines for XYG3 from Gaussian generated log file
fn extract_relevant_lines_from(s: impl BufRead) -> Result<Vec<String>> {
    #[rustfmt::skip]
    let keywords = ["ENTVJ=", "SCF Done:", "alpha-beta", "alpha-alpha", "beta-beta", "Erf(P)="];

    let lines = s
        .lines()
        .filter_map(move |line| {
            // NOTE: line is wrapped in a Result type for UTF-8 issue
            let line = line.ok()?;
            if keywords.iter().any(|key| line.contains(key)) {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    Ok(lines)
}
// c11365a3 ends here

// [[file:../../xo-tools.note::86b74a1f][86b74a1f]]
//  SCF Done:  E(UB3LYP) =  -150.367567881     A.U. after    9 cycles
fn parse_scf_done(line: &str) -> Option<f64> {
    if line.starts_with(" SCF Done:") {
        line[24..44].trim().parse().ok()
    } else {
        None
    }
}
// 86b74a1f ends here

// [[file:../../xo-tools.note::e68b3776][e68b3776]]
// ENTVJ= -133.281125 Ex=  -16.365355 Ec=    0.000000 ETotM2e= -234.7283473371  ETot= -149.6464806455
// ENTVJ= -363.840442 Ex=  -50.358635 Ec=   -5.600997 ETotM2e=-1072.3996282781  ETot= -419.8000737661
fn parse_entvj(line: &str) -> Option<[f64; 4]> {
    // NOTE: could be invalid when the number is large
    // let parts = line.split_whitespace().collect_vec();
    let parts: Vec<_> = line
        .split('=')
        .skip(1)
        .filter_map(|s| {
            let x = s.split_whitespace().next()?;
            x.parse::<f64>().ok()
        })
        .collect();
    assert_eq!(parts.len(), 5, "invalid line: {line:?}");

    Some([parts[0], parts[1], parts[2], parts[3]])
}
// e68b3776 ends here

// [[file:../../xo-tools.note::0491bdad][0491bdad]]
// alpha-beta  T2 =       0.1397256845D+00 E2=     -0.3642781731D+00
fn parse_os_ss(line: &str) -> Option<f64> {
    if line.starts_with("     alpha-") || line.starts_with("     beta-") {
        let value = line[49..].replace("D", "E").trim().parse().ok()?;
        Some(value)
    } else {
        None
    }
}

#[test]
fn test_xdh_os_ss() {
    let line = "     alpha-beta  T2 =       0.1397256845D+00 E2=     -0.3642781731D+00";
    let x = parse_os_ss(line);
    assert_eq!(x, Some(-0.3642781731));

    let line = "     alpha-alpha T2 =       0.1912645165D-01 E2=     -0.6748185095D-01";
    let x = parse_os_ss(line);
    assert_eq!(x, Some(-0.06748185095));

    let line = "     beta-beta   T2 =       0.7629260704D-01 E2=     -0.1043875581D+00";
    let x = parse_os_ss(line);
    assert_eq!(x, Some(-0.1043875581));

    // ENTVJ= -363.840442 Ex=  -50.358635 Ec=   -5.600997 ETotM2e=-1072.3996282781  ETot= -419.8000737661
    let line = "            ENTVJ= -363.840442 Ex=  -50.358635 Ec=   -5.600997 ETotM2e=-1072.3996282781  ETot= -419.8000737661";
    // let line = "            ENTVJ=-1261.054618 Ex= -116.450185 Ec=    0.000000 ETotM2e=-2749.2509298729  ETot=-1377.5048029040";
    let parts: Vec<_> = line
        .split('=')
        .skip(1)
        .filter_map(|s| {
            let x = s.split_whitespace().next()?;
            x.parse::<f64>().ok()
        })
        .collect();
    dbg!(parts);
    let parts = parse_entvj(&line);
    assert!(parts.is_some());
    assert_eq!(parts.unwrap()[3], -1072.3996282781);
}
// 0491bdad ends here

// [[file:../../xo-tools.note::029f58f8][029f58f8]]
//  Erf(P)=          -0.000578845929
fn parse_solvent(line: &str) -> Option<f64> {
    if line.starts_with(" Erf(P)= ") {
        line[9..].trim().parse().ok()
    } else {
        None
    }
}

#[test]
fn test_parse_solvent() {
    let line = " Erf(P)=          -0.000578845929";
    assert_eq!(parse_solvent(line), Some(-0.000578845929));
}
// 029f58f8 ends here

// [[file:../../xo-tools.note::f1ce30a8][f1ce30a8]]
fn collect_energy_components_from_file(f: &Path) -> Result<(f64, Component)> {
    let r = file_reader(f)?;
    collect_energy_components_from(r)
}

fn collect_energy_components_from(s: impl BufRead) -> Result<(f64, Component)> {
    let lines = extract_relevant_lines_from(s)?;

    let mut comp = Component::default();
    let mut energy_no_xc = f64::NAN;
    let mut energy_pt2 = [f64::NAN; 2];

    let p: Option<Vec<_>> = lines
        .iter()
        .filter(|line| line.contains("ENTVJ="))
        .map(|line| parse_entvj(line))
        .collect();
    if let Some(x) = p {
        if x.len() != 3 {
            bail!("no ENTVJ line found in output stream");
        }
        energy_no_xc = x[0][0];
        comp[0] = x[0][1];
        comp[1] = x[1][1];
        comp[2] = x[2][1];
        comp[3] = x[1][2];
        comp[4] = x[2][2];
    } else {
        bail!("Error happens in collecting the DFT components in output stream");
    }

    // collect PT2 energy terms
    let p: Option<_> = lines
        .iter()
        .filter(|line| line.contains("alpha-alpha"))
        .last()
        .map(|line| parse_os_ss(line));
    if let Some(Some(x)) = p {
        energy_pt2[1] = x;
    } else {
        bail!("Error happens in collecting the alpha-alpha ssPT2 in output stream");
    }

    let p: Option<_> = lines
        .iter()
        .filter(|line| line.contains("alpha-beta"))
        .last()
        .map(|line| parse_os_ss(line));
    if let Some(Some(x)) = p {
        energy_pt2[0] = x;
    } else {
        bail!("Error happens in collecting the alpha-beta osPT2 in output stream");
    }

    let p: Option<_> = lines
        .iter()
        .filter(|line| line.contains("beta-beta"))
        .last()
        .map(|line| parse_os_ss(line));
    if let Some(Some(x)) = p {
        energy_pt2[1] += x;
    } else {
        bail!("Error happens in collecting the beta-beta ssPT2 in output stream");
    }
    comp[5] = energy_pt2[0];
    comp[6] = energy_pt2[1];

    // collect solvation energy term, which is optional
    let p: Option<_> = lines
        .iter()
        .filter(|line| line.contains("Erf(P)="))
        .last()
        .map(|line| parse_solvent(line));
    if let Some(Some(x)) = p {
        energy_no_xc += x;
    }

    Ok((energy_no_xc, comp))
}
// f1ce30a8 ends here

// [[file:../../xo-tools.note::97608d27][97608d27]]
impl xDH {
    /// Collect from gaussian output file or from stdin stream
    pub fn collect_from_gaussian<'a>(f: impl Into<Option<&'a Path>>) -> Result<Self> {
        let (energy_no_xc, component) = if let Some(f) = f.into() {
            info!("Reading Gaussian output from {f:?} ...");
            let outfile = file_reader(f)?;
            collect_energy_components_from(outfile)?
        } else {
            info!("Reading Gaussian output from stdin ...");
            let mut stdin = std::io::stdin().lock();
            collect_energy_components_from(stdin)?
        };
        let xdh = Self {
            component,
            energy_no_xc,
        };

        Ok(xdh)
    }
}
// 97608d27 ends here

// [[file:../../xo-tools.note::0e2e1938][0e2e1938]]
#[test]
fn test_parse() -> Result<()> {
    let f: &Path = "tests/files/Job_o2.log".as_ref();
    let (e_no_xc, comp) = collect_energy_components_from_file(f)?;
    assert_eq!(e_no_xc, -133.28191752160902);

    #[rustfmt::skip]
    let comp_expected = [-16.364758, -14.908981, -16.505819, -1.426661, -0.573734, -0.3642781731, -0.17186940905];
    assert_eq!(comp, comp_expected);

    Ok(())
}
// 0e2e1938 ends here
