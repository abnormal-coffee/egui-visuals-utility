use eframe;

mod widget_visuals;
mod selection;
mod colour;
mod shadow;
mod rounding;
mod stroke;

use super::Theme;

pub fn editor(theme: &mut Theme, ctx: & eframe::egui::Context, selected_theme: usize) {
    if selected_theme == 0 || selected_theme == 1 {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Please create a new theme to edit it")
        });
    }
    else {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                
                ui.add(eframe::egui::TextEdit::singleline(&mut theme.name).hint_text("Theme Name"));
                ui.add(eframe::egui::TextEdit::singleline(&mut theme.author).hint_text("Author Name"));
                ui.add(eframe::egui::TextEdit::multiline(&mut theme.description).hint_text("Description"));
                
                ui.separator();
                
                ui.collapsing("Widgets", |ui| {
                    ui.separator();
                    widget_visuals::widget_visuals(&mut theme.visuals.widgets.noninteractive, ui, "noninteractive");
                    widget_visuals::widget_visuals(&mut theme.visuals.widgets.inactive, ui, "inactive");
                    widget_visuals::widget_visuals(&mut theme.visuals.widgets.hovered, ui, "hovered");
                    widget_visuals::widget_visuals(&mut theme.visuals.widgets.active, ui, "active");
                    widget_visuals::widget_visuals(&mut theme.visuals.widgets.open, ui, "open");
                });
                ui.separator();
                
                selection::selection(&mut theme.visuals.selection, ui);
                ui.separator();
                
                colour::colour(&mut theme.visuals, ui);
                ui.separator();
                
                ui.collapsing("Window Rounding", |ui| {
                    rounding::rounding(&mut theme.visuals.window_rounding, ui);
                });
                
                ui.separator();
                
                ui.collapsing("Shadows", |ui| {
                    shadow::shadow(&mut theme.visuals.window_shadow, ui, "window shadow");
                    shadow::shadow(&mut theme.visuals.popup_shadow, ui, "popup shadow");
                });
                
                ui.separator();
                
                ui.collapsing("Window Stroke", |ui| {
                    stroke::stroke(&mut theme.visuals.window_stroke, ui);
                });
                
                ui.separator();
                
                ui.collapsing("Sizes", |ui| {
                    ui.add(eframe::egui::Slider::new(&mut theme.visuals.resize_corner_size, 0.0..=10.));
                    ui.add(eframe::egui::Slider::new(&mut theme.visuals.text_cursor_width, 0.0..=10.));
                    ui.add(eframe::egui::Slider::new(&mut theme.visuals.clip_rect_margin, 0.0..=10.));
                });
                
                ui.separator();
                
                ui.collapsing("Toggles", |ui| {
                    ui.toggle_value(&mut theme.visuals.dark_mode, "Darkmode? - probably not useful");
                    ui.toggle_value(&mut theme.visuals.text_cursor_preview, "Text Cursor Preview");
                    ui.toggle_value(&mut theme.visuals.button_frame, "Button Frame");
                    ui.toggle_value(&mut theme.visuals.collapsing_header_frame, "Collapsing Header Frame");
                });
                
            });
        });
    }
}