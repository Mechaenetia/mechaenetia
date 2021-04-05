//use mechaenetia::state::GameState;

use bevy::prelude::*;
use bevy::render::renderer::TextureId;
use bevy::utils::HashMap;
use mechaenetia::game_data::chunks::chunk::{Chunk, ChunkCoord, ChunkCoordAABB};
use mechaenetia::game_data::worlds::world::world_generator::LayeredWorldGenerator;
use mechaenetia::game_data::worlds::Worlds;
use std::sync::{Arc, RwLock};

pub fn start() -> anyhow::Result<()> {
	App::build()
		// DefaultPlugins sets up the standard renderer, winit, logger, some basic components, etc... depending on features enabled
		.add_plugins(DefaultPlugins)
		.add_resource(Arc::new(RwLock::new(Worlds::default())))
		.add_startup_system(create_test_world.system())
		.run();
	Ok(())
}

fn create_test_world(commands: &mut Commands, mut worlds: ResMut<Arc<RwLock<Worlds>>>) {
	let mut worlds = worlds.write().expect("lock was poisoned");
	let test_world_id =
		worlds.create_world("test".into(), Box::new(LayeredWorldGenerator::new(vec![])));
	let mut test_world = worlds.get_world_mut(test_world_id);
	let chunk = test_world.get_or_generate_chunk_now(ChunkCoord::new(0, 0, 0));

	commands.spawn((ChunkRenderable::new(chunk),));
}

struct ChunkRenderable {
	meshes: HashMap<TextureId, Mesh>,
}

impl ChunkRenderable {
	fn new(chunk: &Chunk) -> Self {
		let meshes = HashMap::default();
		// TODO:  When textures exist
		Self { meshes }
	}
}
