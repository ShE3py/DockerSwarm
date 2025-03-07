use eframe::wasm_bindgen::closure::Closure;
use std::cell::{Cell, OnceCell};
use std::rc::Rc;
use eframe::egui::{Align2, Context, Window};
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

/// Swarm workers monitor.
#[derive(Debug, Default)]
pub(crate) struct Monitor {
    /// The worker's websocket.
    spy: OnceCell<WebSocket>,
    
    /// The number of working workers in the swarm.
    working: Cell<u32>,
    
    /// The number of workers in the swarm.
    workers: Cell<u32>,
}

impl Monitor {
    pub(crate) fn new(spy: &WebSocket) -> Rc<Monitor> {
        let this = Rc::new(Monitor::default());
        
        // on open
        let that = Rc::clone(&this);
        let ws = spy.clone();
        let on_open = Closure::<dyn FnMut()>::new(move || {
            that.spy.set(ws.clone()).unwrap();
        });
        spy.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        
        // on message
        let that = Rc::clone(&this);
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
            
            that.working.set(working);
            that.workers.set(workers);
        });
        spy.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        
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
}
