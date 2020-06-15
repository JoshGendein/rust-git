use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::io;
use std::collections::BTreeMap;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

pub struct Commit {
    hash: Option<String>,
    parent: Option<Vec<Commit>>,
    author: Option<String>,
    message: Option<String>,
    snapshot: BTreeMap<String, String>,
}

impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let mut sha = Sha1::new();
        sha.input(&bytes);

        Ok(
            Blob {
                hash: sha.result_str(),
                data: bytes,
            }
        )
    }
}