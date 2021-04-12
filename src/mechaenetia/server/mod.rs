use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct ServerPlugin;

impl PluginGroup for ServerPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {}
}
