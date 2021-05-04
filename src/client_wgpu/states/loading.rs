use crate::universal::exit::Exiting;
use crate::universal::I18N;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::Loading;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(
			SystemSet::on_update(state.clone())
				.with_system(on_update.system())
				.with_system(on_shutdown.system()),
		)
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn on_enter() {
	trace!("Client Loading State: Enter");
}

fn on_update(
	mut state: ResMut<State<super::ClientState>>,
	lang: Res<I18N>,
	egui_ctx: Res<EguiContext>,
) {
	// trace!("Client Loading State: Update");
	// Acquire loading data...
	let lang_to_load = lang.remaining_to_load();
	info!("Languages left to load: {}", lang_to_load);
	// Show loading state
	egui::CentralPanel::default().show(egui_ctx.ctx(), |ui| {
		ui.vertical_centered(|ui| {
			ui.heading("Loading..."); // Skipping lang for this...
			ui.horizontal(|ui| {
				ui.label("Language files left to load:");
				ui.label(lang_to_load.to_string());
			});
		});
	});
	// And test if ready to switch to main menu
	if lang_to_load == 0 {
		state
			.set(super::ClientState::MainMenu)
			.expect("error while transitioning to the MainMenu state");
	}
}

fn on_exit() {
	trace!("Client Loading State: Exit");
}

fn on_shutdown(exiting: Option<Res<Exiting>>, mut state: ResMut<State<super::ClientState>>) {
	if let Some(_exiting) = exiting {
		state
			.overwrite_replace(super::ClientState::Exiting)
			.expect("Failed to transition Client to exiting state");
	}
}
