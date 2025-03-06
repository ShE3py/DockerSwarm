use crate::app::manual::Manual;
use eframe::{egui, App, CreationContext, Frame};
use egui::{CentralPanel, Context, TopBottomPanel};
use std::rc::Rc;
use web_sys::WebSocket;
use crate::app::monitor::Monitor;

mod manual;
mod monitor;

#[derive(Debug)]
pub(crate) struct Hive {
    /// User-initiated manual MD5 break
    manual: Rc<Manual>,
    
    /// Swarm workers monitor.
    monitor: Rc<Monitor>,
}

impl Hive {
    pub(crate) fn new(ccx: &CreationContext<'_>, worker: &WebSocket, spy: &WebSocket) -> Hive {
        ccx.egui_ctx.set_pixels_per_point(1.2);
        
        Hive {
            manual: Manual::new(worker),
            monitor: Monitor::new(spy),
        }
    }
}

impl App for Hive {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        
        CentralPanel::default().show(ctx, |ui| {
            self.manual.ui(ctx, ui);
        });
        
        self.monitor.show(ctx);
    }
}
