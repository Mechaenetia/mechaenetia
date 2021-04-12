use bevy::log::LogPlugin;
use bevy::prelude::*;
use std::path::PathBuf;
use tracing::log::LevelFilter;

// pub mod game_data;
// pub mod state;
pub mod client;
pub mod server;
pub mod universal;

mod logger;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Logging initialization error")]
	LoggerError(#[from] logger::Error),
}

#[derive(Debug)]
pub struct Mechaenetia {
	config_dir: PathBuf,
	logging_level_override: Option<LevelFilter>,
	include_server: bool,
	include_client: bool,
}

/// Central Mechaenetia entrance point, start by calling `Mechaenetia::new()` and call its functions
/// as appropriate and finish with `.run()` to execute it.
impl Mechaenetia {
	pub fn new(config_dir: impl Into<PathBuf>) -> Mechaenetia {
		Mechaenetia {
			config_dir: config_dir.into(),
			logging_level_override: None,
			include_server: false,
			include_client: false,
		}
	}

	pub fn run(self) -> Result<(), Error> {
		logger::init_logging(Some(self.config_dir.as_path()))?;
		let mut app_builder = App::build();

		app_builder.add_plugins(universal::UniversalPlugin::default());

		if self.include_client {
			app_builder
				.add_plugins_with(
					DefaultPlugins,
					|group| group.disable::<LogPlugin>(), // We have a more configurable logger, log4rs, so don't use EnvFilter
				)
				.add_plugins(client::ClientPlugin::default());
		} else {
			app_builder
				.add_plugins(MinimalPlugins)
				.add_plugin(bevy::transform::TransformPlugin::default())
				.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
				.add_plugin(bevy::input::InputPlugin::default())
				.add_plugin(bevy::window::WindowPlugin::default())
				.add_plugin(bevy::asset::AssetPlugin::default())
				.add_plugin(bevy::scene::ScenePlugin::default());
		}

		if self.include_server {
			app_builder.add_plugins(server::ServerPlugin::default());
		}

		Ok(app_builder.run())
	}

	pub fn override_logging_level(&mut self, level: LevelFilter) -> &mut Self {
		self.logging_level_override = Some(level);
		self
	}

	pub fn set_include_server(&mut self, include_server: bool) -> &mut Self {
		self.include_server = include_server;
		self
	}

	pub fn set_include_client(&mut self, include_client: bool) -> &mut Self {
		self.include_client = include_client;
		self
	}
}
