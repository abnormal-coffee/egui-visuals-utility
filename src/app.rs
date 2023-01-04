use eframe::{self};
use serde;
use home;

mod preview;
mod editor;
mod demo_widgets;
mod simplified;
mod file_dialog;
mod load_theme;


#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum Mode {
    Preview,
    Editor,
    Simplified
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Preview
    }
}


#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Theme {
    name: String,
    author: String,
    description: String,
    visuals: eframe::egui::Visuals,
    scale: f64,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: String::from("New Theme"),
            author: String::from("Author"),
            description: String::from("Description"),
            visuals: eframe::egui::Visuals::default(),
            scale: 1.,
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
pub enum FileDialog {
    Save(String),
    Select(String),
    None,
}

impl Default for FileDialog {
    fn default() -> Self {
        FileDialog::None
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoApp {
    #[serde(skip)]
    window_open: bool,
    #[serde(skip)]
    slider: f64,
    #[serde(skip)]
    text_edit: String,
    demo_open: bool,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            window_open: false,
            slider: 0.0,
            text_edit: String::from("Editable string \nNewlines will only display in a multiline textbox"),
            demo_open: true,
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {
    demo: DemoApp,
    selected_theme: usize,
    mode: Mode,
    themes: Vec<Theme>,
    file_dialog: FileDialog,
    eq: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            demo: DemoApp::default(),
            selected_theme: 0,
            mode: Mode::default(),
            themes: vec![
                Theme {
                    name: "egui default dark mode".to_string(),
                    author: "from standard egui crate".to_string(),
                    description: "The standard dark theme for egui, this just sets Visuals::dark()".to_string(),
                    visuals: eframe::egui::Visuals::dark(),
                    scale: 1.,
                },
                Theme {
                    name: "egui default light mode".to_string(),
                    author: "from standard egui crate".to_string(),
                    description: "The standard light theme for egui, this just sets Visuals::light()".to_string(),
                    visuals: eframe::egui::Visuals::light(),
                    scale: 1.,
                },
            ],
            file_dialog: FileDialog::default(),
            eq: Theme::default(),
        }
    }
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        load_theme::load_theme(& cc.egui_ctx);
        
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for AppState {
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self { eq, demo, mode, themes, selected_theme, file_dialog} = self;
        
        if eq != &mut themes[selected_theme.clone()] {
            ctx.set_visuals(themes[selected_theme.clone()].visuals.clone());
            *eq = themes[selected_theme.clone()].clone();
            load_theme::load_theme(ctx);
        }
        
        eframe::egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Add Theme").clicked() {
                        *file_dialog = FileDialog::Select(home::home_dir().unwrap().to_str().unwrap().to_string());
                    }
                    if ui.button("Save").clicked() {
                        *file_dialog = FileDialog::Save(home::home_dir().unwrap().to_str().unwrap().to_string());
                    }
                });
                
                ui.menu_button("Mode", |ui| {
                    ui.radio_value(mode, Mode::Preview, "Preview Themes");
                    ui.radio_value(mode, Mode::Editor, "Edit Theme");
                });
                ui.toggle_value(&mut demo.demo_open, "Demo Widgets");
            })
        });
        
        file_dialog::file_dialog(file_dialog, themes, selected_theme.clone(), ctx);
        
        if demo.demo_open == true {
            demo_widgets::demo_widgets(ctx, demo);
        }
        
        match mode {
            Mode::Preview => {
                preview::preview(mode, themes, selected_theme, ctx)
            }
            Mode::Editor => {
                editor::editor(&mut themes[selected_theme.clone()], ctx, selected_theme.clone())
            }
            Mode::Simplified => {
                
            }
        }
        
    }
}
