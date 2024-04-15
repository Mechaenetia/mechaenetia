use bevy::asset::AssetPath;
use bevy::prelude::*;
use bevy::winit::WinitWindows;

pub struct WindowIconPlugin(pub AssetPath<'static>);

impl Plugin for WindowIconPlugin {
	fn build(&self, _app: &mut App) {}

	fn cleanup(&self, app: &mut App) {
		let Some(asset_server) = app.world.get_resource::<AssetServer>() else {
			warn!("Failed to get asset server, not handling window icon");
			return;
		};
		let icon: Handle<Image> = asset_server.load(self.0.clone());
		app.insert_resource(ProgramIcon(icon))
			.add_systems(Update, set_primary_window_icon);
	}
}

#[derive(Resource)]
pub struct ProgramIcon(Handle<Image>);

fn set_primary_window_icon(
	mut image_asset_events: EventReader<AssetEvent<Image>>,
	windows: NonSend<WinitWindows>,
	primary_id: Query<Entity, With<bevy::window::PrimaryWindow>>,
	program_icon: Res<ProgramIcon>,
	image_assets: Res<Assets<Image>>,
) {
	let primary_id = primary_id.single();
	let Some(primary) = windows.get_window(primary_id) else {
		warn!("Failed to get primary window");
		return;
	};

	if !image_asset_events.read().any(|ev| match ev {
		AssetEvent::Added { id } if id == &program_icon.0.id() => true,
		AssetEvent::LoadedWithDependencies { id } if id == &program_icon.0.id() => true,
		AssetEvent::Modified { id } if id == &program_icon.0.id() => true,
		AssetEvent::Removed { id } if id == &program_icon.0.id() => false,
		_ => false,
	}) {
		return;
	}

	let Some(icon) = image_assets.get(&program_icon.0) else {
		warn!("Failed to get program icon");
		return;
	};

	let icon_size = icon.texture_descriptor.size;

	let icon = match winit::window::Icon::from_rgba(icon.data.clone(), icon_size.width, icon_size.height) {
		Ok(icon) => icon,
		Err(err) => {
			warn!("Failed to create window icon: {}", err);
			return;
		}
	};

	primary.set_window_icon(Some(icon));
}
