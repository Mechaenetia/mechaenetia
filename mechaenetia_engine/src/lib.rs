#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::app::{PluginGroup, PluginGroupBuilder};
use std::path::PathBuf;

pub mod sim;
pub mod states;

pub struct EnginePlugins {
	pub default_save_path: PathBuf,
}

impl PluginGroup for EnginePlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(states::StatePlugin)
			.add(sim::SimPlugin {
				default_save_path: self.default_save_path,
			})
	}
}
