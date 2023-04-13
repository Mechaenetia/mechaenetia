//! This is just a scaffolding, nothing done or complete and probably all is wrong, it's just
//! something to compile and that's all.

use bevy::prelude::*;

// A chunk is a cube of tiles, note the "cube" part.
pub const CHUNK_SIZE_LOG2: usize = 4;
pub const CHUNK_SIZE: usize = 1 << CHUNK_SIZE_LOG2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

pub struct Chunk {
	// tiles: Tiles,
}

// enum Tiles {
// 	Single(TileId),
// 	Many(Box<[[[TileId; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>),
// }

pub struct TileId(u32);

pub struct Tile {
	pub color: Color,
}
