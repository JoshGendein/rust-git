use std::env;
use std::fs;
use std::fs::File;

use std::io;
use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use super::error::Error;

pub struct FileService {
    pub root_dir: PathBuf,
    pub gitr_dir: PathBuf,
    pub object_dir: PathBuf,
    pub abs_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Result<FileService, Error> {
        let root_dir = FileService::find_root()?;
        let gitr_dir = root_dir.join(".gitr").to_path_buf();
        let object_dir = gitr_dir.join("objects").to_path_buf();
        let abs_dir = env::current_dir()?;
    
        Ok(
            FileService {
                root_dir,
                gitr_dir,
                object_dir,
                abs_dir,
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

    pub fn write_object(&self, hash: &str, data: &Vec<u8>) -> Result<(), Error> {
        // Check if first two character of hash dir exists.
        let dir = self.object_dir.join(&hash[..2]);
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        // Write data to file with name last 38 characters of hash.
        let filename = dir.join(&hash[2..]);
        let mut object_file = File::create(&filename)?;
        object_file.write_all(&data)?;
        Ok(())
    }

    pub fn get_all_files_in_dir(&self, dir: &PathBuf) -> io::Result<Vec<PathBuf>> {

        let mut files: Vec<PathBuf> = vec![];

        self.visit_dirs(dir, &mut files)?;

        return Ok(files);
    }

    fn visit_dirs(&self, dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if self.is_ignored(&path) {
                    continue;
                }
                if path.is_dir() {
                    self.visit_dirs(&path, files)?;
                } else {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    pub fn is_ignored (&self, entry: &PathBuf) -> bool {
        // TODO: Check .gitrignore file.
        // This is a terrible method but it works for now.
        let entry = entry.to_path_buf();
        let mut cmp_dir = PathBuf::from("");
        let abs_path = self.abs_dir.to_path_buf();
        cmp_dir.push(abs_path);
        cmp_dir.push(".");
        cmp_dir.push("target");
        if entry == cmp_dir { return true };
        cmp_dir.pop();
        cmp_dir.push(".gitr");
        if entry == cmp_dir { return true };
        cmp_dir.pop();
        cmp_dir.push(".git");
        if entry == cmp_dir { return true };

        return false;
    }
}