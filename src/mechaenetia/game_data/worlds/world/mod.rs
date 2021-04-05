pub mod world_generator;

use crate::game_data::chunks::chunk::{Chunk, ChunkCoord};
use crate::game_data::chunks::Chunks;
use crate::game_data::worlds::WorldId;
use smol_str::SmolStr;
use world_generator::WorldGenerator;

pub struct World {
	idx: WorldId,
	name: SmolStr,
	generator: Box<dyn WorldGenerator>,
	chunks: Chunks,
}

impl World {
	pub(super) fn new(idx: WorldId, name: SmolStr, generator: Box<dyn WorldGenerator>) -> Self {
		let chunk_edge_len = generator.get_chunk_edge_len();
		Self {
			idx,
			name,
			generator,
			chunks: Chunks::new(chunk_edge_len),
		}
	}

	pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
		self.chunks.get_chunk(coord)
	}

	pub fn get_chunk_mut(&mut self, coord: &ChunkCoord) -> Option<&mut Chunk> {
		self.chunks.get_chunk_mut(coord)
	}

	pub fn get_or_generate_chunk_now(&mut self, coord: ChunkCoord) -> &mut Chunk {
		let (chunk, inserted) = self.chunks.get_or_create_chunk(coord);
		if inserted {
			// TODO:  self.generator.generate_tiles_in_chunk(chunk);
		}
		chunk
	}
}
