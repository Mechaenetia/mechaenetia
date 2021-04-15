mod states;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowCloseRequested;

#[derive(Default)]
pub struct ClientPlugin;

impl PluginGroup for ClientPlugin {
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
			.add(ClientPlugin::default())
			.add(states::ClientStatePlugin::default());
	}
}

impl Plugin for ClientPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(startup.system())
			.add_system(exit_on_window_close.system());
	}
}

fn exit_on_window_close(
	mut windows_closed: EventReader<WindowCloseRequested>,
	mut state: ResMut<State<states::ClientState>>,
) {
	// We only support a single window currently, change this if that changes
	if let Some(window_closed) = windows_closed.iter().next() {
		trace!("Window closed `{:?}`: exiting", window_closed.id);
		state.overwrite_replace(states::ClientState::Exiting).expect("failed transitioning to the exit state");
	}
}

fn startup(mut commands: Commands) {
	trace!("Client startup");
	// This spawns the camera that renders the 2D UI over the whole screen
	commands.spawn_bundle(UiCameraBundle::default());
}
