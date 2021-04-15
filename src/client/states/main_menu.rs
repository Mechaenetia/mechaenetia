use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::MainMenu;
	app.add_plugin(EguiPlugin)
		.add_system(update_egui_scale_factor.system())
		.init_resource::<Option<MainMenuState>>()
		.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(SystemSet::on_update(state.clone()).with_system(on_update.system()))
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn update_egui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
	if let Some(window) = windows.get_primary() {
		egui_settings.scale_factor = 1.0 / window.scale_factor();
	}
}

struct MainMenuState {}

impl MainMenuState {
	fn new() -> Self {
		trace!("Creating main menu");
		MainMenuState {}
	}

	fn render(&mut self, e: &mut EguiContext, state: &mut ResMut<State<super::ClientState>>) {
		// egui::CentralPanel::default().show(e.ctx(), |ui| {
		// 	ui.centered_and_justified(|ui| {
		// 		ui.label("Hello World!");
		// 	});
		// });
		egui::Window::new("Main Menu")
			.collapsible(false)
			.resizable(false)
			.show(e.ctx(), |ui| {
				ui.label("Hello World!");
				if ui.button("Exit").clicked() {
					state.overwrite_replace(super::ClientState::Exiting).expect(
						"failed transitioning to exiting state by clicking exit main menu button",
					);
				}
			});
	}
}

fn on_enter(mut main_menu_state: ResMut<Option<MainMenuState>>) {
	trace!("MainMenu State: Enter");
	// Make the main menu entity
	*main_menu_state = Some(MainMenuState::new());
}

fn on_update(
	mut egui_ctx: ResMut<EguiContext>,
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	mut state: ResMut<State<super::ClientState>>,
) {
	// trace!("MainMenu State: Update");
	if let Some(m) = &mut *main_menu_state {
		m.render(&mut *egui_ctx, &mut state);
	}
}

fn on_exit(mut main_menu_state: ResMut<Option<MainMenuState>>) {
	trace!("MainMenu State: Exit");
	*main_menu_state = None;
}
