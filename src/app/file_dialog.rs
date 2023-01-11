use eframe;

use super::{FileDialog, Theme};
use ron;

pub fn file_dialog(file_dialog: &mut FileDialog, themes: &mut Vec<Theme>, selected_theme: usize, ctx: &eframe::egui::Context) {
    let mut close = false;
    match file_dialog {
        FileDialog::Save(path) => {
            eframe::egui::Window::new("Save theme").show(ctx, |ui| {
                ui.add(eframe::egui::TextEdit::singleline(path));
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        std::fs::write(path, ron::to_string(&themes[selected_theme]).unwrap());
                        close = true;
                    }
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });
        } 
        FileDialog::JustVisuals(path) => {
            eframe::egui::Window::new("Save theme").show(ctx, |ui| {
                ui.add(eframe::egui::TextEdit::singleline(path));
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        std::fs::write(path, ron::to_string(&themes[selected_theme].visuals).unwrap());
                        close = true;
                    }
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });
        }
        FileDialog::Select(path) => {
            eframe::egui::Window::new("Select theme").show(ctx, |ui| {
                ui.add(eframe::egui::TextEdit::singleline(path));
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        if let Ok(file) = std::fs::read_to_string(path) {
                            let theme: Theme = ron::from_str(file.as_str()).unwrap();
                            themes.push(theme);
                            close = true;
                        }
                    }
                    if ui.button("Close").clicked() {
                        close = true;
                    }
                });
            });
        }
        FileDialog::None => {}
    }
    if close == true {
        *file_dialog = FileDialog::None
    }
}