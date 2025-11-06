use eframe::egui; // (1)

#[derive(Default)] // (2)
struct WhiteFCApp {}

impl eframe::App for WhiteFCApp {
    // (3)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // (4)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello egui"); // (5)
        });
    }
}

fn run_app() -> Result<(), eframe::Error> {
    let option = eframe::NativeOptions::default(); // (6)
    eframe::run_native(
        // (7)
        "WhiteFC App", // title
        option,        // NativeOptions
        Box::new(|cc| {
            // Force light visuals (white background)
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(WhiteFCApp::default()))
        }), // (8)
    )
}

fn main() -> Result<(), eframe::Error> {
    run_app()
}
