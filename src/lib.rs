// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/xo-tools.rs/xo-tools.note::*lib.rs][lib.rs:1]]
use std::path::Path;
use std::io::BufRead;

pub use quicli::prelude::*;
pub type Result<T> = ::std::result::Result<T, Error>;

pub fn file_reader<P: AsRef<Path>>(f: P) -> Result<impl BufRead> {
    let f = f.as_ref();
    let f = std::fs::File::open(f).map_err(|e| {
        error!("Failed to open {}", f.display());
        e
    })?;

    let f = std::io::BufReader::new(f);
    Ok(f)
}
// lib.rs:1 ends here
