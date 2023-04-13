use crate::sim::chunks::{Chunk, ChunkPos};
// use bevy::prelude::*;
use bevy::utils::StableHashMap;

pub struct Dimension {
	pub name: String,
	pub chunks: StableHashMap<ChunkPos, Chunk>,
}
