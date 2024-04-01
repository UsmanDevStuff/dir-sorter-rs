// disable console on windows
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use eframe::egui;
use eframe::egui::CursorIcon;

use rfd::FileDialog;

#[derive(Default)]
struct SorterApp {
    text: String,
}

impl eframe::App for SorterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.label("Enter Full Directory Path:");
                ui.add_space(5.0);
            });

            ui.vertical_centered(|ui| {
                ui.text_edit_singleline(&mut self.text);
                ui.add_space(4.0);
                if ui.button("Select Directory").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.text = path.display().to_string();
                    }
                }
            });
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                if ui
                    .button("Sort Directory")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    // Handle button click here (e.g., print the text)
                    println!("Input Directory Path: {}", self.text);
                    sort(&self.text).ok().expect("sort function failed");
                    println!("Directory sorted successfully!");
                }
            });
            //let _dir = self.text.as_str();
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 170.0]),
        ..Default::default()
    };
    // Fix: Pass the closure with CreationContext argument
    let _ = eframe::run_native(
        "Auto Directory Sorter",
        native_options,
        Box::new(|_cc| Box::new(SorterApp::default())),
    );
}

fn sort(input_path: &str) -> io::Result<()> {
    // Define categories and their corresponding file extensions
    let categories: HashMap<&str, Vec<&str>> = [
        ("Images", vec!["jpg", "png", "gif", "bmp", "webp", "jpeg"]),
        ("Videos", vec!["mp4", "mkv", "avi", "mov", "wmv", "webm", "mpg"]),
        ("Documents", vec!["pdf", "doc", "docx", "txt", "odt", "rtf", "csv", "xls", "xlsx"]),
        ("Code", vec!["rs", "py", "js", "cpp", "c", "java", "css", "html", "xml", "sh"]),
        ("Archives", vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz"]),
        ("Executables", vec!["exe", "msi"]),
        ("ISO files", vec!["iso", "dmg"]),
        // Add more categories and extensions as needed
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, Vec<&str>>>();

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
    sort_files_by_category(input_path, &categories)?;

    Ok(())
}

fn sort_files_by_category(
    input_path: &Path,
    categories: &HashMap<&str, Vec<&str>>,
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
                    println!("Moved uncategorized file: {} \n", path.display());
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

fn determine_category(path: &Path, categories: &HashMap<&str, Vec<&str>>) -> Option<String> {
    let ext = path.extension()?;
    let ext_str = ext.to_string_lossy().to_string();

    for (category, extensions) in categories {
        if extensions.iter().any(|&ext| ext == &*ext_str) {
            return Some(category.to_string());
        }
    }

    None
}

fn create_or_get_category_dir(category: &str, base_path: &Path) -> io::Result<PathBuf> {
    let category_dir = base_path.join(category);
    if !category_dir.exists() {
        fs::create_dir_all(&category_dir)?;
        println!("Created category directory: {} \n", category_dir.display());
    }
    Ok(category_dir)
}

fn delete_empty_directory(dir: &Path) -> io::Result<()> {
    if fs::read_dir(dir)?.next().is_none() {
        fs::remove_dir(dir)?;
        println!("Deleted empty directory: {} \n", dir.display());
    }
    Ok(())
}
