use std::io;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::path::PathBuf;

use super::error::Error;

/// Index includes all files that have been added. i.e. What we are tracking

pub struct Index {
    pub path: PathBuf,
    pub hashtree: BTreeMap<String, String>,
}

impl Index {
    pub fn new(root_dir: &PathBuf) -> Result<Index, Error> {
        let mut index = Index {
            path: root_dir.join(".gitr").join("index"),
            hashtree: BTreeMap::new(),
        };
        if !index.path.exists() {
            return Ok(index);
        }
        // Build BTreeMap of the files from the index file.
        let file = BufReader::new(File::open(&index.path)?);
        for line in file.lines() {
            let ln = line?;
            let blob: Vec<&str> = ln.split(' ').collect();
            if blob.len() != 2 {
                return Err(Error::InvalidIndex);
            }
            index.update(blob[1], blob[0]);
        }

        Ok(index)
    }

    pub fn update(&mut self, path: &str, hash: &str) {
        self.hashtree.insert(path.to_string(), hash.to_string());
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = File::create(&self.path)?;
        for (ref hash, ref path) in self.hashtree.iter() {
            writeln!(&mut index, "{} {}", hash, path)?;
        }
        Ok(())
    }
}
