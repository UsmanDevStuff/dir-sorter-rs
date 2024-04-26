use std::io;
use std::fs;
use std::collections::HashMap;
use std::path::Path;

use crate::sort_files;

pub fn sort(input_path: &str) -> io::Result<()> {
    if input_path.is_empty() {
        println!("Please provide directory path.");
        return Ok(());
    }
    // Define categories and their corresponding file extensions
    let categories: HashMap<&str, Vec<&str>> = [
        ("Images", vec!["jpg", "png", "gif", "bmp", "webp", "jpeg"]),
        ("Videos", vec!["mp4", "mkv", "avi", "mov", "wmv", "webm", "mpg"]),
        ("Documents", vec!["pdf", "doc", "docx", "txt", "odt", "rtf", "csv", "xls", "xlsx"]),
        ("Code", vec!["rs", "py", "js", "cpp", "c", "java", "css", "html", "xml", "sh"]),
        ("Archives", vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz"]),
        ("Executables", vec!["exe", "msi"]),
        ("ISO Files", vec!["iso"]),
        // Add more categories and extensions as needed
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, Vec<&str>>>();

    let exclude_folders = vec!["Images", "Videos", "Documents", "Code", "Archives", "Executables", "ISO Files", "Uncategorized", "Directories"];

    //let input_dir = "C:\\Users\\Muhammad_Usman\\Downloads";
    // Get input directory path from user
    //io::stdout().flush().unwrap();
    //let mut input_dir = String::new();
    //println!("Enter the directory path to sort: ");
    //io::stdin().read_line(&mut input_dir).unwrap();

    // Remove trailing newline character from user input //input_dir.pop();
    //let input_dir = input_dir.trim();
    let input_dir = input_path;

    let input_path = Path::new(&input_dir);
    sort_files::sort_files_by_category(input_path, &categories, exclude_folders)?;
    println!("Directory sorted successfully!");
    Ok(())
}

pub fn delete_empty_directory(dir: &Path) -> io::Result<()> {
    if fs::read_dir(dir)?.next().is_none() {
        fs::remove_dir(dir)?;
        println!("Deleted empty directory: {} \n", dir.display());
    }
    Ok(())
}