use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::io;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

pub struct Tree {
    pub hash: String,
    pub data: String,
}

pub struct Commit {
    pub hash: String,
    pub data: String,
    pub parents: Option<Vec<String>>,
    pub author: String,
    pub message: String,
    pub tree: String,
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

impl Tree {
    pub fn new(data: String) -> Self {
        let mut sha = Sha1::new();
        sha.input_str(&data);

        Tree {
            hash: sha.result_str(),
            data,
        }
    }
}

impl Commit {
    pub fn new(
            parent: Option<String>, 
            tree: String, 
            author: String,
             message: String) -> Commit {

        // Your parent is the commit where the HEAD is pointing
        // A commit has two parents in the case of a merge.
        // TODO: add support for multiple parents.
        let mut parent_refs = Vec::new();
        if let Some(parent_ref) = parent {
             parent_refs.push(parent_ref)
        }

        let mut commit_file = String::new();
        commit_file.push_str(&format!["tree {}\n", tree]);
        commit_file.push_str(&format!["author {}\n", author]);
        for parent in &parent_refs {
            commit_file.push_str(&format!["parent {}\n", parent]);
        }
        commit_file.push_str("\n");
        commit_file.push_str(&message);

        let mut sha = Sha1::new();
        sha.input_str(&commit_file);

        let commit = Commit {
            hash: sha.result_str(),
            data: commit_file,
            author: author,
            message: message,
            parents: Some(parent_refs),
            tree: tree,
        };

        return commit
    }
}