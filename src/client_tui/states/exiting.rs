use crate::client_tui::tui_plugin::Frame;
use bevy::ecs::world::WorldCell;
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::Exiting;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(SystemSet::on_update(state.clone()).with_system(on_update.system()))
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn on_enter() {
	trace!("Client Exiting State: Enter");
}

fn on_update() {
	trace!("Client Exiting State: Update");
}

fn on_exit() {
	trace!("Client Exiting State: Exit");
}

pub fn draw(_world: &WorldCell, f: &mut Frame) {
	use tui::widgets::*;
	let size = f.size();
	let block = Block::default().title("Exiting").borders(Borders::ALL);
	f.render_widget(block, size);
}
