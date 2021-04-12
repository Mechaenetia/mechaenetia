use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct UniversalPlugin;

impl PluginGroup for UniversalPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group.add(UniversalPlugin);
	}
}

impl Plugin for UniversalPlugin {
	fn build(&self, app: &mut AppBuilder) {
	}
}
