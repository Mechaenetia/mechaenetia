pub mod exit;
pub mod i18n;
pub mod local_server;

pub use i18n::I18N;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use unic_langid::LanguageIdentifier;

pub struct UniversalPluginGroup {
	language: LanguageIdentifier,
}

impl Default for UniversalPluginGroup {
	fn default() -> Self {
		Self {
			language: "en-US"
				.parse()
				.expect("Parsing `en-US` as a language failed"),
		}
	}
}

impl UniversalPluginGroup {
	pub fn language(self, language: &LanguageIdentifier) -> Self {
		Self {
			language: language.to_owned(),
			..self
		}
	}
}

impl PluginGroup for UniversalPluginGroup {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::core::CorePlugin::default())
			.add(bevy::transform::TransformPlugin::default())
			.add(bevy::diagnostic::DiagnosticsPlugin::default())
			.add(bevy::input::InputPlugin::default())
			.add(bevy::window::WindowPlugin::default())
			.add(bevy::asset::AssetPlugin::default())
			.add(bevy::scene::ScenePlugin::default())
			.add(exit::ExitPlugin::default())
			.add(i18n::I18NPlugin::new(self.language.clone()))
			.add(local_server::LocalServerPlugin::default());
	}
}
