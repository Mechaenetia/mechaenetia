use crate::universal::exit::Exiting;
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ServerState::NotRunning;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(
			SystemSet::on_update(state.clone())
				.with_system(on_update.system())
				.with_system(on_shutdown.system()),
		)
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn on_enter() {
	trace!("Server NotRunning State: Enter");
}

fn on_update() {
	// trace!("Server NotRunning State: Update");
}

fn on_exit() {
	trace!("Server NotRunning: Exit");
}

fn on_shutdown(exiting: Option<Res<Exiting>>, mut state: ResMut<State<super::ServerState>>) {
	if let Some(_exiting) = exiting {
		state
			.overwrite_replace(super::ServerState::Exiting)
			.expect("Failed to transition Client to exiting state");
	}
}
