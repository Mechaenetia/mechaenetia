use crate::universal::exit::{Exiting, RequestExit};
use crate::universal::i18n::{
	scan_languages_on_fs, I18nChangeLanguageTo, I18nLanguageChangedEvent, MsgCache, MsgKey,
};
use crate::universal::local_server::{LocalServerCommand, LocalServerPublicState};
use crate::universal::I18n;
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use fluent::types::{FluentNumber, FluentNumberOptions, FluentNumberStyle};
use std::path::PathBuf;

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
				.with_system(update_local_server_state.system())
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
	lang: Res<I18n>,
	mut event: EventReader<I18nLanguageChangedEvent>,
) {
	if event.iter().next().is_some() {
		if let Some(menu) = &mut *main_menu_state {
			menu.update_language(&lang);
		}
	}
}

fn update_local_server_state(
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	lang: Res<I18n>,
	mut state: EventReader<LocalServerPublicState>,
) {
	if let Some(state) = state.iter().last() {
		if let Some(main_menu_state) = &mut *main_menu_state {
			match state {
				LocalServerPublicState::Off => {
					main_menu_state
						.local_server_state_msg
						.attr("off")
						.update(&*lang);
				}
				LocalServerPublicState::Loading(completion) => {
					main_menu_state
						.local_server_state_msg
						.attr("loading")
						.update_args_iter(
							&*lang,
							std::iter::once((
								"completion",
								FluentNumber::new(
									*completion,
									FluentNumberOptions {
										style: FluentNumberStyle::Percent,
										..Default::default()
									},
								),
							)),
						);
				}
				LocalServerPublicState::Running => {
					main_menu_state
						.local_server_state_msg
						.attr("running")
						.update(&*lang);
				}
				LocalServerPublicState::ShuttingDown => {
					main_menu_state
						.local_server_state_msg
						.attr("shutting-down")
						.update(&*lang);
				}
			}
		}
	}
}

#[derive(PartialEq, Eq)]
enum MainMenuScreen {
	Empty,
	LocalServer,
	LoadJoinLocalServer,
	JoinServer,
	Settings,
}

impl Default for MainMenuScreen {
	fn default() -> Self {
		MainMenuScreen::Empty
	}
}

struct MainMenuState {
	cur_lang: String,
	possible_languages: Vec<String>,
	l_title: MsgCache,
	l_quit: MsgCache,
	l_server_local: MsgCache,
	l_server_local_starting: MsgCache,
	l_server_local_starting_cancel: MsgCache,
	l_server_local_test: MsgCache,
	l_server_join: MsgCache,
	l_settings_title: MsgCache,
	l_settings_cancel: MsgCache,
	l_settings_current_language: MsgCache,
	l_settings_choose_language: MsgCache,
	screen: MainMenuScreen,
	local_server_state_msg: MsgCache,
}

impl Default for MainMenuState {
	fn default() -> Self {
		Self {
			cur_lang: "".to_string(),
			possible_languages: vec![],
			l_title: MsgCache::new(MsgKey::new("title")),
			l_quit: MsgCache::new(MsgKey::new("quit")),
			l_server_local: MsgCache::new(MsgKey::new("menu-server-local")),
			l_server_local_starting: MsgCache::new(MsgKey::new("menu-server-starting")),
			l_server_local_starting_cancel: MsgCache::new(
				MsgKey::new("menu-server-starting").with_attr("cancel"),
			),
			l_server_local_test: MsgCache::new(MsgKey::new("menu-server-local").with_attr("test")),
			l_server_join: MsgCache::new(MsgKey::new("menu-server-join")),
			l_settings_title: MsgCache::new(MsgKey::new("settings-title")),
			l_settings_cancel: MsgCache::new(MsgKey::new("settings-cancel")),
			l_settings_current_language: MsgCache::new(MsgKey::new("settings_current_language")),
			l_settings_choose_language: MsgCache::new(MsgKey::new("settings_choose_language")),
			screen: Default::default(),
			local_server_state_msg: MsgCache::new(MsgKey::new("local-server-state")),
		}
	}
}

impl MainMenuState {
	fn new(lang: &I18n) -> Self {
		trace!("Creating main menu");
		let mut menu = MainMenuState::default();
		menu.update_language(lang);
		menu
	}

	fn update_language(&mut self, lang: &I18n) {
		self.cur_lang = lang.get_current_language().to_string();
		self.possible_languages = scan_languages_on_fs()
			.unwrap_or(vec![])
			.iter()
			.map(|l| l.to_string())
			.collect();
		self.possible_languages.sort();
		self.l_title.update(lang);
		self.l_quit.update(lang);
		self.l_server_local.update(lang);
		self.l_server_local_starting.update(lang);
		self.l_server_local_starting_cancel.update(lang);
		self.l_server_local_test.update(lang);
		self.l_server_join.update(lang);
		self.l_settings_title.update(lang);
		self.l_settings_cancel.update(lang);
		self.l_settings_current_language.update(lang);
		self.l_settings_choose_language.update(lang);
	}

	fn render(
		&mut self,
		e: &mut EguiContext,
		state: &mut ResMut<State<super::ClientState>>,
		_windows: &Windows,
		change_lang: &mut EventWriter<I18nChangeLanguageTo>,
		local_server_state: &Option<Res<LocalServerPublicState>>,
		local_server_cmd: &mut EventWriter<LocalServerCommand>,
		exit: &mut EventWriter<RequestExit>,
	) {
		egui::TopPanel::top("top_title").show(e.ctx(), |ui| {
			ui.centered_and_justified(|ui| {
				ui.heading(self.l_title.as_str());
			});
		});
		if self.screen == MainMenuScreen::LoadJoinLocalServer {
			self.loading_local_server(e.ctx(), local_server_state, local_server_cmd);
		} else {
			egui::SidePanel::left("news_panel", 150.0).show(e.ctx(), |ui| {
				self.render_main_menu(ui, local_server_state, exit);
			});
			egui::CentralPanel::default().show(e.ctx(), |ui| {
				match self.screen {
					MainMenuScreen::Empty => (),
					MainMenuScreen::LocalServer => {
						self.render_server_local(ui, local_server_state, local_server_cmd)
					}
					MainMenuScreen::LoadJoinLocalServer => (),
					MainMenuScreen::JoinServer => self.render_server_join(ui, state),
					MainMenuScreen::Settings => self.render_settings(ui, state, change_lang),
				};
			});
		}
	}

	fn loading_local_server(
		&mut self,
		ctx: &egui::CtxRef,
		local_server_state: &Option<Res<LocalServerPublicState>>,
		local_server_cmd: &mut EventWriter<LocalServerCommand>,
	) {
		if let Some(_local_server_state) = local_server_state {
			// if let LocalServerPublicState::Loading(msg)
			egui::CentralPanel::default().show(ctx, |ui| {
				ui.heading(self.l_server_local_starting.as_str());
				ui.label(self.local_server_state_msg.as_str());
				if ui
					.button(self.l_server_local_starting_cancel.as_str())
					.clicked()
				{
					self.screen = MainMenuScreen::Empty;
					local_server_cmd.send(LocalServerCommand::StopServer { force: true });
				}
			});
		} else {
			self.screen = MainMenuScreen::Empty;
		}
	}

	fn render_main_menu(
		&mut self,
		ui: &mut Ui,
		local_server_state: &Option<Res<LocalServerPublicState>>,
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

			if local_server_state.is_some() {
				menu_btn(
					ui,
					&mut self.screen,
					MainMenuScreen::LocalServer,
					self.l_server_local.as_str(),
				);
			}
			menu_btn(
				ui,
				&mut self.screen,
				MainMenuScreen::JoinServer,
				self.l_server_join.as_str(),
			);
			menu_btn(
				ui,
				&mut self.screen,
				MainMenuScreen::Settings,
				self.l_settings_title.as_str(),
			);

			if ui.button(self.l_quit.as_str()).clicked() {
				exit.send(RequestExit);
			};
		});
	}

	fn render_server_local(
		&mut self,
		ui: &mut Ui,
		local_server_exists: &Option<Res<LocalServerPublicState>>,
		local_server_cmd: &mut EventWriter<LocalServerCommand>,
	) {
		ui.centered_and_justified(|ui| {
			ui.vertical(|ui| {
				if local_server_exists.is_some() {
					if ui.button(self.l_server_local_test.as_str()).clicked() {
						local_server_cmd.send(LocalServerCommand::CreateStartServer {
							path: PathBuf::new().join("saves").join("local"),
							config_only_if_not_existing: false,
						});
						self.screen = MainMenuScreen::LoadJoinLocalServer;
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
		change_lang: &mut EventWriter<I18nChangeLanguageTo>,
	) {
		ui.centered_and_justified(|ui| {
			let mut frame = egui::Frame::none()
				.fill(egui::Color32::from_rgb(32, 0, 0))
				.stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 32, 0)));
			frame.margin = (5.0, 5.0).into();
			frame.show(ui, |ui| {
				ui.vertical(|ui| {
					ui.heading(self.l_settings_title.as_str());
					ui.separator();
					ui.horizontal(|ui| {
						ui.heading(self.l_settings_current_language.as_str());
						ui.label(&self.cur_lang);
					});
					ui.heading(self.l_settings_choose_language.as_str());
					ui.horizontal_wrapped(|ui| {
						for lang in &self.possible_languages {
							if ui.radio(lang == &self.cur_lang, lang).clicked() {
								change_lang.send(I18nChangeLanguageTo(vec![lang.parse().expect(
									"This was already confirmed valid, so should never fail, report this",
								)]));
							}
						}
					});
					ui.separator();
					if ui.button(self.l_settings_cancel.as_str()).clicked() {
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

fn on_enter(mut main_menu_state: ResMut<Option<MainMenuState>>, lang: Res<I18n>) {
	trace!("Client MainMenu State: Enter");
	// Make the main menu entity
	*main_menu_state = Some(MainMenuState::new(&lang));
}

fn on_update(
	mut egui_ctx: ResMut<EguiContext>,
	mut main_menu_state: ResMut<Option<MainMenuState>>,
	mut state: ResMut<State<super::ClientState>>,
	windows: Res<Windows>,
	mut change_lang: EventWriter<I18nChangeLanguageTo>,
	local_server_state: Option<Res<LocalServerPublicState>>,
	mut local_server_cmd: EventWriter<LocalServerCommand>,
	mut exit: EventWriter<RequestExit>,
) {
	// trace!("Client MainMenu State: Update");
	if let Some(m) = &mut *main_menu_state {
		m.render(
			&mut *egui_ctx,
			&mut state,
			&*windows,
			&mut change_lang,
			&local_server_state,
			&mut local_server_cmd,
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
