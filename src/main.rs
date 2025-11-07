#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Include the shared app implementation into the binary crate as well.
include!("shared.rs");

// when compiling to native
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let option = eframe::NativeOptions::default();
    eframe::run_native(
        "WhiteFC App",
        option,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::light());
            Box::new(WhiteFCApp::default())
        }),
    )
}

// When building for wasm this binary target still needs a `main` symbol.
// Provide a no-op main for the wasm target so cargo can build the binary
// when running a wasm-targeted build (trunk invokes cargo for wasm).
#[cfg(target_arch = "wasm32")]
fn main() {}
