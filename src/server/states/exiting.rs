use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ServerState::Exiting;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(SystemSet::on_update(state.clone()).with_system(on_update.system()))
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn on_enter() {
	trace!("Server Exiting State: Enter");
}

fn on_update() {
	trace!("Server Exiting State: Update");
}

fn on_exit() {
	trace!("Server Exiting State: Exit");
}
