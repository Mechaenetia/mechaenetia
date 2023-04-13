#![warn(clippy::pedantic)]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod states;

pub struct EnginePlugins;

impl PluginGroup for EnginePlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(states::StatePlugin)
	}
}
