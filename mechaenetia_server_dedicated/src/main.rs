#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

use bevy::asset::AssetMetaCheck;
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
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

	/// Hot reload assets by watching for changes with delay of milliseconds
	#[clap(long, default_value = "None")]
	hot_reload_assets: Option<bool>,
}

fn main() {
	let args = Args::parse();
	// logging::init_logger(&args.log_args)?;
	App::new()
		.add_plugins(bevy::log::LogPlugin {
			level: args.log_args.verbosity,
			filter: args.log_args.log_filter,
			custom_layer: |_app| None,
		})
		.add_plugins(MinimalPlugins)
		.add_plugins(AssetPlugin {
			watch_for_changes_override: args.hot_reload_assets,
			file_path: args
				.config_dir
				.as_ref()
				.and_then(|p| p.join("assets").to_str().map(std::borrow::ToOwned::to_owned))
				.unwrap_or_else(|| "assets".to_owned()),
			processed_file_path: args
				.config_dir
				.as_ref()
				.and_then(|p| {
					p.join("imported_assets")
						.join("Default")
						.to_str()
						.map(std::borrow::ToOwned::to_owned)
				})
				.unwrap_or_else(|| "imported_assets/Default".to_owned()),
			mode: AssetMode::Unprocessed,
			meta_check: AssetMetaCheck::Always,
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
