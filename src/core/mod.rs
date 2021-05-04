//! The `crates::core` module is for the code that is used to set up everything else, but then is
//! not touched by anything else.  The code here is minimal.

use crate::universal::exit::RequestExit;
use crate::universal::local_server::{LocalServerCommand, LocalServerPublicState};
use bevy::app::Events;
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use std::convert::Infallible;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;
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

/// Client type to include
#[derive(Debug, PartialEq)]
pub enum ClientType {
	/// This has no interactive client, purely just an output logger, need to fully init the server
	/// if it should be doing anything.
	Logger,
	/// Full 3D renderer, requires Vulkan currently.
	#[cfg(feature = "client_wgpu")]
	WGPU,
	/// An extensive Terminal User Interface, no GUI needed, runs entirely in a terminal.
	#[cfg(feature = "client_tui")]
	TUI,
}

impl FromStr for ClientType {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_ref() {
			"logger" => Ok(Self::Logger),
			#[cfg(feature = "client_wgpu")]
			"wgpu" => Ok(Self::WGPU),
			#[cfg(feature = "client_tui")]
			"tui" => Ok(Self::TUI),
			_ => Err("invalid value (logger, wgpu, or tui"),
		}
	}
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
	pub client_type: ClientType,
	pub game_configuration_path: Option<PathBuf>,
}

/// Central engine entrance point, start by calling `Engine::new()` and call its functions
/// as appropriate and finish with `.run()` to execute it.
impl Engine {
	pub fn new(config_dir: impl Into<PathBuf>) -> Result<Engine, EngineError<Infallible>> {
		let config_dir = config_dir.into();
		logger::init_logging(Some(config_dir.as_path()))?;
		Ok(Engine {
			config_dir: config_dir,
			logging_level_override: None,
			include_server: true,
			client_type: ClientType::Logger,
			#[cfg(feature = "server")]
			game_configuration_path: None,
		})
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
		let mut app_builder = App::build();

		let asset_folder = std::env::current_dir()
			.map(|mut p| {
				p.push("assets");
				p.to_string_lossy().to_string()
			})
			.unwrap_or_else(|_| "assets".to_owned());
		info!("Setting base assets directory to: {:?}", &asset_folder);
		app_builder.insert_resource(AssetServerSettings { asset_folder });

		app_builder.add_plugins(crate::universal::UniversalPluginGroup::default());

		// Make sure server is added before clients so its runner won't override the client runner
		if self.include_server {
			#[cfg(feature = "server")]
			app_builder.add_plugins(crate::server::ServerPluginGroup::default());
		}

		match self.client_type {
			ClientType::Logger => {
				app_builder
					.add_plugin(bevy::app::ScheduleRunnerPlugin::default())
					.add_system(shut_down_when_server_is_off.system());
			}
			#[cfg(feature = "client_wgpu")]
			ClientType::WGPU => {
				app_builder.add_plugins(crate::client_wgpu::ClientWgpuPluginGroup::default());
			}
			#[cfg(feature = "client_tui")]
			ClientType::TUI => {
				app_builder.add_plugins(crate::client_tui::ClientTuiPluginGroup::default());
			}
		}

		#[cfg(feature = "server")]
		if let Some(path) = self.game_configuration_path.clone() {
			app_builder
				.app
				.world
				.get_resource_mut::<Events<LocalServerCommand>>()
				.expect("`LocalServerCommand` event is missing")
				.send(LocalServerCommand::CreateStartServer {
					path,
					config_only_if_not_existing: true,
				})
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

	pub fn set_client_type(&mut self, client_type: ClientType) -> &mut Self {
		self.client_type = client_type;
		self
	}

	#[cfg(feature = "server")]
	pub fn load_game_configuration(
		&mut self,
		game_configuration_path: Option<PathBuf>,
	) -> &mut Self {
		self.game_configuration_path = game_configuration_path;
		self
	}
}

fn shut_down_when_server_is_off(
	mut server_state_change: EventReader<LocalServerPublicState>,
	mut exit: EventWriter<RequestExit>,
	mut skipped_first: Local<bool>,
) {
	for state in server_state_change.iter() {
		if *state == LocalServerPublicState::Off {
			if *skipped_first {
				info!("Server is now Off with no client, requesting shut down");
				exit.send(RequestExit);
			} else {
				// Skip first `on_enter` off state for the server
				*skipped_first = true;
			}
		}
	}
}
