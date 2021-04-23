use crate::universal::local_server::LocalServerExists;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct ServerPlugin;

impl PluginGroup for ServerPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::app::ScheduleRunnerPlugin::default())
			.add(ServerPlugin);
	}
}

impl Plugin for ServerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_state(ServerState::NotRunning)
			.insert_resource(LocalServerExists);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ServerState {
	NotRunning,
	// Loading,
	// Running,
	// Paused,
}
