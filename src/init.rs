use super::error::Error;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn init() -> Result<(), Error> {
    let dir = Path::new(".gitr");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    let mut head = File::create(dir.join("HEAD"))?;
    head.write_all("refs: refs/heads/main".as_bytes())?;
    Ok(())
}