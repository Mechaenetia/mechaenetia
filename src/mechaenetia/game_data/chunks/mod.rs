pub mod chunk;

use crate::game_data::chunks::chunk::ChunkCoord;
use chunk::Chunk;
use std::collections::HashMap;
use std::num::NonZeroU8;

pub struct Chunks {
	chunk_edge_len: NonZeroU8,
	chunks: HashMap<ChunkCoord, Chunk>,
}

impl Chunks {
	pub fn new(chunk_edge_len: NonZeroU8) -> Self {
		Self {
			chunk_edge_len,
			chunks: HashMap::default(),
		}
	}

	pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
		self.chunks.get(coord)
	}

	pub fn get_chunk_mut(&mut self, coord: &ChunkCoord) -> Option<&mut Chunk> {
		self.chunks.get_mut(coord)
	}

	pub fn get_or_create_chunk(&mut self, coord: ChunkCoord) -> (&mut Chunk, bool) {
		let inserted = !self.chunks.contains_key(&coord);
		let chunk_edge_len = self.chunk_edge_len;
		let chunk = self
			.chunks
			.entry(coord)
			.or_insert_with_key(|k| Chunk::new(k.clone(), chunk_edge_len));
		(chunk, inserted)
	}
}

// // Definitely not gonna be the final Chunk System. But imagine the Game was that small. XD
// // Each of the Bits represents one of the Vertices being active/inactive for Marching Cubes Render.
// pub struct Chunk {
// 	pub data: [[[u8; 4]; 4]; 4],
// }
//
// impl Chunk {
// 	// in this example I only have Full Blocks and Empty Blocks, no Slopes or such.
// 	// the Result of this Array should be reminiscent of an Egg.
// 	// I can only guess which way the Egg is gonna be rotated, but this is just some hardcoded Test.
// 	pub fn new() -> Self {
// 		Chunk {
// 			data: [
// 				[
// 					[0, 0, 0, 0],
// 					[0, 255, 255, 0],
// 					[255, 255, 255, 255],
// 					[0, 255, 255, 0],
// 				],
// 				[
// 					[0, 255, 255, 0],
// 					[255, 255, 255, 255],
// 					[255, 255, 255, 255],
// 					[255, 255, 255, 255],
// 				],
// 				[
// 					[0, 255, 255, 0],
// 					[255, 255, 255, 255],
// 					[255, 255, 255, 255],
// 					[255, 255, 255, 255],
// 				],
// 				[
// 					[0, 0, 0, 0],
// 					[0, 255, 255, 0],
// 					[255, 255, 255, 255],
// 					[0, 255, 255, 0],
// 				],
// 			],
// 		}
// 	}
// }
