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
		app.add_state::<InterfaceState>()
			.add_system(back_to_main_menu.in_set(OnUpdate(InterfaceState::Sim)));
	}
}

fn back_to_main_menu(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<NextState<InterfaceState>>) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		state.set(InterfaceState::MainMenu);
	}
}
