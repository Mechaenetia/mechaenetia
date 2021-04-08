use anyhow::Context;
use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::PathBuf;

const DEFAULT_LOGGING_DEFINITION: &'static str = r#"
refresh_rate: 30 seconds
appenders:
  stdout:
    kind: console
    encoder:
      pattern: "{d} [{t}:{I}:{T}] {h({l})} {M}: {m}{n}"

root:
  level: trace
  appenders:
    - stdout
"#;

/// Initializes the logging system, panics on failure
pub fn init_logging(config_dir: &Option<PathBuf>) -> anyhow::Result<()> {
	match config_dir {
		Some(path) => {
			if !path.is_dir() {
				std::fs::create_dir_all(&path).with_context(|| {
					format!("Unable to create `config_dir` directory of `{:?}`", path)
				})?;
			}
			let logger_config = {
				let mut path = path.clone();
				path.push("log4rs.yml");
				if !path.is_file() {
					std::fs::write(&path, DEFAULT_LOGGING_DEFINITION).with_context(|| {
						format!("Failed to write default `log4rs.yml` file at: {:?}", path)
					})?;
				}
				path
			};
			log4rs::init_file(logger_config, Default::default()).with_context(|| {
				format!("Failed to initialize the log4rs logging system with the `log4rs.yml` configuration file")
			})?;
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
				.logger(Logger::builder().build("tbsserver", LevelFilter::Warn))
				.build(Root::builder().appender("stderr").build(LevelFilter::Warn))
				.with_context(|| {
					format!("Failed to internally configure the log4rs logging system")
				})?;

			let _logger_handle = log4rs::init_config(config).with_context(|| {
				format!("Failed to initialize the log4rs logging system with the internal configuration")
			})?;
		}
	};
	Ok(())
}
