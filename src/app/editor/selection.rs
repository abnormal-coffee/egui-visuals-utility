use eframe;

pub fn selection(selection: &mut eframe::egui::style::Selection, ui: &mut eframe::egui::Ui) {
    eframe::egui::CollapsingHeader::new("Selection").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("bg_fill");
            colour_picker(&mut selection.bg_fill, ui);
        });
        stroke(&mut selection.stroke, ui);
    });
}

fn stroke(stroke: &mut eframe::egui::Stroke, ui: &mut eframe::egui::Ui) {
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