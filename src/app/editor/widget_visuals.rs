use eframe;


pub fn widget_visuals(widget_visuals: &mut eframe::egui::style::WidgetVisuals, ui: &mut eframe::egui::Ui, title: &str) {
    eframe::egui::CollapsingHeader::new(title).show(ui, |ui| {
        ui.collapsing("bg_fill", |ui| {
            colour_picker(&mut widget_visuals.bg_fill, ui);
            });
            ui.collapsing("bg_stroke", |ui| {
                stroke(&mut widget_visuals.bg_stroke, ui);
            });
            ui.collapsing("rounding", |ui| {
                rounding(&mut widget_visuals.rounding, ui)
            });
            ui.collapsing("fg_stroke", |ui| {
                stroke(&mut widget_visuals.fg_stroke, ui);
            });
            ui.collapsing("expansion", |ui| {
                ui.add(eframe::egui::Slider::new(&mut widget_visuals.expansion, 0.0..=10.));
            });
        });
        ui.separator();
}

fn stroke(stroke: &mut eframe::egui::Stroke, ui: &mut eframe::egui::Ui) {
    ui.add(eframe::egui::Slider::new(&mut stroke.width, 0.0..=10.).text("Thickness"));
    colour_picker(&mut stroke.color, ui);
}

fn rounding(rounding: &mut eframe::egui::Rounding, ui: &mut eframe::egui::Ui) {
    ui.add(eframe::egui::Slider::new(&mut rounding.nw, 0.0..=10.).text("NW"));
    ui.add(eframe::egui::Slider::new(&mut rounding.ne, 0.0..=10.).text("NE"));
    ui.add(eframe::egui::Slider::new(&mut rounding.sw, 0.0..=10.).text("SW"));
    ui.add(eframe::egui::Slider::new(&mut rounding.se, 0.0..=10.).text("SE"));
}

fn colour_picker(colour: &mut eframe::epaint::Color32, ui: &mut eframe::egui::Ui) {
    ui.horizontal(|ui| {
        eframe::egui::color_picker::color_edit_button_srgba(ui, colour, eframe::egui::color_picker::Alpha::BlendOrAdditive);
        for c in 0..4 {
            ui.add(eframe::egui::DragValue::new(&mut colour[c]));
        }
    });
}