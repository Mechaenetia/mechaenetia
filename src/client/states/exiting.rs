use bevy::app::AppExit;
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::Exiting;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(SystemSet::on_update(state.clone()).with_system(on_update.system()))
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

pub fn on_enter() {
	trace!("Exiting State: Enter");
}

pub fn on_update(mut exit: EventWriter<AppExit>) {
	trace!("Exiting State: Update");
	// Exit after all cleanup is done
	exit.send(AppExit)
}

pub fn on_exit() {
	trace!("Exiting State: Exit");
}
