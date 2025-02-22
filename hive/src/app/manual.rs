use eframe::egui::{Context, TextEdit, TextStyle, Ui};
use eframe::wasm_bindgen::closure::Closure;
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
use garde::Validate;
use hex::FromHexError;
use std::cell::{Cell, OnceCell, RefCell};
use std::ops::{Deref as _, DerefMut as _};
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

/// User-specified manual MD5 break.
#[derive(Debug, Validate)]
pub(crate) struct Manual {
    /// The md5's field value.
    #[garde(custom(validate_md5))]
    md5: RefCell<String>,
    
    /// `true` is a MD5 break request is in progress.
    #[garde(skip)]
    in_progress: Cell<bool>,
    
    /// The websocket.
    #[garde(skip)]
    socks: OnceCell<WebSocket>,
    
    /// The last broken MD5 (or error message).
    #[garde(skip)]
    broken: RefCell<Option<String>>,
}

fn validate_md5(md5: &RefCell<String>, _cx: &()) -> garde::Result {
    let mut digest = [0; 16];
    hex::decode_to_slice(md5.borrow().deref(), &mut digest).map_err(|e| garde::Error::new(match e {
        FromHexError::InvalidHexCharacter { c, index: _ } => format!("{c:?} n'est pas un chiffre valide"),
        FromHexError::OddLength | FromHexError::InvalidStringLength => "Le MD5 doit faire 32 caractères".to_owned(),
    }))
}

impl Manual {
    pub(crate) fn new(thighs: WebSocket) -> Rc<Manual> {
        let this = Rc::new(Manual::default());
        
        // on open
        let that = this.clone();
        let ws = thighs.clone();
        let on_open = Closure::<dyn FnMut()>::new(move || {
            that.socks.set(ws.clone()).unwrap();
        });
        thighs.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();
        
        // on message
        let that = this.clone();
        let on_message = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            that.broken.replace(Some(e.data().as_string().expect("got a non-string msg")));
            that.in_progress.set(false);
        });
        thighs.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        
        this
    }
    
    pub(crate) fn ui(&self, _rcx: &Context, ui: &mut Ui) {
        ui.heading("Hive");
        
        ui.label("Exemples : f71dbe52628a3f83a77ab494817525c6 / 5d933eef19aee7da192608de61b6c23d / 49d02d55ad10973b7b9d0dc9eba7fdf0");
        
        ui.add_enabled_ui(!self.in_progress.get() && self.socks.get().is_some(), |ui| {
            let mut form = Form::new().add_report(GardeReport::new(self.validate()));
            
            FormField::new(&mut form, field_path!("md5"))
                .label("MD5")
                .ui(ui, TextEdit::singleline(self.md5.borrow_mut().deref_mut()).font(TextStyle::Monospace));
            
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
                if let Err(e) = ws.send_with_str(self.md5.borrow().deref()) {
                    error!("send(): {e:?}");
                }
            }
        });
        
        if let Ok(lock) = self.broken.try_borrow() {
            if let Some(s) = lock.as_ref() {
                ui.label(s.as_str());
            }
        }
    }
}

impl Default for Manual {
    fn default() -> Self {
        Manual {
            md5: RefCell::new("81dc9bdb52d04dc20036dbd8313ed055".to_owned()),
            in_progress: Cell::new(false),
            socks: OnceCell::new(),
            broken: RefCell::new(None),
        }
    }
}
