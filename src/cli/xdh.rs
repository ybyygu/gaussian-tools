// [[file:../../xo-tools.note::3b069e29][3b069e29]]
#![allow(non_camel_case_types)]

use super::*;
// 3b069e29 ends here

// [[file:../../xo-tools.note::c11365a3][c11365a3]]
/// Read all relevant lines for XYG3 from Gaussian generated log file
fn extract_relevant_lines(f: &Path) -> Result<Vec<String>> {
    let f = file_reader(f)?;

    #[rustfmt::skip]
    let keywords = ["ENTVJ= ", "SCF Done:", "alpha-beta", "alpha-alpha", "beta-beta", "Erf(P)="];

    let lines = f
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
        line[24..].trim().parse().ok()
    } else {
        None
    }
}
// 86b74a1f ends here

// [[file:../../xo-tools.note::e68b3776][e68b3776]]
// ENTVJ= -133.281125 Ex=  -16.365355 Ec=    0.000000 ETotM2e= -234.7283473371  ETot= -149.6464806455
fn parse_entvj(line: &str) -> Option<[f64; 4]> {
    let parts = line.split_whitespace().collect_vec();
    assert_eq!(parts.len(), 10, "invalid {line:?}");

    [
        parts[1].parse().ok()?,
        parts[3].parse().ok()?,
        parts[5].parse().ok()?,
        parts[7].parse().ok()?,
    ]
    .into()
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
}
// 0491bdad ends here

// [[file:../../xo-tools.note::029f58f8][029f58f8]]
//  Erf(P)=          -0.000578845929
fn parse_solvent(line: &str) -> Option<f64> {
    if line.starts_with(" Erf(P)= ") {
        dbg!(line[9..].trim()).parse().ok()
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

// [[file:../../xo-tools.note::0e2e1938][0e2e1938]]
#[test]
fn test_parse() -> Result<()> {
    let f = "tests/files/Job_o2.log";

    let lines = extract_relevant_lines(f.as_ref())?;

    let p: Option<Vec<_>> = lines
        .iter()
        .filter(|line| line.contains("ENTVJ="))
        .map(|line| parse_entvj(line))
        .collect();
    dbg!(p);

    let p: Option<_> = lines
        .iter()
        .filter(|line| line.contains("Erf(P)="))
        .last()
        .map(|line| parse_solvent(line));
    dbg!(p);

    let p: Option<Vec<_>> = lines
        .iter()
        .filter(|line| line.contains("alpha-alpha"))
        .map(|line| parse_os_ss(line))
        .collect();
    dbg!(p);

    let p: Option<Vec<_>> = lines
        .iter()
        .filter(|line| line.contains("alpha-beta"))
        .map(|line| parse_os_ss(line))
        .collect();
    dbg!(p);

    let p: Option<Vec<_>> = lines
        .iter()
        .filter(|line| line.contains("beta-beta"))
        .map(|line| parse_os_ss(line))
        .collect();
    dbg!(p);

    Ok(())
}
// 0e2e1938 ends here
