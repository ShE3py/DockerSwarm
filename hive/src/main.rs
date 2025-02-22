#![cfg(target_arch = "wasm32")]

#[macro_use]
extern crate log;

use eframe::{WebLogger, WebOptions, WebRunner};
use log::LevelFilter;
use web_sys::WebSocket;
use eframe::wasm_bindgen::JsCast as _;
use crate::app::Hive;

mod app;

fn main() {
    WebLogger::init(LevelFilter::Debug).ok();
    info!("Démarrage de l'application...");
    
    let web_options = WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");
        
        // Connexion au websocket
        let thighs = WebSocket::new("ws://localhost:8000").unwrap();
        
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
                Box::new(|rcx| Ok(Box::new(Hive::new(rcx, thighs)))),
            )
            .await;
        
        // Remove the loading text and spinner
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                    info!("Application démarrée.")
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
