use eframe;
use home;
use ron::{self, error::SpannedError};

use super::{Mode, Theme, simplified::Settings};

pub fn preview(mode: &mut Mode, themes: &mut Vec<Theme>, selected_theme: &mut usize, ctx: & eframe::egui::Context) {
    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        let mut remove: (bool, usize) = (false, 0);
        for (i, theme) in themes.iter().enumerate() {
            if i == 0 || i == 1 {
                let frame = eframe::egui::Frame::none().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.collapsing(format!("{} - {}", theme.name.clone(), i) ,|ui| {
                            ui.label(theme.author.clone())
                        });
                    });
                });
                if frame.response.interact(eframe::egui::Sense::click()).clicked() {
                    *selected_theme = i;
                };
                if frame.response.interact(eframe::egui::Sense::click()).double_clicked() {
                    *mode = Mode::Editor;
                    *selected_theme = i;
                };
            }
            else {
                let frame = eframe::egui::Frame::none().show(ui, |ui| {
                    ui. horizontal(|ui| {
                        ui.collapsing(format!("{} - {}", theme.name.clone(), i) ,|ui| {
                            ui.label(format!("Author: {}", theme.author));
                            if ui.button("Remove").clicked() {
                                
                                remove = (true, i);
                            };
                        })
                    });
                });
                if frame.response.interact(eframe::egui::Sense::click()).clicked() {
                    *selected_theme = i;
                };
                if frame.response.interact(eframe::egui::Sense::click()).double_clicked() {
                    *mode = Mode::Editor;
                    *selected_theme = i;
                };
            }
        }
        if ui.button("New Theme").clicked() {
            themes.push(Theme::default())
        }
        if ui.button("Theme Wizard").clicked() {
            *mode = Mode::Simplified(Settings::default());
        }
        
        ui.collapsing("Visuals struct as text", |ui| {
            let mut temp = &mut format!("{:?}", themes[selected_theme.clone()].visuals).clone();
            ui.add(eframe::egui::TextEdit::multiline(temp).desired_width(1000.));
        });
        
        if remove.0 == true {
            *selected_theme = remove.1 - 1;
            themes.remove(remove.1);
        }
    });
}

fn load_theme(ctx: &eframe::egui::Context) {
    println!("Attempting to load theme");
    let path = format!("{}/.egui-theme", home::home_dir().unwrap().to_str().unwrap());
    println!("Path set to: {}", path);
    if let Ok(file) = std::fs::read_to_string(path) {
        println!("File succesfully loaded");
        let theme: Result<eframe::egui::Visuals, SpannedError>  = ron::from_str(file.as_str());
        if let Ok(theme) = theme.clone() {
            ctx.set_visuals(theme);
            println!("Set theme")
        }
        if let Err(_) = theme.clone() {
            println!("An error occurred loading the theme");
        }
    }
}
