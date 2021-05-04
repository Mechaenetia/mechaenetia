use crate::server::save::{SaveConfig, SaveLoadState};
use crate::universal::exit::Exiting;
use crate::universal::local_server::{LocalServerCommand, LocalServerPublicState};
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ServerState::NotRunning;
	app.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(
			SystemSet::on_update(state.clone())
				.with_system(on_update.system())
				.with_system(on_server_public_cmd.system())
				.with_system(on_shutdown.system()),
		)
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn on_enter(
	mut public_state: ResMut<LocalServerPublicState>,
	mut update_public_state: EventWriter<LocalServerPublicState>,
) {
	trace!("Server NotRunning State: Enter");
	*public_state = LocalServerPublicState::Off;
	update_public_state.send(public_state.clone());
}

fn on_update() {
	// trace!("Server NotRunning State: Update");
}

fn on_exit() {
	trace!("Server NotRunning State: Exit");
}

fn on_shutdown(exiting: Option<Res<Exiting>>, mut state: ResMut<State<super::ServerState>>) {
	if let Some(_exiting) = exiting {
		state
			.overwrite_replace(super::ServerState::Exiting)
			.expect("Failed to transition Server to exiting state");
	}
}

fn on_server_public_cmd(
	mut cmds: EventReader<LocalServerCommand>,
	mut state: ResMut<State<super::ServerState>>,
	mut update_public_state: EventWriter<LocalServerPublicState>,
	mut save_config_res: ResMut<Option<SaveConfig>>,
) {
	for cmd in cmds.iter() {
		match cmd {
			LocalServerCommand::CreateStartServer {
				path,
				config_only_if_not_existing,
			} => {
				info!("Launching server: {:?}", path);
				let save_config = match SaveConfig::load_or_create_path(&path) {
					Err(e) => {
						update_public_state.send(LocalServerPublicState::Off);
						error!(
							"Error loading or creating SaveConfig at `{:?}`: {:?}",
							&path, e
						);
						continue;
					}
					Ok(SaveLoadState::Existing(save_config)) => save_config,
					Ok(SaveLoadState::Created(save_config)) => {
						if *config_only_if_not_existing {
							info!("Created Save Configuration at path: `{:?}`", &path);
							update_public_state.send(LocalServerPublicState::Off);
							continue;
						} else {
							save_config
						}
					}
				};
				*save_config_res = Some(save_config);
				state
					.set(super::ServerState::Loading)
					.expect("Failed to transition server from NotRunning to Loading state");
			}
			LocalServerCommand::StopServer { force: _ } => {
				info!("Server Stop requested when server is already not running");
			}
		}
	}
}
