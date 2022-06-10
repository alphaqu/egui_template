extern crate eframe;
extern crate egui;

use eframe::{App, Frame, NativeOptions, run_native};
use egui::{Context, Visuals};

fn main() {
	run_native(
		"Rust GUI",
		NativeOptions::default(),
		Box::new(|cc| {
			cc.egui_ctx.set_visuals(Visuals::dark());
			Box::new(Application {})
		}),
	);
}

pub struct Application {}

impl App for Application {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// Top menu
		});

		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			// Side panel on the right
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// Central area
		});
	}
}