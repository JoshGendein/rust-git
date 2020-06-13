use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use std::collections::HashMap;
use std::fs;
// use super::error::Error;


pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

pub enum Path {
    Blob(Blob),
    Tree(Tree),
}

pub struct Tree {
    pub path: PathBuf,
    pub hash: String,
    pub subtree: HashMap<String, Path>, 
}

#[derive(Clone)]
pub struct Blob {
    pub path: PathBuf,
    pub hash: String,
    pub data: Vec<u8>,
}

pub struct Commit {
    parent: Vec<Commit>,
    author: String,
    message: String,
    snapshot: Tree,
}

//TODO: References
// references = map<string, string>
// maps human-readable names to Sha-1 hash
// kind of what 'index' does but for everything (commits, branches, etc)

impl Blob {
    // TODO: zLib compress data.
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {

        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let mut sha = Sha1::new();
        sha.input(&bytes);

        Ok(
            Blob {
                path: path.to_path_buf(),
                hash: sha.result_str(),
                data: bytes,
            }
        )
    }
}

impl Tree {
    pub fn from_path(path: &PathBuf) -> io::Result<Tree> {

        let mut subtree = HashMap::new();

        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if !path.is_dir() {
                // Create a Blob
                let blob = Blob::from_path(&path)?;
                // Add to subtree
                subtree.insert(blob.hash.to_string(), Path::Blob(blob));
            }
            else {
                // Recurse to get subtree
                let tree = Tree::from_path(&path)?;
                // Add to subtree
                subtree.insert(tree.hash.to_string(), Path::Tree(tree));
            }
        }

        let mut sha = Sha1::new();
        sha.input(&Self::build_tree_file(&subtree)?);

        Ok(
            Tree {
                path: path.to_path_buf(),
                hash: sha.result_str(),
                subtree: subtree,
            }
        )
    }

    pub fn build_tree_file(subtree: &HashMap<String, Path>) -> io::Result<Vec<u8>> {

        let mut tree_file = String::new();

        for (hash, path) in subtree {
            match path {
                Path::Blob(blob) => {
                    tree_file.push_str(&format!("blob {}\t{}\n", hash, blob.path.to_str().unwrap()));
                },
                Path::Tree(tree) => {
                    tree_file.push_str(&format!("tree {}\t{}\n", hash, tree.path.to_str().unwrap()));
                }
            }
        }

        Ok(tree_file.into_bytes())
    }
}