#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

mod window_icon_handling;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use clap::Parser;
use mechaenetia_client::ClientPlugins;
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
		.add_plugins(
			DefaultPlugins
				.set(bevy::log::LogPlugin {
					level: args.log_args.verbosity,
					filter: args.log_args.log_filter,
				})
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Mechaenetia".to_string(),
						present_mode: PresentMode::AutoNoVsync,
						..Default::default()
					}),
					exit_condition: bevy::window::ExitCondition::OnAllClosed,
					close_when_requested: true,
				})
				.set(AssetPlugin {
					watch_for_changes: args.hot_reload_assets,
					asset_folder: args
						.config_dir
						.as_ref()
						.and_then(|p| p.join("assets").to_str().map(std::borrow::ToOwned::to_owned))
						.unwrap_or_else(|| "assets".to_owned()),
				}),
		)
		.add_plugin(SystemInformationDiagnosticsPlugin)
		.add_plugin(FrameTimeDiagnosticsPlugin)
		// These system logs don't log on dynamic builds as a fair warning...
		// .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
		.add_plugins(EnginePlugins {
			default_save_path: args.config_dir.unwrap_or_default().join("saves/default"),
		})
		.add_plugins(ClientPlugins)
		.add_plugin(window_icon_handling::WindowIconPlugin("logo.png".into()))
		.run();
}
