#![cfg(target_arch = "wasm32")]

#[macro_use]
extern crate log;

use crate::app::Hive;
use eframe::wasm_bindgen::JsCast as _;
use eframe::{WebLogger, WebOptions, WebRunner};
use log::LevelFilter;

mod app;

fn main() {
    _ = WebLogger::init(LevelFilter::Debug);
    info!("Démarrage de l'application...");
    
    let web_options = WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");
        
        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");
        
        // Start the app in a web runner
        let start_result = WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|rcx| Ok(Box::new(Hive::new(rcx)))),
            )
            .await;
        
        // Remove the loading text and spinner
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(()) => {
                    loading_text.remove();
                    info!("Application démarrée.");
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
