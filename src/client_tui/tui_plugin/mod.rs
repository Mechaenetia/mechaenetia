mod transform_event_keyboard;

use bevy::app::{AppExit, Events};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::window::{WindowCommand, WindowId, WindowMode, WindowResizeConstraints};
use crossterm::event::MouseEventKind;
use crossterm::execute;
use std::borrow::Cow;
use std::time::Duration;
use tui::backend::CrosstermBackend;

pub type Terminal = tui::Terminal<CrosstermBackend<std::io::Stdout>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<std::io::Stdout>>;

/// Terminal User Interface wrapping resource.  Acquire the usual way to create your own drawing
/// callback.  This uses `NonSend` as only one thread can write to the screen at a time.
///
/// ```rust,no_run
/// # struct TUI; struct My; struct Renderable; struct Data;
/// # impl TUI {
/// #     fn get_frame(&mut self) -> tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>> {
/// #         todo!("don't even accidentally run this during testing...")
/// #     }
/// # }
/// # use bevy::prelude::*;
/// # use tui::layout::Rect;
/// # use tui::widgets::Block;
/// fn my_bevy_draw_system(mut tui: NonSendMut<TUI>, query: Query<(&My, &Renderable, &Data)>) {
///     // Don't forget to make a startup system and call `tui.set_raw_mode(true)` once if you want
///     // fancy drawing instead of just printing lines.
///     let block = Block::default();
///     let area = Rect::new(0, 0, 5, 5);
///     let mut frame = tui.get_frame();
///     frame.render_widget(block, area);
/// }
/// ```
pub struct TUI {
	terminal: Terminal,
	is_raw: bool,
	is_mouse_enabled: bool,
	is_alternate_screen: bool,
}

impl TUI {
	/// Acquire the `tui::Terminal`.
	#[allow(dead_code)]
	pub fn term(&self) -> &Terminal {
		&self.terminal
	}

	/// Acquire the `tui::Terminal` mutably.
	#[allow(dead_code)]
	pub fn term_mut(&mut self) -> &mut Terminal {
		&mut self.terminal
	}

	/// Quicker access to make a frame from the terminal.
	#[allow(dead_code)]
	pub fn get_frame(&mut self) -> Frame {
		self.terminal.get_frame()
	}

	pub fn draw(&mut self, f: impl FnOnce(&mut Frame)) -> std::io::Result<()> {
		self.terminal.draw(f)
	}

	/// Try to set raw mode on the `stdout` output, returns a result for success or failure.
	pub fn try_set_raw_mode(&mut self, set: bool) -> crossterm::Result<()> {
		if self.is_raw == set {
			Ok(())
		} else {
			trace!(
				"raw mode was {} and is now going to be {}",
				self.is_raw,
				set
			);
			self.is_raw = set;
			if set {
				crossterm::terminal::enable_raw_mode()
			} else {
				crossterm::terminal::disable_raw_mode()
			}
		}
	}

	/// Set raw mode on the `stdout` output, panics on failure.
	#[allow(dead_code)]
	pub fn set_raw_mode(&mut self, set: bool) {
		self.try_set_raw_mode(set)
			.expect("failed to set raw mode on `stdout`");
	}

	/// Ask the terminal to change its title if its supported.
	pub fn try_set_title(&mut self, title: &str) -> crossterm::Result<()> {
		trace!("tui attempting to set title to: {}", title);
		execute!(std::io::stdout(), crossterm::terminal::SetTitle(title))
	}

	/// Ask the terminal to change its title if its supported, ignores any error.
	pub fn set_title(&mut self, title: &str) {
		let _ignore = self.try_set_title(title);
	}

	pub fn try_change_mouse_state(&mut self, enable: bool) -> crossterm::Result<()> {
		if self.is_mouse_enabled == enable {
			Ok(())
		} else {
			trace!(
				"mouse enabled was {} and is now going to be {}",
				self.is_mouse_enabled,
				enable
			);
			self.is_mouse_enabled = enable;
			if enable {
				execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
			} else {
				execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)
			}
		}
	}

	pub fn change_mouse_state(&mut self, enable: bool) {
		let _ignore = self.try_change_mouse_state(enable);
	}

	pub fn try_alternate_screen_state(&mut self, enable: bool) -> crossterm::Result<()> {
		if self.is_alternate_screen == enable {
			Ok(())
		} else {
			trace!(
				"alternate screen was {} and is now going to be {}",
				self.is_alternate_screen,
				enable
			);
			self.is_alternate_screen = enable;
			if enable {
				execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)
			} else {
				execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
			}
		}
	}

	pub fn alternate_screen_state(&mut self, enable: bool) {
		let _ignore = self.try_alternate_screen_state(enable);
	}
}

impl Drop for TUI {
	fn drop(&mut self) {
		let _ignore_set_raw_mode = self.try_set_raw_mode(false);
		self.change_mouse_state(false);
		self.alternate_screen_state(false);
		self.set_title("");
	}
}

pub struct TuiMaxEventsPerTick(usize);

pub struct TuiRunnerPlugin {
	pub title: Option<Cow<'static, str>>,
	pub max_events_per_tick: TuiMaxEventsPerTick,
	pub start_in_raw_mode: bool,
	pub start_with_mouse_captured: bool,
	pub enable_alternate_screen: bool,
}

impl<'a> Default for TuiRunnerPlugin {
	fn default() -> Self {
		Self {
			title: None,
			max_events_per_tick: TuiMaxEventsPerTick(128),
			start_in_raw_mode: false,
			start_with_mouse_captured: false,
			enable_alternate_screen: false,
		}
	}
}

impl TuiRunnerPlugin {
	pub fn title(self, title: &'static str) -> Self {
		Self {
			title: Some(Cow::Borrowed(title)),
			..self
		}
	}

	#[allow(dead_code)]
	pub fn title_owned(self, title: String) -> Self {
		Self {
			title: Some(Cow::Owned(title)),
			..self
		}
	}

	pub fn max_events_per_tick(self, max_events_per_tick: usize) -> Self {
		let max_events_per_tick = TuiMaxEventsPerTick(max_events_per_tick);
		Self {
			max_events_per_tick,
			..self
		}
	}

	pub fn start_in_raw_mode(self, start_in_raw_mode: bool) -> Self {
		Self {
			start_in_raw_mode,
			..self
		}
	}

	pub fn start_with_mouse_captured(self, start_with_mouse_captured: bool) -> Self {
		Self {
			start_with_mouse_captured,
			..self
		}
	}

	pub fn enable_alternate_screen(self, enable_alternate_screen: bool) -> Self {
		Self {
			enable_alternate_screen,
			..self
		}
	}
}

impl Plugin for TuiRunnerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		trace_span!("setting up TUI plugin");

		let stdout = std::io::stdout();
		let backend = CrosstermBackend::new(stdout);
		let mut terminal: Terminal = Terminal::new(backend).expect("unable to initialize TUI");

		let (x, y) =
			crossterm::terminal::size().expect("unable to access terminal size information");
		let window_descriptor = WindowDescriptor {
			width: x as _,
			height: y as _,
			resize_constraints: WindowResizeConstraints {
				min_width: 0.0,
				min_height: 0.0,
				max_width: u16::MAX as _,
				max_height: u16::MAX as _,
			},
			scale_factor_override: None,
			title: self
				.title
				.as_ref()
				.unwrap_or(&Cow::Borrowed(""))
				.to_string(),
			vsync: false,
			resizable: true,
			decorations: false,
			cursor_visible: true,
			cursor_locked: false,
			mode: WindowMode::Fullscreen { use_size: true },
		};

		let primary_window = Window::new(
			WindowId::primary(),
			&window_descriptor,
			x as _,
			y as _,
			1.0,
			None,
		);

		app.world_mut()
			.get_resource_mut::<Windows>()
			.expect("the `Windows` resource must have been inserted before `TuiPlugin` is added")
			.add(primary_window);

		app.world_mut()
			.get_resource_or_insert_with(|| TuiMaxEventsPerTick(128));

		let (x, y) = terminal.get_cursor().unwrap_or((0, 0));

		let mut tui = TUI {
			terminal,
			is_raw: !self.start_in_raw_mode,
			is_mouse_enabled: !self.start_with_mouse_captured,
			is_alternate_screen: !self.enable_alternate_screen,
		};
		self.title
			.as_ref()
			.map(|title| tui.set_title(title.as_ref()));
		tui.set_raw_mode(self.start_in_raw_mode);
		tui.change_mouse_state(self.start_with_mouse_captured);
		tui.alternate_screen_state(self.enable_alternate_screen);

		app.insert_non_send_resource(tui)
			.insert_resource(CursorLocation(x, y))
			.add_system_to_stage(CoreStage::First, event_poller.exclusive_system())
			.add_system_to_stage(CoreStage::Last, change_window.exclusive_system())
			.add_system_to_stage(CoreStage::Last, reset_on_exit.system());
	}
}

pub struct CursorLocation(u16, u16);

fn event_poller(world: &mut World) {
	let world = world.cell();
	let mut max_events = world.get_resource_mut::<TuiMaxEventsPerTick>().unwrap().0;
	let mut windows = world.get_resource_mut::<Windows>().unwrap();
	let mut resized: Option<(u16, u16)> = None;

	while max_events != 0 {
		max_events -= 1;
		if crossterm::event::poll(Duration::from_nanos(0)).expect("failed polling for events") {
			let event = crossterm::event::read()
				.expect("an event was ready to read but vanished before it was read?");
			use crossterm::event::Event;
			match event {
				Event::Key(key) => {
					world
						.get_resource_mut::<Events<KeyboardInput>>()
						.unwrap()
						.extend(transform_event_keyboard::to_bevy_iterator(key).map(|k| {
							trace!("tui key pressed: {:?}", k);
							k
						}));
				}
				Event::Mouse(mouse_event) => {
					trace!("TUI mouse event: {:?}", mouse_event);

					let x = mouse_event.column as f32;
					let y = mouse_event.row as f32;
					let mut mouse = world.get_resource_mut::<Events<MouseMotion>>().unwrap();
					let mut old_loc = world.get_resource_mut::<CursorLocation>().unwrap();
					let old_x = old_loc.0 as f32;
					let old_y = old_loc.1 as f32;
					mouse.send(MouseMotion {
						delta: Vec2::new(x - old_x, y - old_y),
					});
					old_loc.0 = mouse_event.column;
					old_loc.1 = mouse_event.row;

					match mouse_event.kind {
						MouseEventKind::Down(button) => {
							let button = match button {
								crossterm::event::MouseButton::Left => MouseButton::Left,
								crossterm::event::MouseButton::Right => MouseButton::Right,
								crossterm::event::MouseButton::Middle => MouseButton::Middle,
							};
							let mut mouse = world
								.get_resource_mut::<Events<MouseButtonInput>>()
								.unwrap();
							mouse.send(MouseButtonInput {
								button,
								state: ElementState::Pressed,
							});
						}
						MouseEventKind::Up(button) => {
							let button = match button {
								crossterm::event::MouseButton::Left => MouseButton::Left,
								crossterm::event::MouseButton::Right => MouseButton::Right,
								crossterm::event::MouseButton::Middle => MouseButton::Middle,
							};
							let mut mouse = world
								.get_resource_mut::<Events<MouseButtonInput>>()
								.unwrap();
							mouse.send(MouseButtonInput {
								button,
								state: ElementState::Released,
							});
						}
						MouseEventKind::Drag(_button) => { /* Already moved */ }
						MouseEventKind::Moved => { /* Already moved */ }
						MouseEventKind::ScrollDown => {
							let mut mouse = world.get_resource_mut::<Events<MouseWheel>>().unwrap();
							mouse.send(MouseWheel {
								unit: MouseScrollUnit::Line,
								x: 0.0,
								y: -1.0,
							})
						}
						MouseEventKind::ScrollUp => {
							let mut mouse = world.get_resource_mut::<Events<MouseWheel>>().unwrap();
							mouse.send(MouseWheel {
								unit: MouseScrollUnit::Line,
								x: 0.0,
								y: 1.0,
							})
						}
					}
				}
				Event::Resize(x, y) => resized = Some((x, y)),
			}
		} else {
			break;
		}
	}

	if let Some((x, y)) = resized {
		let window = windows
			.get_primary_mut()
			.expect("primary window was somehow missing");
		window.update_actual_size_from_backend(x as u32, y as u32);
		info!("tui resized to: {}:{}", x, y);
	}
}

fn change_window(world: &mut World) {
	let world = world.cell();
	let mut windows = world.get_resource_mut::<Windows>().unwrap();
	let mut tui = world.get_non_send_mut::<TUI>().unwrap();

	for bevy_window in windows.iter_mut() {
		let id = bevy_window.id();
		assert_eq!(
			id,
			WindowId::primary(),
			"only a single `primary` window is allowed in the TUI"
		);
		for command in bevy_window.drain_commands() {
			match command {
				WindowCommand::SetTitle { title } => {
					if let Err(e) = tui.try_set_title(&title) {
						error!("failed to set TUI title: {:?}", e)
					}
				}
				WindowCommand::SetCursorLockMode { .. } => {
					todo!("set cursor lock mode")
				}
				WindowCommand::SetCursorVisibility { visible } => {
					if let Err(e) = if visible {
						tui.term_mut().show_cursor()
					} else {
						tui.term_mut().hide_cursor()
					} {
						error!("failed to set cursor visibility for TUI: {:?}", e)
					}
				}
				WindowCommand::SetCursorPosition { position } => {
					if let Err(e) = tui
						.term_mut()
						.set_cursor(position.x as u16, position.y as u16)
					{
						error!("failed to set cursor position for TUI: {:?}", e)
					}
				}
				unsupported_cmd => {
					error!(
						"unsupported WindowCommand on Window `{}`: {:?}",
						id, unsupported_cmd
					)
				}
			}
		}
	}
}

fn reset_on_exit(mut exiting: EventReader<AppExit>, mut tui: NonSendMut<TUI>) {
	if exiting.iter().next().is_some() {
		// Ignore the results of these in case `stdout` is already dead
		let _ignore_show_mouse = tui.term_mut().show_cursor();
		let _ignore_disable_mouse_capture =
			execute!(std::io::stdout(), crossterm::event::DisableMouseCapture);
		let _ignore_alternate_screen =
			execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen);
		let _ignore_erase_title = execute!(std::io::stdout(), crossterm::terminal::SetTitle(""));
		let _ignore_disable_raw_mode = crossterm::terminal::disable_raw_mode();
	}
}
