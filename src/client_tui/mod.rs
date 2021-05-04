mod states;
mod tui_plugin;

use crate::client_tui::tui_plugin::TUI;
use crate::universal::conditional_map::ConditionalMap;
use crate::universal::exit::RequestExit;
use bevy::app::{PluginGroupBuilder, ScheduleRunnerSettings};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;
use std::sync::atomic::Ordering;
use std::time::Duration;

#[derive(Default)]
pub struct ClientTuiPluginGroup;

#[derive(Default)]
struct ClientTuiPlugin;

impl PluginGroup for ClientTuiPluginGroup {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		trace!("disabling `console` conditional map for the logger");
		tracing::log::logger().flush();
		ConditionalMap::get_or_create_by_id("console".to_owned(), false)
			.store(false, Ordering::Relaxed);
		group
			.add(bevy::audio::AudioPlugin::default())
			.add(bevy::gilrs::GilrsPlugin::default())
			.add(bevy::app::ScheduleRunnerPlugin::default())
			.add(
				tui_plugin::TuiRunnerPlugin::default()
					.title(env!("CARGO_PKG_NAME"))
					.start_with_mouse_captured(true)
					.start_in_raw_mode(true)
					.enable_alternate_screen(true)
					.max_events_per_tick(128),
			)
			.add(states::ClientStatePlugin::default())
			.add(ClientTuiPlugin)
			.add(bevy::app::ScheduleRunnerPlugin::default());
	}
}

impl Plugin for ClientTuiPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
			1.0 / 20.0,
		)))
		.add_system(exit_on_window_close.system())
		.add_system(exit_on_escape.system())
		.add_system_to_stage(CoreStage::PostUpdate, draw.exclusive_system());
	}
}

fn exit_on_escape(mut keys: EventReader<KeyboardInput>, mut exit: EventWriter<RequestExit>) {
	for key in keys.iter() {
		if key.key_code == Some(KeyCode::Escape) && key.state == ElementState::Released {
			trace!("escape pressed to request exit");
			exit.send(RequestExit);
		}
	}
}

fn exit_on_window_close(
	mut windows_closed: EventReader<WindowCloseRequested>,
	mut exit: EventWriter<RequestExit>,
) {
	// We only support a single window currently, change this if that changes
	if let Some(window_closed) = windows_closed.iter().next() {
		trace!("Window closed `{:?}`: exiting", window_closed.id);
		exit.send(RequestExit);
	}
}

fn draw(world: &mut World) {
	let world = world.cell();
	let mut tui = world.get_resource_mut::<TUI>().unwrap();
	let cur_state = world
		.get_resource::<State<states::ClientState>>()
		.unwrap()
		.current()
		.clone();

	match tui.draw(|f| {
		cur_state.draw(&world, f);
	}) {
		Ok(()) => (),
		Err(e) => error!("failed TUI draw call: {:?}", e),
	}
}
