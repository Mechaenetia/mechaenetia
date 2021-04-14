/// The `crates::core` module is for the code that is used to set up everything else, but then is
/// not touched by anything else.  The code here is minimal.
use bevy::log::LogPlugin;
use bevy::prelude::*;
use std::path::PathBuf;
use tracing::log::LevelFilter;
use std::fmt::Debug;
use std::convert::Infallible;

mod logger;

/// Possible errors returned from the Engine
#[derive(thiserror::Error, Debug)]
pub enum EngineError<CustErr: 'static + std::error::Error> {
	#[error("Logging initialization error")]
	LoggerError(#[from] logger::Error),
	#[error("Custom Runner Error")]
	CustomRunnerError(#[source] CustErr),
}

/// The Engine structure is what holds all the initialization data before eventually running the
/// bevy backend when `Engine::run` is run.  Standard builder so call it as you normally would.
///
/// ```no_run
/// crate::prelude::Engine::new()
/// 	...
/// 	.run()
/// ```
#[derive(Debug)]
pub struct Engine {
	pub config_dir: PathBuf,
	pub logging_level_override: Option<LevelFilter>,
	pub include_server: bool,
	pub include_client: bool,
	pub game_configuration_path: Option<PathBuf>,
}

/// Central engine entrance point, start by calling `Engine::new()` and call its functions
/// as appropriate and finish with `.run()` to execute it.
impl Engine {
	pub fn new(config_dir: impl Into<PathBuf>) -> Engine {
		Engine {
			config_dir: config_dir.into(),
			logging_level_override: None,
			include_server: true,
			include_client: true,
			game_configuration_path: None,
		}
	}
	
	pub fn run(&self) -> Result<(), EngineError<Infallible>> {
		self.custom_run(|mut app| Ok(app.run()))
	}

	pub fn custom_run<Out, Err: 'static + std::error::Error, Runner: FnOnce(AppBuilder) -> Result<Out, Err>>(
		&self,
		runner: Runner,
	) -> Result<Out, EngineError<Err>> {
		logger::init_logging(Some(self.config_dir.as_path()))?;
		let mut app_builder = App::build();

		app_builder.add_plugins(crate::universal::UniversalPlugin::default());

		if self.include_client {
			app_builder
				.add_plugins_with(
					DefaultPlugins,
					|group| group.disable::<LogPlugin>(), // We have a more configurable logger, log4rs, so don't use EnvFilter
				)
				.add_plugins(crate::client::ClientPlugin::default());
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
			app_builder.add_plugins(crate::server::ServerPlugin::default());
		}

		runner(app_builder).map_err(|e| EngineError::CustomRunnerError(e))
	}

	pub fn override_logging_level(&mut self, level: Option<LevelFilter>) -> &mut Self {
		self.logging_level_override = level;
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

	pub fn load_game_configuration(
		&mut self,
		game_configuration_path: Option<PathBuf>,
	) -> &mut Self {
		self.game_configuration_path = game_configuration_path;
		self
	}
}
