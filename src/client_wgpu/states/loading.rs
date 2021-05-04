use crate::universal::exit::Exiting;
use bevy::prelude::*;

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

fn on_update(mut state: ResMut<State<super::ClientState>>) {
	trace!("Client Loading State: Update");
	// Nothing to do in the Loading state yet, so transition to the Main Menu state
	state
		.set(super::ClientState::MainMenu)
		.expect("error while transitioning to the MainMenu state");
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
