# egui-visuals-utility
A tool for creating egui Visuals (themes).
The code is rather messy and might crash but it seems to work.

To load the theme use something similar to this:
```
use eframe;
use ron::{self, error::SpannedError};
use home;

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
```
