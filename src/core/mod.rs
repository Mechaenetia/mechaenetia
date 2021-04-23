/// The `crates::core` module is for the code that is used to set up everything else, but then is
/// not touched by anything else.  The code here is minimal.
use bevy::prelude::*;
use std::convert::Infallible;
use std::fmt::Debug;
use std::path::PathBuf;
use tracing::log::LevelFilter;

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

	pub fn custom_run<
		Out,
		Err: 'static + std::error::Error,
		Runner: FnOnce(AppBuilder) -> Result<Out, Err>,
	>(
		&self,
		runner: Runner,
	) -> Result<Out, EngineError<Err>> {
		logger::init_logging(Some(self.config_dir.as_path()))?;
		let mut app_builder = App::build();

		app_builder.add_plugins(crate::universal::UniversalPlugin::default());

		// Make sure server is added before clients so its runner won't override the client runner
		if self.include_server {
			#[cfg(feature = "server")]
			app_builder.add_plugins(crate::server::ServerPlugin::default());
		}

		if self.include_client {
			#[cfg(feature = "client_wgpu")]
			app_builder.add_plugins(crate::client::ClientPlugin::default());
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
