use super::file::FileService;
use super::types::Commit;
use super::error::Error;
use super::index::Index;

pub fn commit() -> io::Result<Commit, Error> {
    // Find all files that have been added
    // Create a commit snapshot

    let file_service = FileService::new()?;
    let current_dir = env::current_dir()?;
    let mut index = Index::new(&file_service.root_dir)?;
}