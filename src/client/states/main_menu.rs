use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::MainMenu;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(SystemSet::on_update(state.clone()).with_system(on_update.system()))
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

pub fn on_enter() {
	trace!("MainMenu State: Enter");
}

pub fn on_update(mut _state: ResMut<State<super::ClientState>>) {
	// trace!("MainMenu State: Update");
	// Overwrite any possible pending state change, exit the app now by transitioning to the exit state to clean up
	// state
	// 	.overwrite_replace(super::ClientState::Exiting)
	// 	.expect("error while transitioning to the Exiting state");
}

pub fn on_exit() {
	trace!("MainMenu State: Exit");
}
