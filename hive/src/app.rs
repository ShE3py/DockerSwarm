use eframe::egui::{FontFamily, TextStyle};
use eframe::wasm_bindgen::closure::Closure;
use eframe::{egui, App, CreationContext, Frame};
use egui::{CentralPanel, Context, TextEdit, TopBottomPanel};
use egui_form::garde::GardeReport;
use egui_form::{garde::field_path, Form, FormField};
use garde::Validate;
use hex::FromHexError;
use std::cell::{Cell, OnceCell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

#[derive(Debug, Validate)]
pub struct Hive {
    /// The md5's field value.
    #[garde(custom(validate_md5))]
    md5: String,
    
    /// `true` is a MD5 break request is in progress.
    #[garde(skip)]
    in_progress: Rc<Cell<bool>>,
    
    /// The websocket.
    #[garde(skip)]
    socks: Rc<OnceCell<WebSocket>>,
    
    /// The last broken MD5 (or error message).
    #[garde(skip)]
    broken: Rc<RefCell<Option<String>>>,
}

fn validate_md5(md5: &str, _cx: &()) -> garde::Result {
    let mut digest = [0; 16];
    hex::decode_to_slice(md5, &mut digest).map_err(|e| garde::Error::new(match e {
        FromHexError::InvalidHexCharacter { c, index: _ } => format!("{c:?} n'est pas un chiffre valide"),
        FromHexError::OddLength | FromHexError::InvalidStringLength => "Le MD5 doit faire 32 caractères".to_owned(),
    }))
}

impl Hive {
    pub(crate) fn new(ccx: &CreationContext<'_>, thighs: WebSocket) -> Hive {
        ccx.egui_ctx.set_pixels_per_point(1.2);
        
        let in_progress = Rc::new(Cell::new(false));
        let socks = Rc::new(OnceCell::new());
        let broken = Rc::new(RefCell::new(None));
        
        // on open
        let closure_ws = thighs.clone();
        let closure_ws_rc = socks.clone();
        let on_open = Closure::<dyn FnMut()>::new(move || {
            closure_ws_rc.deref().set(closure_ws.clone()).unwrap();
        });
        thighs.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        
        // on message
        let closure_in_progress_rc = in_progress.clone();
        let closure_broken_rc = broken.clone();
        let on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            closure_broken_rc.deref().replace(Some(e.data().as_string().expect("Got a non-string msg")));
            closure_in_progress_rc.deref().set(false);
        });
        thighs.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        
        Hive {
            md5: "81dc9bdb52d04dc20036dbd8313ed055".to_owned(),
            in_progress,
            socks,
            broken,
        }
    }
}

impl App for Hive {
    fn update(&mut self, rcx: &Context, _frame: &mut Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        
        TopBottomPanel::top("top_panel").show(rcx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        
        CentralPanel::default().show(rcx, |ui| {
            ui.heading("Hive");
            
            ui.label("Exemples : f71dbe52628a3f83a77ab494817525c6 / 5d933eef19aee7da192608de61b6c23d / 49d02d55ad10973b7b9d0dc9eba7fdf0");
            
            ui.add_enabled_ui(!self.in_progress.get() && self.socks.get().is_some(), |ui| {
                let mut form = Form::new().add_report(GardeReport::new(self.validate()));
                
                FormField::new(&mut form, field_path!("md5"))
                    .label("MD5")
                    .ui(ui, TextEdit::singleline(&mut self.md5).font(TextStyle::Monospace));
                
                let res = ui.horizontal(|ui| {
                    let res = ui.button("Retrouver");
                    if self.in_progress.get() {
                        ui.spinner();
                    }
                    res
                }).inner;
                
                if let Some(Ok(())) = form.handle_submit(&res, ui) {
                    self.in_progress.set(true);
                    info!("MD5: {:?}", self.md5);
                    
                    let ws = self.socks.get().unwrap();
                    if let Err(e) = ws.send_with_str(&self.md5) {
                        error!("send(): {e:?}");
                    }
                }
            });
            
            if let Ok(lock) = self.broken.try_borrow() {
                if let Some(s) = lock.as_ref() {
                    ui.label(s.as_str());
                }
            }
        });
    }
}
