#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::diagnostic::{Diagnostics, DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use clap::Parser;
use mechaenetia_engine::states::SimState;
use mechaenetia_engine::EnginePlugins;
use mechaenetia_utils::logging;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub struct Args {
	#[command(flatten)]
	log_args: logging::LogArgs,

	/// Path to a directory to store configuration files
	#[clap(short, long)]
	config_dir: Option<PathBuf>,

	/// Hot reload assets by watching for changes
	#[clap(long)]
	hot_reload_assets: bool,
}

fn main() {
	let args = Args::parse();
	// logging::init_logger(&args.log_args)?;
	App::new()
		.add_plugin(bevy::log::LogPlugin {
			level: args.log_args.verbosity,
			filter: args.log_args.log_filter,
		})
		.add_plugins(MinimalPlugins)
		.add_plugin(AssetPlugin {
			watch_for_changes: args.hot_reload_assets,
			..Default::default()
		})
		.add_plugin(TransformPlugin)
		.add_plugin(HierarchyPlugin)
		.add_plugin(DiagnosticsPlugin)
		.add_plugin(LogDiagnosticsPlugin::default())
		.add_plugin(FrameTimeDiagnosticsPlugin)
		.add_plugins(EnginePlugins {
			default_save_path: PathBuf::from("save/default/"),
		})
		.add_startup_system(setup)
		// .add_system(print_diagnostics) // This is... a lot, should be throttled
		.run();
}

fn setup(mut sim_state: ResMut<NextState<SimState>>) {
	sim_state.set(SimState::Loaded);
}

fn print_diagnostics(diagnostics: Res<Diagnostics>) {
	for diagnostic in diagnostics.iter().filter(|d| d.is_enabled) {
		println!(
			"{}: {}  avg: {}",
			diagnostic.name,
			diagnostic.value().unwrap_or_default(),
			diagnostic.average().unwrap_or_default()
		);
	}
}
