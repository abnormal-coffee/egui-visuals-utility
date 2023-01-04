use eframe;
use ron::{self, error::SpannedError};
use home;

// this could probably be made nicer
pub fn load_theme(ctx: &eframe::egui::Context) {
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
