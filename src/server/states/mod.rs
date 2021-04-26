mod exiting;
mod loading;
mod not_running;
mod unloading;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ServerState {
	NotRunning,
	Loading,
	Running,
	Paused,
	Unloading,
	Exiting,
}

#[derive(Default)]
pub struct ServerStatePlugin;

impl Plugin for ServerStatePlugin {
	fn build(&self, app: &mut AppBuilder) {
		// Add the Server state into the system.
		app.add_state(ServerState::NotRunning);
		exiting::register_systems(app);
		loading::register_systems(app);
		unloading::register_systems(app);
		not_running::register_systems(app);
	}
}
