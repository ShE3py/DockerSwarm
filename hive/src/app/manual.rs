use eframe::egui::{Context, FontFamily, Key, RichText, TextEdit, TextStyle, Ui};
use eframe::wasm_bindgen::closure::Closure;
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
use garde::Validate;
use std::cell::{Cell, OnceCell, RefCell};
use std::num::NonZero;
use std::ops::{Deref as _, DerefMut as _};
use std::rc::Rc;
use std::sync::atomic::{AtomicU8, Ordering};
use web_sys::wasm_bindgen::JsCast as _;
use web_sys::{MessageEvent, WebSocket};

/// Auto-mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum Mode {
    #[default]
    Manual,
    Kind,
    Normal,
    Aggressive,
}

/// User-specified manual MD5 break.
#[derive(Debug, Validate)]
pub(crate) struct Manual {
    /// The auto-try mode.
    #[garde(skip)]
    mode: Cell<Mode>,
    
    /// The JS interval ID of the auto mode.
    #[garde(skip)]
    interval: Cell<Option<NonZero<i32>>>,
    
    /// The word to break (autofill `md5` field)
    #[garde(custom(validate_word))]
    word: RefCell<String>,
    
    /// The md5's field value.
    #[garde(custom(validate_md5))]
    md5: RefCell<String>,
    
    /// The number of MD5 break request is in progress.
    #[garde(skip)]
    in_progress: AtomicU8,
    
    /// The websocket.
    #[garde(skip)]
    socks: OnceCell<WebSocket>,
    
    /// The last broken MD5 (or error message).
    #[garde(skip)]
    result: RefCell<String>,
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
            that.result.replace(e.data().as_string().expect("got a non-string msg"));
            that.in_progress.fetch_sub(1, Ordering::SeqCst);
        });
        thighs.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
        
        this
    }
    
    pub(crate) fn ui(self: &Rc<Manual>, ctx: &Context, ui: &mut Ui) {
        ui.heading("Hive");
        
        if self.socks.get().is_none() {
            ui.horizontal(|ui| {
                ui.label("Connexion…");
                ui.spinner();
            });
            
            return;
        }
        
        // Form
        let mut form = Form::new().add_report(GardeReport::new(self.validate()));
        
        // Mode field
        FormField::new(&mut form, field_path!("mode"))
            .label("Mode")
            .ui(ui, |ui: &mut Ui| {
                ui.horizontal(|ui| {
                    let mut mode = self.mode.get();
                    
                    let m = ui.selectable_value(&mut mode, Mode::Manual, "Manuel");
                    let k = ui.selectable_value(&mut mode, Mode::Kind, "Gentil");
                    let n = ui.selectable_value(&mut mode, Mode::Normal, "Normal");
                    let a = ui.selectable_value(&mut mode, Mode::Aggressive, "Agressif");
                    
                    let res = m | k | n | a;
                    if res.clicked() {
                        self.mode.set(mode);
                        
                        let window = web_sys::window().expect("no window?");
                        if let Some(interval) = self.interval.replace(None) {
                            window.clear_interval_with_handle(interval.get());
                        }
                        
                        let interval = mode.interval_ms();
                        if interval > 0 {
                            let this = Rc::clone(self);
                            let ctx = ctx.clone();
                            let on_interval = Closure::<dyn FnMut()>::new(move || {
                                const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                                let word = String::from_utf8(fastrand::choose_multiple(ALPHABET.iter().copied(), 4)).expect("bad generated word?");
                                
                                this.word.replace(word);
                                this.update_md5();
                                this.ws_send();
                                ctx.request_repaint();
                            });
                            match window.set_interval_with_callback_and_timeout_and_arguments_0(on_interval.as_ref().unchecked_ref(), interval) {
                                Ok(0) => error!("setInterval(): returned 0?"),
                                Ok(interval) => self.interval.set(NonZero::new(interval)),
                                Err(e) => error!("setInterval(): {e:?}"),
                            }
                            on_interval.forget();
                        }
                    }
                    res
                }).inner
            });
        
        let in_progress = self.in_progress.load(Ordering::SeqCst);
        let is_manual = self.mode.get() == Mode::Manual;
        ui.add_enabled_ui(in_progress == 0 && is_manual, |ui| {
            // Word field
            let res = FormField::new(&mut form, field_path!("word"))
                .label("Mot")
                .ui(ui, TextEdit::singleline(self.word.borrow_mut().deref_mut()).font(TextStyle::Monospace));
            if res.changed() { self.update_md5() };
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
                
                // Value
                let result = RichText::new(self.result.borrow().as_str())
                    .color(ui.visuals().override_text_color.unwrap_or(ui.visuals().widgets.inactive.text_color()))
                    .family(FontFamily::Monospace);
                ui.label(result);
                
                // Update/spinner
                submit |= ui.horizontal(|ui| if is_manual && in_progress < 2 {
                    let res = ui.button("Obtenir").clicked();
                    if in_progress > 0 {
                        ui.spinner();
                    }
                    res
                }
                else {
                    ui.label(format!("En attente : {in_progress}"));
                    ui.spinner();
                    false
                }).inner;
            });
            
            
            // Submit action
            if submit && form.try_submit(ui).is_ok() {
                self.ws_send();
            }
        });
    }
    
    /// Update fields calculated from other fields.
    fn update_md5(&self) {
        self.md5.replace(hex::encode(md5::compute(self.word.borrow().deref().trim()).0));
    }
    
    /// Send a MD5 break request to the worker.
    fn ws_send(&self) {
        self.in_progress.fetch_add(1, Ordering::SeqCst);
        info!("MD5: {:?}", self.md5);
        
        let ws = self.socks.get().unwrap();
        if let Err(e) = ws.send_with_str(self.md5.borrow().deref()) {
            error!("send(): {e:?}");
        }
    }
}

impl Default for Manual {
    fn default() -> Self {
        Manual {
            mode: Cell::new(Mode::default()),
            interval: Cell::new(None),
            word: RefCell::new("1234".to_owned()),
            md5: RefCell::new("81dc9bdb52d04dc20036dbd8313ed055".to_owned()),
            in_progress: AtomicU8::new(0),
            socks: OnceCell::new(),
            result: RefCell::new("1234".to_owned()),
        }
    }
}

impl Mode {
    const fn interval_ms(self) -> i32 {
        match self {
            Mode::Manual => -1,
            Mode::Kind => 2000,
            Mode::Normal => 1000,
            Mode::Aggressive => 500,
        }
    }
}
