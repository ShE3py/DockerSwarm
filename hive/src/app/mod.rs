//!
//! The app widgets & logic.
//!

use crate::app::home::Home;
use crate::app::monitor::Monitor;
use eframe::{egui, App, CreationContext, Frame};
use egui::{CentralPanel, Context, TopBottomPanel};
use std::rc::Rc;

mod home;
mod monitor;

/// The app.
#[derive(Debug)]
pub(crate) struct Hive {
    /// User-initiated manual MD5 break.
    home: Rc<Home>,
    
    /// Swarm workers monitor.
    monitor: Rc<Monitor>,
}

impl Hive {
    pub(crate) fn new(ccx: &CreationContext<'_>) -> Hive {
        ccx.egui_ctx.set_pixels_per_point(1.2);
        
        Hive {
            home: Home::new(),
            monitor: Monitor::new(),
        }
    }
}

impl App for Hive {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        
        CentralPanel::default().show(ctx, |ui| {
            self.home.ui(ctx, ui);
        });
        
        self.monitor.show(ctx);
    }
}
