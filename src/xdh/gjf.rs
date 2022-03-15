// [[file:../../xo-tools.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../../xo-tools.note::f45e0853][f45e0853]]
use regex::Regex;

fn rewrite_route_section(s: &str) -> Result<String> {
    assert!(s.starts_with("#"), "invalid route section: {s:?}");

    #[rustfmt::skip]
    let re = Regex::new(r"(?x)
(?i)XYG3               # XYG3, case insensitive
(?P<core>\(\w+\)){0,1} # XYG3(FULL) or XYG3(FC)
").unwrap();

    let mut reformed: String = re.replace(s, "B3LYP").to_lowercase();
    // iop(5/33=1) nosymm extraoverlay
    if !reformed.contains("nosymm") {
        reformed.push_str(" nosymm");
    }
    if !reformed.contains("extraoverlay") {
        reformed.push_str(" extraoverlay");
    }
    if !reformed.contains("iop(5/33=1)") {
        reformed.push_str(" iop(5/33=1)");
    }

    // turn on frozen core algorithm or not
    let caps = re.captures(s).unwrap();
    let frozen_core = if let Some(core_type) = caps.name("core") {
        match core_type.as_str().to_uppercase().as_str() {
            "(FULL)" => false,
            "(FC)" => true,
            x @ _ => {
                bail!("invalid syntax: {s:?}");
            }
        }
    } else {
        false
    };

    // write extra overlays
    writeln!(&mut reformed, "\n");
    // 'AE':['8/7=1,10=90/1;','9/16=-3/6;','6//8;'],
    // 'FC':['8/7=1,10=4/1;','9/16=-3/6;','6//8;']
    if frozen_core {
        writeln!(&mut reformed, "8/7=1,10=4/1;");
    } else {
        writeln!(&mut reformed, "8/7=1,10=90/1;");
    }
    writeln!(&mut reformed, "9/16=-3/6;");
    writeln!(&mut reformed, "6//8;");

    Ok(reformed)
}

#[test]
fn test_xdh_reform_route() {
    dbg!(rewrite_route_section("# XYG3/6-31g** test"));
    dbg!(rewrite_route_section("# xyg3(full)/6-31g** nosymm test"));
    dbg!(rewrite_route_section("# xyg3(fc)/6-31g** nosymm test"));
}
// f45e0853 ends here

// [[file:../../xo-tools.note::98726040][98726040]]
// gen basis set
fn rewrite_final_section(s: &str) -> Result<String> {
    let mut reformed = String::new();
    for line in s.lines() {
        // include external file
        if line.starts_with("@") {
            let path = &line[1..];
            info!("including external file: {path}");
            // NOTE: if path is relative, we could get into troulbe for current directory issue
            let txt = gut::fs::read_file(path)?;
            writeln!(&mut reformed, "{txt}");
        } else {
            writeln!(&mut reformed, "{line}");
        }
    }
    // write extra data in the end
    let reformed = format!("{}\n\n100\n205\n402\n", reformed.trim_end());

    Ok(reformed)
}

#[test]
fn test_xdh_gjf_rewrite_gen() -> Result<()> {
    let s = "N 0
6-311+G(3df,p)
****
@tests/files/Test008.H
";

    let x = rewrite_final_section(s)?;
    println!("{x}");

    Ok(())
}
// 98726040 ends here

// [[file:../../xo-tools.note::8b2a8f8c][8b2a8f8c]]
impl xDH {
    /// Rewrite Gaussian input `s` to make it suitable for XYG3 type calculation
    pub fn rewrite_gaussian_input(s: &str) -> Result<String> {
        let re = Regex::new(r"\n\s*\n").unwrap();
        let sections = re.split(s.trim()).collect_vec();
        let n = sections.len();
        if n < 3 {
            bail!("invalid Gaussian input: {s:?}");
        }

        let route = rewrite_route_section(&sections[0])?;
        let other: String = sections[1..n - 1].iter().map(|x| x.to_string()).collect();
        let gen = rewrite_final_section(dbg!(&sections[n - 1]))?;

        let s = format!("{route}\n{other}\n{gen}");

        Ok(s)
    }
}
// 8b2a8f8c ends here

// [[file:../../xo-tools.note::5a888014][5a888014]]
#[test]
fn test_rewrite_input() -> Result<()> {
    let f: &Path = "tests/files/Test008.gjf".as_ref();
    let s = gut::fs::read_file(f)?;

    // for path issue when including external file
    // std::env::set_current_dir(f.parent().unwrap());
    let x = xDH::rewrite_gaussian_input(&s)?;
    println!("reformed input\n{x}");

    Ok(())
}
// 5a888014 ends here
