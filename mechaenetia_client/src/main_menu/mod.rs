mod style;

use crate::main_menu::style::{
	BUTTON_TEXT_FONT, CLICKED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, LOGO_NODE, MAIN_MENU_BUTTON_NODE, MAIN_MENU_NODE,
	MAIN_MENU_TITLE_NODE, NORMAL_BUTTON_COLOR,
};
use crate::states::InterfaceState;
use bevy::app::AppExit;
use bevy::color::palettes::css;
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
		exit.send(AppExit::error());
		return;
	};
	commands.spawn((
		Camera2d,
		Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
		UIMainMenuCleanup,
	));

	commands
		.spawn((
			Node {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			UIMainMenuCleanup,
		))
		.with_children(|parent| {
			parent.spawn(MAIN_MENU_NODE()).with_children(|parent| {
				parent.spawn(MAIN_MENU_TITLE_NODE()).with_children(|parent| {
					parent.spawn(LOGO_NODE(&asset_server));
					parent
						.spawn(Node {
							justify_content: JustifyContent::Center,
							..default()
						})
						.with_child((
							Text::new("Mechaenetia!"),
							TextFont {
								font_size: 64.0,
								..BUTTON_TEXT_FONT(&asset_server)
							},
							TextColor(css::WHITE.into()),
						));
					parent.spawn(LOGO_NODE(&asset_server));
				});
				// Direct Launch Test Game button
				parent
					.spawn((
						Button,
						BackgroundColor::from(NORMAL_BUTTON_COLOR),
						Node {
							..MAIN_MENU_BUTTON_NODE()
						},
						UIMainMenuBtn::StartGame,
					))
					.with_child((Text::new("Start"), BUTTON_TEXT_FONT(&asset_server)));
				// Quit button
				parent
					.spawn((
						Button,
						BackgroundColor::from(NORMAL_BUTTON_COLOR),
						Node {
							..MAIN_MENU_BUTTON_NODE()
						},
						UIMainMenuBtn::Quit,
					))
					.with_child((Text::new("Quit"), BUTTON_TEXT_FONT(&asset_server)));
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
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		exit.send(AppExit::Success);
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
						exit.send(AppExit::Success);
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
