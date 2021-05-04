mod conditional_append_appender;
mod launch_roll_file_appender;

use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::runtime::ConfigErrors;
use log4rs::config::{Appender, Config, Deserializers, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::{Path, PathBuf};
use tracing::log::{LevelFilter, SetLoggerError};

const DEFAULT_LOGGING_DEFINITION: &'static str = r#"refresh_rate: 30 seconds

appenders:
  console:
    kind: conditional_appender
    id: console
    appender:
      kind: console
      encoder:
        kind: pattern
        pattern: "{d} [{t}:{I}:{T}] {h({l})} {M}: {m}{n}"
  log_file:
    kind: launch_roll_file
    path: log/current.log
    launch_roller:
      kind: fixed_window
      count: 5
      pattern: log/previous-{}.log
    appender:
      kind: file
      path: log/current.log
      encoder:
        kind: pattern
        pattern: "{d} [{t}:{I}:{T}] {h({l})} {M}: {m}{n}"

root:
  level: trace
  appenders:
    - console
    - log_file

loggers:
  tracing::span:
    level: warn
  gpu_alloc:
    level: warn
  gfx_backend_vulkan:
    level: warn
  wgpu_core:
    level: warn
  gpu_descriptor:
    level: info
  bevy_wgpu:
    level: info
  bevy_app::event:
    level: info
  mio::poll:
    level: info
  naga:
    level: info
  bevy_app::plugin_group:
    level: warn
  bevy_app::app_builder:
    level: warn
  bevy_winit:
    level: info
  bevy_ecs::schedule:
    level: warn
"#;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Unable to create configuration directory at: {0:?}")]
	CreateDirError(PathBuf, #[source] std::io::Error),
	#[error("Unable to write missing default `log4rs.yml` file at: {0:?}")]
	UnableToWriteDefaultConfig(PathBuf, #[source] std::io::Error),
	#[error("Unable to initialize logging system from configuration file")]
	UnableToInitializeLoggingSystem(#[from] anyhow::Error),
	#[error("Unable to configure logging system")]
	ConfigFailure(#[from] ConfigErrors),
	#[error("Unable to initialize logging system from configuration")]
	ConfigurationInit(#[from] SetLoggerError),
}

/// Initializes the logging system, panics on failure
pub fn init_logging(config_dir: Option<&Path>) -> Result<(), Error> {
	match config_dir {
		Some(path) => {
			if !path.is_dir() {
				std::fs::create_dir_all(&path)
					.map_err(|e| Error::CreateDirError(path.into(), e))?;
			}
			let logger_config = {
				let mut path: PathBuf = path.into();
				path.push("log4rs.yml");
				if !path.is_file() {
					std::fs::write(&path, DEFAULT_LOGGING_DEFINITION)
						.map_err(|e| Error::UnableToWriteDefaultConfig(path.clone(), e))?;
				}
				path
			};
			let mut deserializers = Deserializers::new();
			deserializers.insert(
				"launch_roll_file",
				launch_roll_file_appender::RollFileOnLaunchAppenderDeserializer,
			);
			deserializers.insert(
				"conditional_appender",
				conditional_append_appender::ConditionallyAppendAppenderDeserializer,
			);
			log4rs::init_file(logger_config, deserializers)?;
		}
		None => {
			let stderr = ConsoleAppender::builder()
				.target(Target::Stderr)
				.encoder(Box::new(PatternEncoder::new(
					"{d} [{t}:{I}:{T}] {h({l})} {M}: {m}{n}",
				)))
				.build();

			let config = Config::builder()
				.appender(Appender::builder().build("stderr", Box::new(stderr)))
				.logger(Logger::builder().build(env!("CARGO_PKG_NAME"), LevelFilter::Warn))
				.build(Root::builder().appender("stderr").build(LevelFilter::Warn))?;

			let _logger_handle = log4rs::init_config(config)?;
		}
	};
	Ok(())
}
