// disable console on windows
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use eframe::egui::CursorIcon;

use rfd::FileDialog;

mod sort;
mod sort_files;
mod category;

//#[derive(Default)]
struct SorterApp {
    text: String,
    current_page: CurrentPage,
}

enum CurrentPage {
    Sorter,
    About,
}

impl Default for SorterApp {
    fn default() -> Self {
        SorterApp {
            text: String::default(),
            current_page: CurrentPage::Sorter,
        }
    }
}

impl eframe::App for SorterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let mut current_page = CurrentPage::Sorter;

        egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
         // Add buttons or other UI elements to switch pages
        ui.horizontal(|ui| {
            if ui.button("Sorter").clicked() {
                self.current_page = CurrentPage::Sorter;
            }
            if ui.button("About").clicked(){
                self.current_page = CurrentPage::About;
            }
        });
    });

        match self.current_page {
            CurrentPage::Sorter => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(15.0);
                        ui.label("Enter Full Directory Path:");
                        ui.add_space(5.0);
                    });
        
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.text);
                            ui.add_space(4.0);
                            if ui.button("Select Directory").clicked() {
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    self.text = path.display().to_string();
                            }
                        }
                        });
                        
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
                            sort::sort(&self.text).ok().expect("sort function failed OR a file/folder is in use, close it and run again.");
                        }
                    });
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.separator();
                        ui.label("Made with ❤️ by Muhammad Usman");
                    });
                    //let _dir = self.text.as_str();
                });
            },
            CurrentPage::About => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(15.0);
                        ui.label("About");
                        ui.add_space(10.0);
                        ui.label("Me:");
                        ui.label("Muhammad Usman");
                        ui.add_space(5.0);
                        ui.hyperlink("https://github.com/usmandevstuff");
                    });
                });
            }
        }
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 180.0]).with_resizable(false),
        ..Default::default()
    };
    // Fix: Pass the closure with CreationContext argument
    let _ = eframe::run_native(
        "Auto Directory Sorter",
        native_options,
        Box::new(|_cc| Box::new(SorterApp::default())),
    );
}