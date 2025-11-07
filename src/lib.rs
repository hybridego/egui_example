// Include the shared app implementation so it is compiled into both the
// native binary and the WASM cdylib produced by trunk.
include!("shared.rs");

// WASM entrypoint
#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use super::WhiteFCApp;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn start() {
        // Better panic messages in the browser console
        console_error_panic_hook::set_once();
        tracing_wasm::set_as_global_default();

        let web_options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async {
            eframe::WebRunner::new()
                .start(
                    "the_canvas_id",
                    web_options,
                    Box::new(|cc| {
                        cc.egui_ctx.set_visuals(eframe::egui::Visuals::light());
                        Box::new(WhiteFCApp)
                    }),
                )
                .await
                .expect("failed to start eframe");
        });
    }
}
