//!
//! The monitor (i.e. number of workers) window.
//!

use eframe::egui::{Align2, Context, Window};
use eframe::wasm_bindgen::closure::Closure;
use std::cell::Cell;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

/// Swarm workers monitor.
#[derive(Debug, Default)]
pub(crate) struct Monitor {
    /// The number of working workers in the swarm.
    working: Cell<u32>,
    
    /// The number of workers in the swarm.
    workers: Cell<u32>,
}

impl Monitor {
    pub(crate) fn new() -> Rc<Monitor> {
        let this = Rc::new(Monitor::default());
        this.connect();
        this
    }
    
    pub(crate) fn show(self: &Rc<Monitor>, ctx: &Context) {
        Window::new("Moniteur").anchor(Align2::RIGHT_TOP, [-16.0, 40.0]).collapsible(false).resizable(false).show(ctx, |ui| {
            let workers = self.workers.get();
            let available = self.working.get();
            
            ui.label(format!("Workers actifs : {workers}"));
            ui.label(format!("Workers disponibles : {available}"));
        });
    }
    
    /// Connect to the worker
    fn connect(self: &Rc<Monitor>) {
        info!("Connexion au spy...");
        let worker = WebSocket::new("ws://localhost:4000").unwrap();
        
        // on open
        let on_open = Closure::<dyn FnMut()>::new(move || {
            info!("Connecté au spy.");
        });
        worker.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        
        // on close
        let this = Rc::clone(self);
        let on_close = Closure::<dyn FnMut()>::new(move || {
            this.working.set(0);
            this.workers.set(0);
            this.connect();
        });
        worker.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();
        
        
        // on message
        let this = Rc::clone(self);
        let on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            let msg = e.data().as_string().expect("got a non-string msg");
            let (working, workers) = msg
                .split_once('/')
                .map(|(working, workers)|
                    working.parse::<u32>().and_then(|working|
                        workers.parse::<u32>().map(|workers| (working, workers))
                    )
                )
                .expect("bad msg")
                .expect("bad msg");
            
            this.working.set(working);
            this.workers.set(workers);
        });
        worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
    }
}
