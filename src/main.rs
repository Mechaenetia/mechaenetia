use anyhow::Context;
use mechaenetia::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::log::LevelFilter;
use unic_langid::LanguageIdentifier;

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

	/// Load game configuration file, this will generate a new file then exit if it doesn't exit so
	/// as to allow it to be filled out manually before actually loading it.
	#[structopt(long)]
	load_game: Option<PathBuf>,

	/// Override the in-game language via the specified language code
	#[structopt(long)]
	language: Option<LanguageIdentifier>,
}

fn main() -> anyhow::Result<()> {
	let opts = CLIOpts::from_args();
	Engine::new(opts.config_dir.unwrap_or(PathBuf::from("./config")))
		.override_logging_level(opts.log_level)
		.load_game_configuration(opts.load_game)
		.set_include_server(!opts.no_server)
		.set_include_client(!opts.no_client)
		.run()
		.context("Failed to run the engine")
}
