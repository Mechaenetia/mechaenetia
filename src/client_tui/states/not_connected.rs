use crate::client_tui::tui_plugin::Frame;
use crate::universal::exit::Exiting;
use bevy::ecs::world::WorldCell;
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::NotConnected;
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

fn on_update() {
	// trace!("Client Loading State: Update");
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

pub fn draw(_world: &WorldCell, f: &mut Frame) {
	use tui::widgets::*;
	let size = f.size();
	let block = Block::default().title("Not Loaded").borders(Borders::ALL);
	f.render_widget(block, size);
}
