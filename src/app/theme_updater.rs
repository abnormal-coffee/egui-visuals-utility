use eframe;
use std::{hash::Hasher, collections::hash_map::DefaultHasher};
use bincode;
use ron::{self, error::SpannedError};
use home;

fn hash_theme(theme: eframe::egui::Visuals) -> u64 {
    let bin: Vec<u8> = bincode::serialize(&theme).unwrap();
    let mut hasher = DefaultHasher::new();
    hasher.write(bin.as_slice());
    return hasher.finish();
}

pub fn update_theme(visuals: eframe::egui::Visuals, hash: &mut u64) -> bool {
    let newhash = hash_theme(visuals.clone());
    if hash.clone() != newhash {
        *hash = newhash;
        let home = home::home_dir().unwrap().to_str().unwrap().to_string();
        if let Err(err) = std::fs::write(format!("{}/.egui-theme", home), ron::to_string(&visuals).unwrap()) {println!("{}", err)};
        return true;
    }
    else {
        return false;
    }
}

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
