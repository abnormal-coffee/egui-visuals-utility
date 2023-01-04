use eframe;

pub fn shadow(shadow: &mut eframe::epaint::Shadow, ui: &mut eframe::egui::Ui, title: &str) {
    eframe::egui::CollapsingHeader::new(title).show(ui, |ui| {
        ui.add(eframe::egui::Slider::new(&mut shadow.extrusion, 0.0..=10.).text("Thickness"));
        ui.horizontal(|ui| {
            ui.label("Colour");
            colour_picker(&mut shadow.color, ui);
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