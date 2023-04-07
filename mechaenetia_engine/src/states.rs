use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum InterfaceState {
	#[default]
	NotLoaded,
	MainMenu,
	Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum SimulationState {
	#[default]
	NotLoaded,
	Paused,
	Running,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<InterfaceState>()
			.add_state::<SimulationState>();
	}
}
