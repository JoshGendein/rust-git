use std::env;
use std::fs;
use std::fs::File;

use std::io;
use std::io::{Read, Write};
use std::path::{PathBuf, Path};

use super::types::{ Object, Tree };
use super::error::Error;

pub struct FileService {
    pub root_dir: PathBuf,
    pub gitr_dir: PathBuf,
    pub object_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Result<FileService, Error> {
        let root_dir = FileService::find_root()?;
        let gitr_dir = root_dir.join(".gitr").to_path_buf();
        let object_dir = gitr_dir.join("objects").to_path_buf();
    
        Ok(
            FileService {
                root_dir,
                gitr_dir,
                object_dir,
            }
        )
    }

    fn find_root() -> Result<PathBuf, Error> {
        // Starts at current directory and works its way up until it reaches root or None
        let mut current_dir = env::current_dir()?;
        loop {
            if FileService::is_gitr(&current_dir) {
                return Ok(current_dir)
            }
            if !current_dir.pop() {
                return Err(Error::NoDirectory)
            }
        }
    }

    fn is_gitr<P>(path: P) -> bool 
        where P: Sized + AsRef<Path>
    {
        path.as_ref().join(".gitr").exists()
    }

    pub fn get_head_ref(&self) -> io::Result<PathBuf> {
        let mut head_file = File::open(self.root_dir.join(".gitr/HEAD"))?;
        let mut ref_path = String::new();
        head_file.read_to_string(&mut ref_path)?;

        // Split off the first part "refs: "
        let ref_path = ref_path.split_off(6);
        Ok(self.gitr_dir.join(ref_path))
    }

    pub fn write_object(&self, object: &Object) -> io::Result<()> {
        // &self, hash: &str, data: &Vec<u8>
        match object {
            Object::Blob(blob) => {
                let blob_dir = self.object_dir.join(&blob.hash[..2]);
                if !blob_dir.exists() {
                    fs::create_dir(&blob_dir)?;
                }
                let blob_filename = blob_dir.join(&blob.hash[2..]);
                let mut blob_f = File::create(&blob_filename)?;
                blob_f.write_all(&blob.data)?;

                Ok(())
            },
            Object::Tree(tree) => {
                let tree_dir = self.object_dir.join(&tree.hash[..2]);
                if !tree_dir.exists() {
                    fs::create_dir(&tree_dir)?;
                }
                let tree_filename = tree_dir.join(&tree.hash[2..]);
                let mut tree_f = File::create(&tree_filename)?;

                tree_f.write_all(&Tree::build_tree_file(&tree.subtree)?)?;

                Ok(())
            },
            Object::Commit(Commit) => {

                Ok(())
            },
        }
    }

    pub fn read_object(&self, object: &Object) -> io::Result<()> {
        match object {
            Object::Blob(Blob) => {
            
                Ok(())
            },
            Object::Tree(Tree) => {

                Ok(())
            },
            Object::Commit(Commit) => {

                Ok(())
            },
        }
    }
}