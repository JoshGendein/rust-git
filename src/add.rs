use super::file::FileService;
use super::index::Index;
use super::error::Error;
use super::types::Blob;

pub fn add_all(paths: &Vec<&str>) -> Result<(), Error> {
    let file_service = FileService::new()?;
    let mut index = Index::new(&file_service.root_dir)?;

    for path in paths {
        let full_path = file_service.abs_dir.join(path);
        if full_path.is_file() {
            let blob = Blob::from_path(&full_path)?;
            file_service.write_object(&blob.hash, &blob.data)?;
            index.update(&full_path.to_str().unwrap(), &blob.hash);
        }
        else {
            let nested_files = file_service.get_all_files_in_dir(&full_path)?;
            for file_path in nested_files {
                let blob = Blob::from_path(&file_path)?;
                file_service.write_object(&blob.hash, &blob.data)?;
                index.update(&full_path.to_str().unwrap(), &blob.hash);
            }
        }
    }

    Ok(())
}

