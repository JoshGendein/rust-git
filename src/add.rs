use std::env;

use super::file::FileService;
use super::types::{ Object, Blob, Tree, Path };
use super::index::Index;
use super::error::Error;

pub fn add_all(paths: &Vec<&str>) -> Result<(), Error> {
    let file_service = FileService::new()?;
    let current_dir = env::current_dir()?;
    let mut index = Index::new(&file_service.root_dir)?;

    for path in paths {
        let full_path = current_dir.join(path); 
        if full_path.is_file() {
            let blob = Blob::from_path(&full_path)?;
            add_to_index(&Path::Blob(blob), &mut index, &file_service)?;
        }
        else {
            let tree = Tree::from_path(&full_path)?;
            add_to_index(&Path::Tree(tree), &mut index, &file_service)?;
        }  
    }

    index.write()?;
    Ok(())
}

pub fn add_to_index(path: &Path, index: &mut Index, file_service: &FileService) -> Result<(), Error> {
    
    match path {
        Path::Blob(blob) => {
            let relative_path = blob.path.strip_prefix(&file_service.root_dir).unwrap();
            index.update(&relative_path.to_str().unwrap(), &blob.hash);
            file_service.write_object(&Object::Blob(blob.to_owned()))?;
        }
        Path::Tree(tree) => {
            for path in tree.subtree.values() {
                add_to_index(path, index, file_service)?;
            }
        }
    }

    Ok(())
}
