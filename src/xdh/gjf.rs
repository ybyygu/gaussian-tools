// [[file:../../xo-tools.note::*imports][imports:1]]
use super::*;
// imports:1 ends here

// [[file:../../xo-tools.note::07a8922b][07a8922b]]
/// rewrite at file using absolute path to avoid issue when current directory
/// changes
fn absolute_at_file_path(line: &str, relative: &Path) -> Option<String> {
    // ignore special path with env var, for example: @GAUSS_EXEDIR:mm2.prm
    if line.contains(":") {
        debug!("found env var in at command line, ignored: {line:?}");
        return Some(line.into());
    }

    let re = Regex::new(
        r"(?x)
^\s*@           # a line starts with @
(?P<path>\S+)   # path to external file
",
    )
    .unwrap();

    let caps = re.captures(line)?;
    let path = caps.name("path")?;

    // get absolute path relative to Gaussian input file
    let path = path.as_str();
    let absolute_path = relative.parent()?.join(path);
    line.replace(path, absolute_path.to_str()?).into()
}

#[test]
fn test_xdh_gjf_at_file() {
    let x = absolute_at_file_path("@Test008.H /N", "/path/to/Test008.gjf".as_ref()).unwrap();
    assert_eq!(x, "@/path/to/Test008.H /N");

    let x = absolute_at_file_path("@GAUSS_EXEDIR:mm2.prm", "/path/to/Test008.gjf".as_ref()).unwrap();
    assert_eq!(x, "@GAUSS_EXEDIR:mm2.prm");

    let x = absolute_at_file_path("@Test008.H ", "path/to/Test008.gjf".as_ref()).unwrap();
    assert_eq!(x, "@path/to/Test008.H ");
}
// 07a8922b ends here

// [[file:../../xo-tools.note::f45e0853][f45e0853]]
use regex::Regex;

/// Reference: https://gaussian.com/route/
fn rewrite_route_section(s: &str) -> Result<String> {
    // a pound sign (#) as the first non-blank character of a line
    if !s.trim_start().starts_with("#") {
        bail!("invalid route section: {s:?}");
    }

    // turn on additional output: "#P"
    let re = Regex::new(r"^(?i)\s*#[NPT]{0,1}").unwrap();
    let reformed = re.replace(s, "#p");

    #[rustfmt::skip]
    let re = Regex::new(r"(?x)
(?i)XYG3               # XYG3, case insensitive
(?P<core>\(\w+\)){0,1} # XYG3(FULL) or XYG3(FC)
").unwrap();

    let mut reformed: String = re.replace(reformed.trim_end(), "B3LYP").to_lowercase();
    // iop(5/33=1) nosymm extraoverlay
    if !reformed.contains("iop(5/33=1)") {
        reformed.push_str(" iop(5/33=1)");
    }
    if !reformed.contains("nosymm") {
        reformed.push_str(" nosymm");
    }
    if !reformed.contains("extraoverlay") {
        reformed.push_str(" extraoverlay");
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
#[rustfmt::skip]
fn test_xdh_reform_route() -> Result<()> {
    let x = rewrite_route_section("# XYG3/6-31g")?;
    assert_eq!(x, "#p b3lyp/6-31g iop(5/33=1) nosymm extraoverlay\n\n8/7=1,10=90/1;\n9/16=-3/6;\n6//8;\n");

    let x = rewrite_route_section("# xyg3(full)/6-31g** nosymm test")?;
    assert_eq!(x, "#p b3lyp/6-31g** nosymm test iop(5/33=1) extraoverlay\n\n8/7=1,10=90/1;\n9/16=-3/6;\n6//8;\n");
    let x = rewrite_route_section("# xyg3(fc)/6-31g** nosymm test")?;
    assert_eq!(x, "#p b3lyp/6-31g** nosymm test iop(5/33=1) extraoverlay\n\n8/7=1,10=4/1;\n9/16=-3/6;\n6//8;\n");

    Ok(())
}
// f45e0853 ends here

// [[file:../../xo-tools.note::98726040][98726040]]
// gen basis set
fn rewrite_final_section(s: &str) -> Result<String> {
    // write extra data in the end
    let reformed = format!("{}\n\n100\n205\n402\n", s.trim_end());

    Ok(reformed)
}
// 98726040 ends here

// [[file:../../xo-tools.note::8b2a8f8c][8b2a8f8c]]
impl xDH {
    /// Rewrite Gaussian input stream `f` to make it suitable for XYG3 type
    /// calculation. If f is None, it will read from stdin. The reformed stream
    /// will be printed in stdout.
    pub fn rewrite_gaussian_input<'a>(f: impl Into<Option<&'a Path>>) -> Result<String> {
        if let Some(f) = f.into() {
            let s = file_reader(f)?;
            Self::rewrite_gaussian_input_from(s)
        } else {
            let s = std::io::stdin().lock();
            Self::rewrite_gaussian_input_from(s)
        }
    }

    fn read_gaussian_input_from(s: impl BufRead) -> Result<String> {
        let mut reformed = String::new();
        for line in s.lines() {
            let line = line?;
            if line.trim_start().starts_with("@") {
                debug!("found at file: {line}");
            }
            writeln!(&mut reformed, "{line}");
        }

        Ok(reformed)
    }

    /// Read gaussian input from file, try to:
    ///
    /// 1. avoid Windows/Unix line ending issue
    ///
    /// 2. avoid relative at file issue
    ///
    fn read_gaussian_input(f: &Path) -> Result<String> {
        let f = file_reader(f)?;
        Self::read_gaussian_input_from(f)
    }

    /// Rewrite Gaussian input file `s` to make it suitable for XYG3 type
    /// calculation
    fn rewrite_gaussian_input_from(f: impl BufRead) -> Result<String> {
        let s = Self::read_gaussian_input_from(f)?;
        let re = Regex::new(r"\n\s*\n").unwrap();
        let sections = re.split(s.trim()).collect_vec();
        let n = sections.len();
        if n < 3 {
            bail!("invalid Gaussian input: {s:?}");
        }

        // link0 lines are the lines starting with "%" char
        // for route lines, the first line must be starting with "#"
        let mut lines = sections[0].lines();
        let mut link0 = String::new();
        let mut route = String::new();
        while let Some(line) = lines.next() {
            if line.starts_with("%") {
                writeln!(&mut link0, "{line}");
            } else {
                writeln!(&mut route, "{line}");
            }
        }

        let route = rewrite_route_section(&route)?;
        let other = sections[1..n - 1].join("\n\n");

        let gen = rewrite_final_section(&sections[n - 1])?;

        let s = format!("{link0}{route}\n{other}\n\n{gen}");

        Ok(s)
    }
}
// 8b2a8f8c ends here

// [[file:../../xo-tools.note::5a888014][5a888014]]
#[test]
fn test_rewrite_input() -> Result<()> {
    let f: &Path = "tests/files/Test009.gjf".as_ref();
    let x = xDH::rewrite_gaussian_input(f)?;
    println!("reformed input\n{x}");

    let f: &Path = "tests/files/Test008.gjf".as_ref();
    let x = xDH::rewrite_gaussian_input(f)?;

    let x_expected = gut::fs::read_file("tests/files/Job_Test008.com")?;
    assert_eq!(x, x_expected);

    Ok(())
}
// 5a888014 ends here
