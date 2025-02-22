use eframe::{egui, App, CreationContext, Frame};
use egui::{CentralPanel, Context, TextEdit, TopBottomPanel};
use egui_form::garde::GardeReport;
use egui_form::{garde::field_path, Form, FormField};
use garde::Validate;
use hex::FromHexError;

#[derive(Debug, Validate)]
pub struct Hive {
    #[garde(custom(validate_md5))]
    md5: String,
    
    #[garde(skip)]
    in_progress: bool,
}

fn validate_md5(md5: &str, _cx: &()) -> garde::Result {
    let mut digest = [0; 16];
    hex::decode_to_slice(md5, &mut digest).map_err(|e| garde::Error::new(match e {
        FromHexError::InvalidHexCharacter { c, .. } => format!("{c:?} n'est pas un chiffre valide"),
        FromHexError::OddLength | FromHexError::InvalidStringLength => "Le MD5 doit faire 32 caractères".to_owned(),
    }))
}

impl Default for Hive {
    fn default() -> Self {
        Self {
            md5: "81dc9bdb52d04dc20036dbd8313ed055".to_owned(),
            in_progress: false,
        }
    }
}

impl Hive {
    pub fn new(ccx: &CreationContext<'_>) -> Self {
        ccx.egui_ctx.set_pixels_per_point(1.2);
        Hive::default()
    }
}

impl App for Hive {
    fn update(&mut self, rcx: &Context, _frame: &mut Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        
        TopBottomPanel::top("top_panel").show(rcx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            rcx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        
        CentralPanel::default().show(rcx, |ui| {
            ui.heading("Hive");
            
            ui.add_enabled_ui(!self.in_progress, |ui| {
                let mut form = Form::new().add_report(GardeReport::new(self.validate()));
                
                FormField::new(&mut form, field_path!("md5"))
                    .label("MD5")
                    .ui(ui, TextEdit::singleline(&mut self.md5));
                
                let res = ui.horizontal(|ui| {
                    let res = ui.button("Ok");
                    if self.in_progress {
                        ui.spinner();
                    }
                    res
                }).inner;
                
                if let Some(Ok(())) = form.handle_submit(&res, ui) {
                    self.in_progress = true;
                    println!("Submitted: {:?}", self.md5);
                }
            });
        });
    }
}
