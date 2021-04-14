use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct ClientPlugin;

impl PluginGroup for ClientPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group.add(ClientPlugin);
	}
}

impl Plugin for ClientPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_state(ClientState::Loading);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ClientState {
	Loading,
	MainMenu,
	Joining,
	Joined,
	Paused,
}
