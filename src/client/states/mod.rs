use bevy::prelude::*;

mod exiting;
mod loading;
mod main_menu;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientState {
	Loading,
	MainMenu,
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
		app.add_state(ClientState::Loading);
		loading::register_systems(app);
		main_menu::register_systems(app);
		exiting::register_systems(app);
	}
}
