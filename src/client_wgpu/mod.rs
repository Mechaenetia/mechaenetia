mod states;

use crate::universal::exit::RequestExit;
use crate::universal::i18n::I18NLanguageChangedEvent;
use crate::universal::I18N;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;

#[derive(Default)]
pub struct ClientWgpuPluginGroup;

struct ClientWgpuPlugin;

impl PluginGroup for ClientWgpuPluginGroup {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
		group
			.add(bevy::render::RenderPlugin::default())
			.add(bevy::sprite::SpritePlugin::default())
			.add(bevy::pbr::PbrPlugin::default())
			.add(bevy::ui::UiPlugin::default())
			.add(bevy::text::TextPlugin::default())
			.add(bevy::audio::AudioPlugin::default())
			.add(bevy::gilrs::GilrsPlugin::default())
			.add(bevy::gltf::GltfPlugin::default())
			.add(bevy::winit::WinitPlugin::default())
			.add(bevy::wgpu::WgpuPlugin::default())
			.add(ClientWgpuPlugin)
			.add(states::ClientStatePlugin::default());
	}
}

impl Plugin for ClientWgpuPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.insert_resource(ClearColor(Color::rgb(0.0, 0.25, 0.0)))
			.add_startup_system(startup.system())
			.add_system(update_window_title_from_language.system())
			.add_system(exit_on_window_close.system());
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

fn startup(mut windows: ResMut<Windows>) {
	let title = env!("CARGO_PKG_NAME");
	trace!("client_wgpu startup, setting title: {}", title);
	windows.iter_mut().for_each(|window| {
		window.set_title(title.to_owned());
	});

	// This spawns the camera that renders the 2D Bevy UI over the whole screen, not using bevy's UI
	// currently, so its disabled for now...
	// commands.spawn_bundle(UiCameraBundle::default());
}

fn update_window_title_from_language(
	mut windows: ResMut<Windows>,
	lang: Res<I18N>,
	mut event: EventReader<I18NLanguageChangedEvent>,
) {
	if event.iter().next().is_some() {
		let l_title = lang.get("title");
		trace!("client_wgpu title set on language change: {}", &l_title);
		windows.iter_mut().for_each(|window| {
			window.set_title(l_title.to_string());
		});
	}
}
