use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::log::LevelFilter;
use tracing::*;

// pub mod experimenting;
// pub mod init;

#[derive(StructOpt, Debug)]
#[structopt(name = "Mechaenetia", about = "Mechaenetia Game")]
pub struct CLIOpts {
	/// Logging level override to bypass the logging config, can be: off, error, warn, info, debug, trace
	#[structopt(short, long, parse(try_from_str))]
	log_level: Option<LevelFilter>,

	/// Path to a directory to store configuration files
	#[structopt(short, long, parse(from_str))]
	config_dir: Option<PathBuf>,

	/// Do not include the client code, this will make it headless
	#[structopt(long)]
	no_client: bool,

	/// Do not include server code, this will allow you to join a server but not play locally
	#[structopt(long)]
	no_server: bool,
}

fn main() -> anyhow::Result<()> {
	let opts = CLIOpts::from_args();
	let mut game =
		mechaenetia::Mechaenetia::new(opts.config_dir.unwrap_or(PathBuf::from("./config")));

	if let Some(override_log_level) = opts.log_level {
		game.override_logging_level(override_log_level);
	}

	game.set_include_server(!opts.no_server);
	game.set_include_client(!opts.no_client);

	game.run().context("Failed to run the engine")
}
