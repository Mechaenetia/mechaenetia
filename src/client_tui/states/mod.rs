// mod exiting;
// mod loading;
// mod main_menu;

mod exiting;
mod not_connected;

use crate::client_tui::tui_plugin::Frame;
use bevy::ecs::world::WorldCell;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientState {
	NotConnected,
	// MainMenu,
	// JoinGame,
	// Joining,
	// Joined,
	// Paused,
	Exiting,
}

#[derive(Default)]
pub struct ClientStatePlugin;

impl Plugin for ClientStatePlugin {
	fn build(&self, app: &mut AppBuilder) {
		// Add the Client state into the system.
		app.add_state(ClientState::NotConnected);
		not_connected::register_systems(app);
		// main_menu::register_systems(app);
		exiting::register_systems(app);
	}
}

impl ClientState {
	pub fn draw(&self, world: &WorldCell, f: &mut Frame) {
		match self {
			ClientState::NotConnected => not_connected::draw(world, f),
			ClientState::Exiting => exiting::draw(world, f),
		}
	}
}
