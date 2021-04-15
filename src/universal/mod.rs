use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

#[derive(Default)]
pub struct UniversalPlugin;

impl PluginGroup for UniversalPlugin {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::core::CorePlugin::default())
			.add(bevy::transform::TransformPlugin::default())
			.add(bevy::diagnostic::DiagnosticsPlugin::default())
			.add(bevy::input::InputPlugin::default())
			.add(bevy::window::WindowPlugin::default())
			.add(bevy::asset::AssetPlugin::default())
			.add(bevy::scene::ScenePlugin::default())
			.add(UniversalPlugin);
	}
}

impl Plugin for UniversalPlugin {
	fn build(&self, _app: &mut AppBuilder) {}
}
