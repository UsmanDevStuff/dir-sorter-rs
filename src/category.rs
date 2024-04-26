use std::io;
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn determine_category(path: &Path, categories: &HashMap<&str, Vec<&str>>) -> Option<String> {
    let ext = path.extension()?;
    let ext_str = ext.to_string_lossy().to_string();

    for (category, extensions) in categories {
        if extensions.iter().any(|&ext| ext == &*ext_str) {
            return Some(category.to_string());
        }
    }

    None
}

pub fn create_or_get_category_dir(category: &str, base_path: &Path) -> io::Result<PathBuf> {
    let category_dir = base_path.join(category);
    if !category_dir.exists() {
        fs::create_dir_all(&category_dir)?;
        println!("Created category directory: {} \n", category_dir.display());
    }
    Ok(category_dir)
}