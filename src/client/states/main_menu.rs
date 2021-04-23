use crate::universal::exit::{Exiting, RequestExit};
use crate::universal::i18n::{
	scan_languages_on_fs, I18NChangeLanguageTo, I18NLanguageChangedEvent,
};
use crate::universal::local_server::{CreateServer, LocalServerEvent};
use crate::universal::I18N;
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn register_systems(app: &mut AppBuilder) {
	let state = super::ClientState::MainMenu;
	app.add_plugin(EguiPlugin)
		.add_system(update_egui_scale_factor.system())
		.init_resource::<Option<MainMenuState>>()
		.add_system_set(SystemSet::on_enter(state.clone()).with_system(on_enter.system()))
		.add_system_set(
			SystemSet::on_update(state.clone())
				.with_system(on_update.system())
				.with_system(update_language.system())
				.with_system(on_shutdown.system()),
		)
		.add_system_set(SystemSet::on_exit(state.clone()).with_system(on_exit.system()));
}

fn update_egui_scale_factor(mut egui_settings: ResMut<EguiSettings>, windows: Res<Windows>) {
	if let Some(window) = windows.get_primary() {
		egui_settings.scale_factor = 1.0 / window.scale_factor();
	}
}

fn update_language(
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	lang: Res<I18N>,
	mut event: EventReader<I18NLanguageChangedEvent>,
) {
	if event.iter().next().is_some() {
		if let Some(menu) = &mut *main_menu_state {
			menu.update_language(&lang);
		}
	}
}

#[derive(PartialEq, Eq)]
enum MainMenuScreen {
	Empty,
	LocalServer,
	JoinServer,
	Settings,
}

impl Default for MainMenuScreen {
	fn default() -> Self {
		MainMenuScreen::Empty
	}
}

#[derive(Default)]
struct MainMenuState {
	cur_lang: String,
	possible_languages: Vec<String>,
	l_title: String,
	l_quit: String,
	l_server_local: String,
	l_server_local_test: String,
	l_server_join: String,
	l_settings_title: String,
	l_settings_cancel: String,
	l_settings_current_language: String,
	l_settings_choose_language: String,
	screen: MainMenuScreen,
}

impl MainMenuState {
	fn new(lang: &I18N) -> Self {
		trace!("Creating main menu");
		let mut menu = MainMenuState::default();
		menu.update_language(lang);
		menu
	}

	fn update_language(&mut self, lang: &I18N) {
		self.cur_lang = lang.get_current_language().to_string();
		self.possible_languages = scan_languages_on_fs()
			.unwrap_or(vec![])
			.iter()
			.map(|l| l.to_string())
			.collect();
		self.possible_languages.sort();
		self.l_title = lang.get("title").into_owned();
		self.l_quit = lang.get("quit").into_owned();
		self.l_server_local = lang.get("menu-server-local").into_owned();
		self.l_server_local_test = lang.get_attr("menu-server-local", "test").into_owned();
		self.l_server_join = lang.get("menu-server-join").into_owned();
		self.l_settings_title = lang.get("settings-title").into_owned();
		self.l_settings_cancel = lang.get("settings-cancel").into_owned();
		self.l_settings_current_language = lang.get("settings_current_language").into_owned();
		self.l_settings_choose_language = lang.get("settings_choose_language").into_owned();
	}

	fn render(
		&mut self,
		e: &mut EguiContext,
		state: &mut ResMut<State<super::ClientState>>,
		_windows: &Windows,
		change_lang: &mut EventWriter<I18NChangeLanguageTo>,
		local_server: &mut LocalServerEvent<CreateServer>,
		exit: &mut EventWriter<RequestExit>,
	) {
		egui::TopPanel::top("top_title").show(e.ctx(), |ui| {
			ui.centered_and_justified(|ui| {
				ui.heading(&self.l_title);
			});
		});
		egui::SidePanel::left("news_panel", 150.0).show(e.ctx(), |ui| {
			self.render_main_menu(ui, local_server, exit);
		});
		egui::CentralPanel::default().show(e.ctx(), |ui| {
			match self.screen {
				MainMenuScreen::Empty => (),
				MainMenuScreen::LocalServer => self.render_server_local(ui, state, local_server),
				MainMenuScreen::JoinServer => self.render_server_join(ui, state),
				MainMenuScreen::Settings => self.render_settings(ui, state, change_lang),
			};
		});
	}

	fn render_main_menu(
		&mut self,
		ui: &mut Ui,
		local_server: &mut LocalServerEvent<CreateServer>,
		exit: &mut EventWriter<RequestExit>,
	) {
		ui.vertical_centered_justified(|ui| {
			let menu_btn =
				|ui: &mut Ui, screen: &mut MainMenuScreen, state: MainMenuScreen, text: &str| {
					if ui
						.add(egui::Button::new(text).enabled(*screen != state))
						.clicked()
					{
						*screen = state
					}
				};

			if local_server.exists() {
				menu_btn(
					ui,
					&mut self.screen,
					MainMenuScreen::LocalServer,
					&self.l_server_local,
				);
			}
			menu_btn(
				ui,
				&mut self.screen,
				MainMenuScreen::JoinServer,
				&self.l_server_join,
			);
			menu_btn(
				ui,
				&mut self.screen,
				MainMenuScreen::Settings,
				&self.l_settings_title,
			);

			if ui.button(&self.l_quit).clicked() {
				exit.send(RequestExit);
			};
		});
	}

	fn render_server_local(
		&mut self,
		ui: &mut Ui,
		state: &mut ResMut<State<super::ClientState>>,
		local_server: &mut LocalServerEvent<CreateServer>,
	) {
		ui.centered_and_justified(|ui| {
			ui.vertical(|ui| {
				if let Some(create_server) = local_server.event() {
					if ui.button(&self.l_server_local_test).clicked() {
						create_server.send(CreateServer::new("A New Server".to_owned()));
						state
							.push(super::ClientState::JoinGame)
							.expect("Unable to switch to JoinGame state");
					}
				}
			});
		});
	}

	fn render_server_join(&mut self, ui: &mut Ui, _state: &mut ResMut<State<super::ClientState>>) {
		ui.label("todo");
	}

	fn render_settings(
		&mut self,
		ui: &mut Ui,
		_state: &mut ResMut<State<super::ClientState>>,
		change_lang: &mut EventWriter<I18NChangeLanguageTo>,
	) {
		ui.centered_and_justified(|ui| {
			let mut frame = egui::Frame::none()
				.fill(egui::Color32::from_rgb(32, 0, 0))
				.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 32, 0)));
			frame.margin = (5.0, 5.0).into();
			frame.show(ui, |ui| {
				ui.vertical(|ui| {
					ui.heading(&self.l_settings_title);
					ui.separator();
					ui.horizontal(|ui| {
						ui.heading(&self.l_settings_current_language);
						ui.label(&self.cur_lang);
					});
					ui.heading(&self.l_settings_choose_language);
					ui.horizontal_wrapped(|ui| {
						for lang in &self.possible_languages {
							if ui.radio(lang == &self.cur_lang, lang).clicked() {
								change_lang.send(I18NChangeLanguageTo(lang.parse().expect(
									"This was already confirmed valid, so why did this fail?",
								)));
							}
						}
					});
					ui.separator();
					if ui.button(&self.l_settings_cancel).clicked() {
						self.screen = MainMenuScreen::Empty;
					}
				});
			});
		});
		// ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(64, 0, 0);
		// ui.vertical(|ui| {
		// 	ui.heading(&self.l_settings_title);
		// 	ui.separator();
		// 	if ui.button(&self.l_settings_cancel).clicked() {
		// 		self.screen = MainMenuScreen::Empty;
		// 	}
		// });
	}
}

fn on_enter(mut main_menu_state: ResMut<Option<MainMenuState>>, lang: Res<I18N>) {
	trace!("Client MainMenu State: Enter");
	// Make the main menu entity
	*main_menu_state = Some(MainMenuState::new(&lang));
}

fn on_update(
	mut egui_ctx: ResMut<EguiContext>,
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	mut state: ResMut<State<super::ClientState>>,
	windows: Res<Windows>,
	mut change_lang: EventWriter<I18NChangeLanguageTo>,
	mut local_server: LocalServerEvent<CreateServer>,
	mut exit: EventWriter<RequestExit>,
) {
	// trace!("Client MainMenu State: Update");
	if let Some(m) = &mut *main_menu_state {
		m.render(
			&mut *egui_ctx,
			&mut state,
			&*windows,
			&mut change_lang,
			&mut local_server,
			&mut exit,
		);
	}
}

fn on_exit(mut main_menu_state: ResMut<Option<MainMenuState>>) {
	trace!("Client MainMenu State: Exit");
	*main_menu_state = None;
}

fn on_shutdown(exiting: Option<Res<Exiting>>, mut state: ResMut<State<super::ClientState>>) {
	if let Some(_exiting) = exiting {
		state
			.overwrite_replace(super::ClientState::Exiting)
			.expect("Failed to transition Client to exiting state");
	}
}
