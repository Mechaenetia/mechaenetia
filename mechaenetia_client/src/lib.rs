#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod main_menu;
pub mod states;

pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(states::StatePlugin)
			.add(main_menu::MainMenuPlugin)
	}
}
