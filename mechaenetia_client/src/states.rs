use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum InterfaceState {
	#[default]
	MainMenu,
	Sim,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.init_state::<InterfaceState>()
			.add_systems(Update, back_to_main_menu.run_if(in_state(InterfaceState::Sim)));
	}
}

fn back_to_main_menu(keyboard_input: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<InterfaceState>>) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		state.set(InterfaceState::MainMenu);
	}
}
