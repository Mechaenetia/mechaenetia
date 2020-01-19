use amethyst::{
	assets::{PrefabLoader, PrefabLoaderSystem, RonFormat},
	core::transform::TransformBundle,
	prelude::*,
	renderer::{
		plugins::{RenderShaded3D, RenderToWindow},
		rendy::mesh::{Normal, Position, TexCoord, MeshBuilder},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::{application_root_dir, scene::BasicScenePrefab},
	winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};

// This part prevents us from updating Amethyst.
type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

// Definitely not gonna be the final Chunk System. But imagine the Game was that small. XD
// Each of the Bits represents one of the Vertices being active/inactive for Marching Cubes Render.
struct Chunk {data: [[[u8; 4]; 4]; 4]}
impl Chunk {
	// in this example I only have Full Blocks and Empty Blocks, no Slopes or such.
	// the Result of this Array should be reminiscent of an Egg.
	// I can only guess which way the Egg is gonna be rotated, but this is just some hardcoded Test.
	fn new() -> Self {
		Chunk { data : [
		[[  0,  0,  0,  0],[  0,255,255,  0],[255,255,255,255],[  0,255,255,  0]],
		[[  0,255,255,  0],[255,255,255,255],[255,255,255,255],[255,255,255,255]],
		[[  0,255,255,  0],[255,255,255,255],[255,255,255,255],[255,255,255,255]],
		[[  0,  0,  0,  0],[  0,255,255,  0],[255,255,255,255],[  0,255,255,  0]]
		]}
	}
}

struct GameState;
impl SimpleState for GameState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		println!("Starting Game!");
		
		let chunk = Chunk::new();
		
		let mut builder = MeshBuilder::new();
		
		for x in 0..chunk.data.len() {
			for y in 0..chunk.data[x].len() {
				for z in 0..chunk.data[x][y].len() {
					let value = chunk.data[x][y][z];
					println!("{}; {}; {}; {}", x, y, z, value);
				}
			}
		}
		
		
//		let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
//			loader.load("prefab/sphere.ron", RonFormat, ())
//		});
//		data.world.create_entity().with(handle).build();
	}
	
	
	fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		Trans::None
	}
	
	
	fn handle_event(
		&mut self,
		_: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::KeyboardInput {
						input:
						KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::Escape),
							..
						},
						..
					}
					| WindowEvent::CloseRequested => Trans::Quit,
					_ => Trans::None,
				},
				_ => Trans::None,
			}
		} else {
			Trans::None
		}
	}
}

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
