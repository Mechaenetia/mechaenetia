use crate::universal::i18n::{
	scan_languages_on_fs, I18NChangeLanguageTo, I18NLanguageChangedEvent,
};
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
				.with_system(update_language.system()),
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
	) {
		egui::TopPanel::top("top_title").show(e.ctx(), |ui| {
			ui.centered_and_justified(|ui| {
				ui.heading(&self.l_title);
			});
		});
		egui::SidePanel::left("news_panel", 150.0).show(e.ctx(), |ui| {
			self.render_main_menu(state, ui);
		});
		egui::CentralPanel::default().show(e.ctx(), |ui| {
			match self.screen {
				MainMenuScreen::Empty => (),
				MainMenuScreen::Settings => self.render_settings(state, ui, change_lang),
			};
		});
	}

	fn render_main_menu(&mut self, state: &mut ResMut<State<super::ClientState>>, ui: &mut Ui) {
		ui.vertical_centered_justified(|ui| {
			let settings_but = egui::Button::new(&self.l_settings_title)
				.enabled(self.screen != MainMenuScreen::Settings);
			if ui.add(settings_but).clicked() {
				self.screen = MainMenuScreen::Settings;
			}
			if ui.button(&self.l_quit).clicked() {
				state.overwrite_replace(super::ClientState::Exiting).expect(
					"failed transitioning to exiting state by clicking exit main menu button",
				);
			};
		});
	}

	fn render_settings(
		&mut self,
		_state: &mut ResMut<State<super::ClientState>>,
		ui: &mut Ui,
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
	trace!("MainMenu State: Enter");
	// Make the main menu entity
	*main_menu_state = Some(MainMenuState::new(&lang));
}

fn on_update(
	mut egui_ctx: ResMut<EguiContext>,
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	mut state: ResMut<State<super::ClientState>>,
	windows: Res<Windows>,
	mut change_lang: EventWriter<I18NChangeLanguageTo>,
) {
	// trace!("MainMenu State: Update");
	if let Some(m) = &mut *main_menu_state {
		m.render(&mut *egui_ctx, &mut state, &*windows, &mut change_lang);
	}
}

fn on_exit(mut main_menu_state: ResMut<Option<MainMenuState>>) {
	trace!("MainMenu State: Exit");
	*main_menu_state = None;
}
