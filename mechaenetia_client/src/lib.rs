#![warn(clippy::pedantic)]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod states;

pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(states::StatePlugin)
	}
}
