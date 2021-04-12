use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct ClientPlugin;

impl PluginGroup for ClientPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {}
}
