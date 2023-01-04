use eframe;

pub fn stroke(stroke: &mut eframe::egui::Stroke, ui: &mut eframe::egui::Ui) {
    ui.add(eframe::egui::Slider::new(&mut stroke.width, 0.0..=10.).text("Thickness"));
    colour_picker(&mut stroke.color, ui);
}

fn colour_picker(colour: &mut eframe::epaint::Color32, ui: &mut eframe::egui::Ui) {
    ui.horizontal(|ui| {
        eframe::egui::color_picker::color_edit_button_srgba(ui, colour, eframe::egui::color_picker::Alpha::BlendOrAdditive);
        for c in 0..4 {
            ui.add(eframe::egui::DragValue::new(&mut colour[c]));
        }
    });
}