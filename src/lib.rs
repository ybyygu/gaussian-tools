// [[file:../xo-tools.note::95fd6309][95fd6309]]
use gut::prelude::*;
use std::io::BufRead;
use std::path::{Path, PathBuf};
// 95fd6309 ends here

// [[file:../xo-tools.note::218d7576][218d7576]]
pub mod cli;
// 218d7576 ends here

// [[file:../xo-tools.note::d4c45061][d4c45061]]
// re-exports
pub fn file_reader<P: AsRef<Path>>(f: P) -> Result<impl BufRead> {
    let f = f.as_ref();
    let f = std::fs::File::open(f).with_context(|| format!("Could not read file {:?}", f))?;

    let f = std::io::BufReader::new(f);
    Ok(f)
}
// d4c45061 ends here
