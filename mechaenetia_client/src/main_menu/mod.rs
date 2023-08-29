mod style;

use crate::main_menu::style::{
	get_button_text_style, get_title_text_style, BUTTON_STYLE, CLICKED_BUTTON_COLOR, HOVERED_BUTTON_COLOR,
	MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, TITLE_IMAGE_STYLE, TITLE_STYLE,
};
use crate::states::InterfaceState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[allow(clippy::module_name_repetitions)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(InterfaceState::MainMenu), spawn_main_menu)
			.add_systems(Update, interact_main_menu.run_if(in_state(InterfaceState::MainMenu)))
			.add_systems(OnExit(InterfaceState::MainMenu), despawn_main_menu);
	}
}

#[derive(Component)]
struct UIMainMenuCleanup;

#[derive(Component)]
enum UIMainMenuBtn {
	StartGame,
	Quit,
}

#[tracing::instrument(skip(commands, asset_server, exit))]
fn spawn_main_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mut exit: EventWriter<AppExit>,
) {
	let Ok(window) = window_query.get_single() else {
		error!("Failed to get window size for main menu");
		exit.send(AppExit);
		return;
	};
	commands.spawn((
		Camera2dBundle {
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			..default()
		},
		UIMainMenuCleanup,
	));

	commands
		.spawn((
			NodeBundle {
				style: MAIN_MENU_STYLE,
				..default()
			},
			UIMainMenuCleanup,
		))
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: TITLE_STYLE,
					..default()
				})
				.with_children(|parent| {
					parent.spawn(ImageBundle {
						style: TITLE_IMAGE_STYLE,
						image: asset_server.load("logo.png").into(),
						..default()
					});
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Mechaenetia", get_title_text_style(&asset_server))],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
					parent.spawn(ImageBundle {
						style: TITLE_IMAGE_STYLE,
						image: asset_server.load("logo.png").into(),
						..default()
					});
				});
			// Direct Launch Test Game button
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: NORMAL_BUTTON_COLOR.into(),
						..default()
					},
					UIMainMenuBtn::StartGame,
				))
				.with_children(|parent| {
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Start", get_button_text_style(&asset_server))],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
				});
			// Quit button
			parent
				.spawn((
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: NORMAL_BUTTON_COLOR.into(),
						..default()
					},
					UIMainMenuBtn::Quit,
				))
				.with_children(|parent| {
					parent.spawn(TextBundle {
						text: Text {
							sections: vec![TextSection::new("Quit", get_button_text_style(&asset_server))],
							alignment: TextAlignment::Center,
							..default()
						},
						..default()
					});
				});
		});
}

#[tracing::instrument(skip(commands))]
fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<UIMainMenuCleanup>>) {
	for entity in query.iter() {
		commands.entity(entity).despawn_recursive();
	}
}

#[tracing::instrument(skip(exit))]
fn interact_main_menu(
	mut btn_query: Query<(&Interaction, &mut BackgroundColor, &UIMainMenuBtn), Changed<Interaction>>,
	mut exit: EventWriter<AppExit>,
	mut interface_state: ResMut<NextState<InterfaceState>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		exit.send(AppExit);
		return;
	}
	for (interaction, mut background_color, btn) in &mut btn_query {
		match interaction {
			Interaction::Pressed => {
				*background_color = CLICKED_BUTTON_COLOR.into();
				match btn {
					UIMainMenuBtn::StartGame => {
						interface_state.set(InterfaceState::Sim);
					}
					UIMainMenuBtn::Quit => {
						exit.send(AppExit);
					}
				}
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = NORMAL_BUTTON_COLOR.into();
			}
		}
	}
}
