use crate::server::save::SaveConfig;
use crate::universal::exit::Exiting;
use crate::universal::local_server::{LocalServerCommand, LocalServerPublicState};
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ServerState::Loading;
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
	save_config_res: Res<Option<SaveConfig>>,
) {
	trace!("Server Loading State: Enter: {:?}", &*save_config_res);
	*public_state = LocalServerPublicState::Loading(0.0);
	update_public_state.send(public_state.clone());
}

fn on_update(
	mut public_state: ResMut<LocalServerPublicState>,
	mut update_public_state: EventWriter<LocalServerPublicState>,
) {
	// trace!("Server Loading State: Update");
	if let LocalServerPublicState::Loading(completion) = &mut *public_state {
		*completion += (1.0 - *completion) * 0.01;
	}
	update_public_state.send(public_state.clone());
}

fn on_exit() {
	trace!("Server Loading State: Exit");
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
) {
	for cmd in cmds.iter() {
		match cmd {
			LocalServerCommand::CreateStartServer { .. } => {
				warn!("requested to CreateStartServer when already running a server");
			}
			LocalServerCommand::StopServer { force: _ } => {
				info!("Unloading server from within loading state");
				state.set(super::ServerState::Unloading).expect(
					"Failed transitioning to Server Unloading state from the Loading state",
				);
			}
		}
	}
}
