use eframe::{self, egui::Context};
use serde;

use super::{Theme, Mode};

// Once the rest of the app is done a simplified theme creator wizard should be created here which would map a few basic colours and other parameters to Visuals.

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub struct Settings {
    name: String,
    description: String,
    author: String,
    background: eframe::epaint::Color32,
    widgets: eframe::epaint::Color32,
    text: eframe::epaint::Color32,
    expansion: f32,
    rounding: f32,
    scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            author: String::new(),
            background: eframe::epaint::Color32::default(),
            widgets: eframe::epaint::Color32::default(),
            text: eframe::epaint::Color32::default(),
            expansion: 1.,
            rounding: 4.,
            scale: 1.,
        }
    }
}

impl Settings {
    fn convert_to_visuals(input: Settings) -> Theme {
        Theme {
            name: input.name,
            author: input.author,
            description: input.description,
            visuals: eframe::egui::Visuals {
                window_rounding: eframe::egui::Rounding::from(input.rounding),
                window_fill: input.background,
                panel_fill: input.background,
                override_text_color: Some(input.text),
                widgets: eframe::egui::style::Widgets {
                    noninteractive: eframe::egui::style::WidgetVisuals {
                        bg_stroke: eframe::egui::Stroke{
                            width: input.scale * 2.5,
                            color: input.widgets,
                        },
                        rounding: eframe::egui::Rounding::from(input.rounding),
                        fg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::WHITE,
                        },
                        bg_fill: eframe::egui::Color32::BLACK,
                        expansion: 0.,
                    },
                    inactive: eframe::egui::style::WidgetVisuals {
                        bg_stroke: eframe::egui::Stroke{
                            width: 0.,
                            color: input.widgets,
                        },
                        rounding: eframe::egui::Rounding::from(input.rounding),
                        fg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::WHITE,
                        },
                        bg_fill: eframe::egui::Color32::BLACK,
                        expansion: 0.,
                    },
                    hovered: eframe::egui::style::WidgetVisuals {
                        bg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::BLACK,
                        },
                        rounding: eframe::egui::Rounding::from(input.rounding),
                        fg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::WHITE,
                        },
                        bg_fill: input.widgets,
                        expansion: 1. * input.expansion,
                    },
                    active: eframe::egui::style::WidgetVisuals {
                        bg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::BLACK,
                        },
                        rounding: eframe::egui::Rounding::from(input.rounding),
                        fg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::WHITE,
                        },
                        bg_fill: input.widgets,
                        expansion: 1.5 * input.expansion,
                    },
                    open: eframe::egui::style::WidgetVisuals {
                        bg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::BLACK,
                        },
                        rounding: eframe::egui::Rounding::from(input.rounding),
                        fg_stroke: eframe::egui::Stroke{
                            width: 1. * input.scale,
                            color: eframe::egui::Color32::WHITE,
                        },
                        bg_fill: input.widgets,
                        expansion: 2. * input.expansion,
                    }
                },
                ..Default::default()
            }
        }
    }
}

pub fn simplified(simplified: &mut Settings, themes: &mut Vec<Theme>, ctx: &Context, mode: &mut Mode) {
    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                
            ui.add(eframe::egui::TextEdit::singleline(&mut simplified.name).hint_text("Theme Name"));
            ui.add(eframe::egui::TextEdit::singleline(&mut simplified.author).hint_text("Author Name"));
            ui.add(eframe::egui::TextEdit::multiline(&mut simplified.description).hint_text("Description"));
            
            ui.separator();
            
            colour_picker(&mut simplified.background, ui);
            
            colour_picker(&mut simplified.widgets, ui);
                
            colour_picker(&mut simplified.text, ui);
            
            ui.separator();
                
            ui.label("Expansion");
            ui.add(eframe::egui::Slider::new(&mut simplified.expansion, 0.0..=4.0));
                
            ui.label("Frame Rounding");
            ui.add(eframe::egui::Slider::new(&mut simplified.rounding, 0.0..=10.0));
            
            ui.separator();
            
            if ui.button("Finish").clicked() {
                themes.push(Settings::convert_to_visuals(simplified.clone()));
                *mode = Mode::Preview
            }
        });
    });
}

fn colour_picker(colour: &mut eframe::epaint::Color32, ui: &mut eframe::egui::Ui) {
    ui.horizontal(|ui| {
        eframe::egui::color_picker::color_edit_button_srgba(ui, colour, eframe::egui::color_picker::Alpha::BlendOrAdditive);
        for c in 0..4 {
            ui.add(eframe::egui::DragValue::new(&mut colour[c]));
        }
    });
}