use bevy::diagnostic::DiagnosticsPlugin;
use bevy::prelude::*;
use clap::Parser;
use mechaenetia_utils::logging;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub struct Args {
	#[command(flatten)]
	log_args: logging::LogArgs,

	/// Path to a directory to store configuration files
	#[clap(short, long)]
	config_dir: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();
	// logging::init_logger(&args.log_args)?;
	App::new()
		.add_plugin(bevy::log::LogPlugin {
			level: args.log_args.verbosity,
			filter: args.log_args.log_filter,
		})
		.add_plugins(MinimalPlugins)
		.add_plugin(TransformPlugin)
		.add_plugin(HierarchyPlugin)
		.add_plugin(DiagnosticsPlugin)
		.run();
	Ok(())
}
