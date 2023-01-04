use eframe;


pub fn colour(visuals: &mut eframe::egui::Visuals, ui: &mut eframe::egui::Ui) {
    eframe::egui::CollapsingHeader::new("Colour").show(ui, |ui| {
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.hyperlink_color, ui);
            ui.label("hyperlink colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.faint_bg_color, ui);
            ui.label("faint bg colour colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.extreme_bg_color, ui);
            ui.label("extreme bg colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.code_bg_color, ui);
            ui.label("code bg colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.warn_fg_color, ui);
            ui.label("warn fg colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.error_fg_color, ui);
            ui.label("error fg colour");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.window_fill, ui);
            ui.label("window fill");
        });
        ui.horizontal(|ui| {
            colour_picker(&mut visuals.panel_fill, ui);
            ui.label("panel fill");
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