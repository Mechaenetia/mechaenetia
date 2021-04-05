pub mod world;

use world::{world_generator::WorldGenerator, World};

use indexmap::map::IndexMap;
use smol_str::SmolStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct WorldId(usize);

pub struct Worlds {
	worlds: IndexMap<SmolStr, World>,
}

impl Default for Worlds {
	fn default() -> Self {
		Worlds {
			worlds: IndexMap::default(),
		}
	}
}

impl Worlds {
	pub fn create_world(&mut self, name: SmolStr, generator: Box<dyn WorldGenerator>) -> WorldId {
		let world = World::new(WorldId(self.worlds.len()), name.clone(), generator);
		let (index, inserted) = self.worlds.insert_full(name, world);
		assert!(inserted.is_none(), "world already inserted somehow");
		WorldId(index)
	}

	pub fn get_world(&self, world_id: WorldId) -> &World {
		self.worlds
			.get_index(world_id.0)
			.expect("invalid WorldID for this Worlds")
			.1
	}

	pub fn get_world_mut(&mut self, world_id: WorldId) -> &mut World {
		self.worlds
			.get_index_mut(world_id.0)
			.expect("invalid WorldID for this Worlds")
			.1
	}

	pub fn get_world_name(&self, world_id: WorldId) -> &SmolStr {
		self.worlds
			.get_index(world_id.0)
			.expect("invalid WorldID for this Worlds")
			.0
	}

	pub fn get_world_id_by_name(&self, name: &SmolStr) -> Option<WorldId> {
		self.worlds.get_full(name).map(|t| WorldId(t.0))
	}
}
