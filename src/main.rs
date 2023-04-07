use std::path::PathBuf;
use bevy::prelude::*;
use clap::Parser;
use mechaenetia_utils::logging;

// use mechaenetia::{core::ClientType, prelude::*};
// use structopt::StructOpt;
// use tracing::log::LevelFilter;
// use unic_langid::LanguageIdentifier;

#[derive(Parser, Clone, Debug)]
pub struct Args {
	#[command(flatten)]
	log_args: logging::LogArgs,

	/// Path to a directory to store configuration files
	#[clap(short, long)]
	config_dir: Option<PathBuf>,
	
	// /// Which Client to select
	// #[clap(long)]
	// client: Option<ClientType>,
	// 
	// /// Do not include server code, this will allow you to join a server but not play locally
	// #[clap(long)]
	// no_server: bool,
	// 
	// /// Load game configuration file, this will generate a new file then exit if it doesn't exit so
	// /// as to allow it to be filled out manually before actually loading it.
	// #[cfg(feature = "server")]
	// #[clap(long)]
	// load_game: Option<PathBuf>,
	// 
	// /// Override the in-game language via the specified language code
	// #[clap(long)]
	// language: Option<LanguageIdentifier>,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	logging::init_logger(&args.log_args)?;
	App::new().add_plugins(DefaultPlugins).run();
	Ok(())
	
	
	// #[allow(unused_assignments)]
	// let default_client_type = {
	// 	let mut preferred_client_type = ClientType::Logger;
	// 	#[cfg(feature = "client_tui")]
	// 	{
	// 		preferred_client_type = ClientType::TUI;
	// 	}
	// 	#[cfg(feature = "client_wgpu")]
	// 	if cfg!(target_os = "linux") || std::env::var("DISPLAY").is_ok() {
	// 		preferred_client_type = ClientType::WGPU;
	// 	}
	// 	preferred_client_type
	// };
	// 
	// let opts = CLIOpts::from_args();
	// 
	// let client_type = opts.client.unwrap_or(default_client_type);
	// 
	// let mut engine = Engine::new(opts.config_dir.unwrap_or(PathBuf::from("./config")))?;
	// engine.override_logging_level(opts.log_level);
	// #[cfg(feature = "server")]
	// engine.load_game_configuration(opts.load_game.or_else(|| {
	// 	if client_type == ClientType::Logger {
	// 		tracing::warn!("Logger-only client selected but no server file was set to be loaded, defaulting to `saves/server`");
	// 		Some(PathBuf::new().join("saves").join("server"))
	// 	} else {None}
	// }));
	// engine.set_include_server(!opts.no_server);
	// engine.set_client_type(client_type);
	// engine.run().context("Failed to run the engine")
}
