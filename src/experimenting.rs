use mechaenetia::state::GameState;
use amethyst::{
	assets::{PrefabLoaderSystem},
	core::transform::TransformBundle,
	prelude::*,
	renderer::{
		plugins::{RenderShaded3D, RenderToWindow},
		rendy::mesh::{Normal, Position, TexCoord},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::{application_root_dir, scene::BasicScenePrefab},
};

// This part prevents us from updating Amethyst.
type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

pub fn start() -> amethyst::Result<()> {
	// Always First!
	amethyst::start_logger(amethyst::LoggerConfig::default());
	
	let app_root = application_root_dir()?;
	let display_config_path = app_root.join("config/display.ron");
	let assets_directory = app_root.join("assets/");
	
	let game_data = GameDataBuilder::default()
		.with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
		.with_bundle(TransformBundle::new())?
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)
						.with_clear([0.34, 0.36, 0.52, 1.0]),
				)
				.with_plugin(RenderShaded3D::default()),
		)?;
	
	let mut game = Application::new(assets_directory, GameState, game_data)?;
	game.run();
	
	Ok(())
}
