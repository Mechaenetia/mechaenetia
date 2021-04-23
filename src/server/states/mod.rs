mod exiting;
mod not_running;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ServerState {
	NotRunning,
	// Loading,
	// Running,
	// Paused,
	Exiting,
}

#[derive(Default)]
pub struct ServerStatePlugin;

impl Plugin for ServerStatePlugin {
	fn build(&self, app: &mut AppBuilder) {
		// Add the Server state into the system.
		app.add_state(ServerState::NotRunning);
		not_running::register_systems(app);
		exiting::register_systems(app);
	}
}
