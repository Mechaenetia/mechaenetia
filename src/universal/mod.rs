pub mod conditional_map;
pub mod exit;
pub mod i18n;
pub mod local_server;

pub use i18n::I18N;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

pub struct UniversalPluginGroup {
	languages_root_path: PathBuf,
	languages: Vec<LanguageIdentifier>,
}

impl Default for UniversalPluginGroup {
	fn default() -> Self {
		Self {
			languages_root_path: "lang".into(),
			languages: vec!["en-US"
				.parse()
				.expect("Parsing `en-US` as a language failed")],
		}
	}
}

impl UniversalPluginGroup {
	pub fn languages_root_path(self, languages_root_path: PathBuf) -> Self {
		Self {
			languages_root_path,
			..self
		}
	}
	pub fn language(self, languages: Vec<LanguageIdentifier>) -> Self {
		Self { languages, ..self }
	}
}

impl PluginGroup for UniversalPluginGroup {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::core::CorePlugin::default())
			.add(bevy::transform::TransformPlugin::default())
			.add(bevy::diagnostic::DiagnosticsPlugin::default())
			.add(bevy::input::InputPlugin::default())
			.add(bevy::window::WindowPlugin {
				// Don't exit on window close as we handle our own close handling
				exit_on_close: false,
				..Default::default()
			})
			.add(bevy::asset::AssetPlugin::default())
			.add(bevy::scene::ScenePlugin::default())
			.add(bevy::gltf::GltfPlugin::default())
			.add(exit::ExitPlugin::default())
			.add(i18n::I18NPlugin::new(
				self.languages_root_path.clone(),
				self.languages.clone(),
			))
			.add(local_server::LocalServerPlugin::default());
	}
}
