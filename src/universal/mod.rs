pub mod i18n;

pub use i18n::I18N;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use unic_langid::LanguageIdentifier;

pub struct UniversalPlugin {
	language: LanguageIdentifier,
}

impl Default for UniversalPlugin {
	fn default() -> Self {
		Self {
			language: "en-US"
				.parse()
				.expect("Parsing `en-US` as a language failed"),
		}
	}
}

impl UniversalPlugin {
	pub fn language(self, language: &LanguageIdentifier) -> Self {
		Self {
			language: language.to_owned(),
			..self
		}
	}
}

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
			.add(i18n::I18NPlugin::new(self.language.clone()));
	}
}
