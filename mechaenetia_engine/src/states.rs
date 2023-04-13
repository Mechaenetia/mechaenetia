use bevy::prelude::*;

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
		app.add_state::<SimulationState>();
	}
}
