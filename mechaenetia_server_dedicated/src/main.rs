#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::asset::ChangeWatcher;
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use clap::Parser;
use mechaenetia_engine::states::SimState;
use mechaenetia_engine::EnginePlugins;
use mechaenetia_utils::logging;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Clone, Debug)]
pub struct Args {
	#[command(flatten)]
	log_args: logging::LogArgs,

	/// Path to a directory to store configuration files
	#[clap(short, long)]
	config_dir: Option<PathBuf>,

	/// Hot reload assets by watching for changes with delay of milliseconds
	#[clap(long)]
	hot_reload_assets: Option<u64>,
}

fn main() {
	let args = Args::parse();
	// logging::init_logger(&args.log_args)?;
	App::new()
		.add_plugins(bevy::log::LogPlugin {
			level: args.log_args.verbosity,
			filter: args.log_args.log_filter,
		})
		.add_plugins(MinimalPlugins)
		.add_plugins(AssetPlugin {
			watch_for_changes: args
				.hot_reload_assets
				.map(Duration::from_millis)
				.and_then(ChangeWatcher::with_delay),
			..Default::default()
		})
		.add_plugins(TransformPlugin)
		.add_plugins(HierarchyPlugin)
		.add_plugins(DiagnosticsPlugin)
		.add_plugins(SystemInformationDiagnosticsPlugin)
		.add_plugins(FrameTimeDiagnosticsPlugin)
		// These system logs don't log on dynamic builds as a fair warning...
		// .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
		.add_plugins(EnginePlugins {
			default_save_path: PathBuf::from("save/default/"),
		})
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut sim_state: ResMut<NextState<SimState>>) {
	sim_state.set(SimState::Loaded);
}
