use eframe;

use super::DemoApp;
pub fn demo_widgets(ctx: &eframe::egui::Context, demo_state: &mut DemoApp) {
    
    if demo_state.window_open == true {
        eframe::egui::Window::new("Demo window").show(ctx, |ui| {
            // ui.add(eframe::egui::Button::image_and_text(texture_id, image_size, text))
        });
    }
    eframe::egui::SidePanel::right("demo").resizable(true).show(ctx, |ui| {
        ui.label("Many widgets are yet to be showcased");
        
        if ui.button("Close").clicked() {
            demo_state.demo_open = false;
        }
        ui.toggle_value(&mut demo_state.window_open, "Demo Window?");
        ui.label("Built using egui:");
        ui.hyperlink("https://github.com/emilk/egui");
        ui.separator();
        ui.add(eframe::egui::TextEdit::singleline(&mut demo_state.text_edit));
        ui.add(eframe::egui::TextEdit::multiline(&mut demo_state.text_edit));
    });
}