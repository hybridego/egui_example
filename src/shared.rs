#[derive(Default)]
pub struct WhiteFCApp;

impl eframe::App for WhiteFCApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello egui on WASM!");
        });
    }
}
