use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::category::{create_or_get_category_dir, determine_category};
use crate::sort::delete_empty_directory;

pub fn sort_files_by_category(
    input_path: &Path,
    categories: &HashMap<&str, Vec<&str>>,
    exclude_folders: Vec<&str>,
) -> io::Result<()> {
    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Determine the category based on file extension
            let category = determine_category(path.as_path(), categories);

            let category_dir = match &category {
                Some(category) => create_or_get_category_dir(&category, input_path)?,
                None => {
                    // Handle uncategorized files (optional)
                    // e.g., move to a separate "uncategorized" directory
                    let uncategorized_dir = input_path.join("Uncategorized");
                    if !uncategorized_dir.exists() {
                        fs::create_dir_all(&uncategorized_dir)?;
                    }
                    let new_path = uncategorized_dir.join(path.file_name().unwrap());
                    fs::rename(&path, new_path)?;
                    println!("Moved Uncategorized file: {} to 'Uncategorized'\n", path.display());
                    continue;
                }
            };

            // Move the file to the category directory
            fs::rename(&path, category_dir.join(path.file_name().unwrap()))?;
            println!(
                "Moved file {} to category {} \n",
                path.display(),
                category.unwrap_or("Uncategorized".to_string())
            );
        }
        if !fs::read_dir(&path)?.next().is_none() {
            let filename = path.file_name();
            match filename {
                Some(name) => {
                    let folder_name = name.to_str();
                    if let Some(file_name) = folder_name {
                        if !exclude_folders.contains(&file_name) {
                            // Handle folders (optional)
                            // e.g., move to a separate "Folders" directory
                            let folder_dir = input_path.join("Directories");
                            if !folder_dir.exists() {
                                fs::create_dir_all(&folder_dir)?;
                            }
                            let new_path = folder_dir.join(file_name);
                            fs::rename(&path, new_path)?;
                            println!("Moved folder: {} to 'Directories' \n", path.display());
                        }
                    } else {
                        // Handle the case where conversion fails
                        println!("Could not convert OsStr to valid &str");
                    }
                }
                None => println!("Error: error occured in MATCH filename."),
            }
        } else if path.is_dir() {
            // Recursively process subdirectories
            //println!("Recursing into directory: {} \n", path.display());
            //sort_files_by_category(&path, categories)?;
            // Check for and delete empty directories (optional)
            delete_empty_directory(&path)?;
        }
    }

    Ok(())
}
