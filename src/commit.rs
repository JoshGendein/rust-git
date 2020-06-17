use super::file::FileService;
use super::types::{ Tree, Commit };
use super::error::Error;
use super::index::Index;
use std::collections::{ BTreeMap, HashMap, HashSet };

pub fn commit() -> Result<(), Error> {

    let file_service = FileService::new()?;
    let index = Index::new(&file_service.root_dir)?;


    let file_service = FileService::new()?;
    let mut paths_ordered_by_depth: BTreeMap<usize, HashSet<String>> = BTreeMap::new();
    let mut dir_mapped_to_children: HashMap<String, HashSet<String>> = HashMap::new();

    // Iterate over every file in the staging area. (i.e. index)
    for (path, _ ) in index.hashtree.iter() {
        // split path into its levels
        let mut levels = path.split("\\").collect::<Vec<&str>>();
        'inner: while levels.len() > 0 {
            let current_path = levels.join("\\");
            levels.pop();
            let parent = levels.join("\\");

            // Get the list of our parents children. Otherwise create one.
            let dir_mapping = dir_mapped_to_children.entry(parent).or_default();

            // Check if we are already inserted into our parents list
            // (i.e.) Another file already created path back to root from our parent 
            if dir_mapping.contains(&current_path) {
                break 'inner;
            }
            // Insert ourselves into our parents list of children.
            // Insert into into list at our current depth.
            let same_depth = paths_ordered_by_depth.entry(levels.len() + 1).or_default();
            same_depth.insert(current_path.clone());
            dir_mapping.insert(current_path);
        }
    }

    let mut trees: HashMap<String, Tree> = HashMap::new();

    // Iterate over paths in order of depth decreasing.
    for (_, paths) in paths_ordered_by_depth.iter().rev() {
        for path in paths {
            if dir_mapped_to_children.contains_key(path) {
                let tree_file = create_tree_file(&index, &dir_mapped_to_children[path], &mut trees);
                trees.insert(path.to_string(), Tree::new(tree_file));
            }
        }
    }

    // Special Case for Root
    let roots_children = &dir_mapped_to_children[""];
    let root_tree_file = create_tree_file(&index, &roots_children, &mut trees);
    trees.insert(String::from("\\"), Tree::new(root_tree_file));

    for (_, tree) in &trees {
        let data = tree.data.clone();
        file_service.write_object(&tree.hash, &data.into_bytes())?;
    }

    let parent_ref = file_service.get_head_ref();

    let commit = Commit::new(parent_ref, trees["\\"].hash.clone(), 
                            "William Shakespeare".to_string(), 
                            "Example Commit Message".to_string());

    file_service.write_head_ref(&commit.hash)?;
    file_service.write_object(&commit.hash, &commit.data.into_bytes())?;

    return Ok(())
}
pub fn create_tree_file(index: &Index, 
        children: &HashSet<String>,
        trees: &mut HashMap<String, Tree>) -> String {

    let mut tree_file = String::new();

    for path in children {
        // Check if you are a file or dir.
        if let Some(file_hash) = index.hashtree.get(path) {
            let line = format!("blob {} {}\n", file_hash, path);
            tree_file.push_str(&line);
        }
        else if let Some(tree) = trees.get(path) {
            let line = format!("tree {} {}\n", tree.hash, path);
            tree_file.push_str(&line);
        }
    }

    return tree_file;
}