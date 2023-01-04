use eframe;

pub fn rounding(rounding: &mut eframe::egui::Rounding, ui: &mut eframe::egui::Ui) {
    ui.add(eframe::egui::Slider::new(&mut rounding.nw, 0.0..=10.).text("NW"));
    ui.add(eframe::egui::Slider::new(&mut rounding.ne, 0.0..=10.).text("NE"));
    ui.add(eframe::egui::Slider::new(&mut rounding.sw, 0.0..=10.).text("SW"));
    ui.add(eframe::egui::Slider::new(&mut rounding.se, 0.0..=10.).text("SE"));
}