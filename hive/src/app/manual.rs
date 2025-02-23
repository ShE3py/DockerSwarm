use eframe::egui::{Context, FontFamily, Key, RichText, TextEdit, TextStyle, Ui};
use eframe::wasm_bindgen::closure::Closure;
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
use garde::Validate;
use std::cell::{Cell, OnceCell, RefCell};
use std::ops::{Deref as _, DerefMut as _};
use std::rc::Rc;
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

/// User-specified manual MD5 break.
#[derive(Debug, Validate)]
pub(crate) struct Manual {
    /// The word to break (autofill `md5` field)
    #[garde(custom(validate_word))]
    word: RefCell<String>,
    
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

fn validate_word(word: &RefCell<String>, _cx: &()) -> garde::Result {
    let lock = word.borrow();
    let s = (*lock).as_str().trim();
    
    if s.len() > 5 {
        Err(garde::Error::new("Le mot doit faire au plus 5 caractères"))
    }
    else if s.chars().any(|c| !c.is_ascii_alphanumeric()) {
        Err(garde::Error::new("Le mot doit être alphanumérique"))
    }
    else {
        Ok(())
    }
}

fn validate_md5(md5: &RefCell<String>, _cx: &()) -> garde::Result {
    let lock = md5.borrow();
    let s = (*lock).as_str().trim();
    
    if s.chars().any(|c| !c.is_ascii_hexdigit()) {
        Err(garde::Error::new("Le MD5 doit être hexadécimal"))
    }
    else if s.len() != 32 {
        Err(garde::Error::new("Le MD5 doit faire 32 caractères"))
    }
    else {
        Ok(())
    }
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
        
        if self.socks.get().is_none() {
            ui.horizontal(|ui| {
                ui.label("Connexion…");
                ui.spinner();
            });
            
            return;
        }
        
        // Form
        ui.add_enabled_ui(!self.in_progress.get(), move |ui| {
            let mut form = Form::new().add_report(GardeReport::new(self.validate()));
            
            // Word field
            let res = FormField::new(&mut form, field_path!("word"))
                .label("Mot")
                .ui(ui, TextEdit::singleline(self.word.borrow_mut().deref_mut()).font(TextStyle::Monospace));
            if res.changed() {
                self.md5.replace(hex::encode(md5::compute(self.word.borrow().deref()).0));
            }
            let mut submit = res.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            
            // MD5 field
            let res = FormField::new(&mut form, field_path!("md5"))
                .label("MD5")
                .ui(ui, TextEdit::singleline(self.md5.borrow_mut().deref_mut()).font(TextStyle::Monospace));
            submit |= res.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            
            
            // Result
            ui.vertical(|ui| {
                // Label
                let label = RichText::new("Résultat")
                    .size(ui.style().text_styles.get(&TextStyle::Body).map_or(16.0, |s| s.size) * 0.9);
                ui.label(label);
                
                // Value, if available
                if let Ok(lock) = self.broken.try_borrow() {
                    if let Some(s) = lock.as_ref() {
                        let value = RichText::new(s.as_str())
                            .color(ui.visuals().override_text_color.unwrap_or(ui.visuals().widgets.inactive.text_color()))
                            .family(FontFamily::Monospace);
                        ui.label(value);
                    }
                }
                
                // Update/spinner
                submit |= ui.horizontal(|ui| {
                    let res = ui.button("Obtenir");
                    if self.in_progress.get() {
                        ui.spinner();
                    }
                    res.clicked()
                }).inner;
            });
            
            
            // Submit action
            if submit && form.try_submit(ui).is_ok() {
                self.in_progress.set(true);
                info!("MD5: {:?}", self.md5);
                
                let ws = self.socks.get().unwrap();
                if let Err(e) = ws.send_with_str(self.md5.borrow().deref()) {
                    error!("send(): {e:?}");
                }
            }
        });
    }
}

impl Default for Manual {
    fn default() -> Self {
        Manual {
            word: RefCell::new("1234".to_owned()),
            md5: RefCell::new("81dc9bdb52d04dc20036dbd8313ed055".to_owned()),
            in_progress: Cell::new(false),
            socks: OnceCell::new(),
            broken: RefCell::new(None),
        }
    }
}
