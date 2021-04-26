use crate::universal::exit::Exiting;
use crate::universal::local_server::{LocalServerCommand, LocalServerPublicState};
use bevy::prelude::*;

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ServerState::Unloading;
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
	trace!("Server Unloading State: Enter");
	*public_state = LocalServerPublicState::ShuttingDown;
	update_public_state.send(public_state.clone());
}

fn on_update(mut state: ResMut<State<super::ServerState>>) {
	// trace!("Server Unloading State: Update");
	state
		.set(super::ServerState::NotRunning)
		.expect("failed changing to state NotRunning after Unloading");
}

fn on_exit() {
	trace!("Server Unloading State: Exit");
}

fn on_shutdown(exiting: Option<Res<Exiting>>, mut state: ResMut<State<super::ServerState>>) {
	if let Some(_exiting) = exiting {
		state
			.overwrite_replace(super::ServerState::Exiting)
			.expect("Failed to transition Server to exiting state");
	}
}

fn on_server_public_cmd(mut cmds: EventReader<LocalServerCommand>) {
	for cmd in cmds.iter() {
		match cmd {
			LocalServerCommand::StartServer { title: _ } => {
				error!("Cannot load a server while unloading");
			}
			LocalServerCommand::StopServer { force: _ } => {
				// Already stopping...
			}
		}
	}
}
