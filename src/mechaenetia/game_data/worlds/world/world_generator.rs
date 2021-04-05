use crate::game_data::chunks::chunk::{Chunk, ChunkCoordType};
use std::num::NonZeroU8;

pub trait WorldGenerator: Send + Sync {
	fn get_chunk_edge_len(&self) -> NonZeroU8;

	fn generate_tiles_in_chunk(&mut self, chunk: &mut Chunk) {
		for (coord, tile) in chunk.iter_tiles_mut() {
			// *tile = self.generator.generate_tile_at(coord);
		}
	}

	//fn generate_tile_at(&mut self, coord: TileCoord) -> TileId;
}

pub struct LayeredWorldGenerator {
	layers: Vec<(ChunkCoordType, ())>,
}

impl WorldGenerator for LayeredWorldGenerator {
	fn get_chunk_edge_len(&self) -> NonZeroU8 {
		NonZeroU8::new(32).unwrap()
	}
}

impl LayeredWorldGenerator {
	pub fn new(layers: Vec<(ChunkCoordType, ())>) -> Self {
		Self { layers }
	}
}
