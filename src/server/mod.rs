mod states;

use crate::universal::local_server::LocalServerPublicState;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct ServerPluginGroup;

struct ServerPlugin;

impl PluginGroup for ServerPluginGroup {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::app::ScheduleRunnerPlugin::default())
			.add(ServerPlugin)
			.add(states::ServerStatePlugin::default());
	}
}

impl Plugin for ServerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(LocalServerPublicState::Off);
	}
}
