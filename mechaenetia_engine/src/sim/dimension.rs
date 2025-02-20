#![expect(clippy::zero_sized_map_values)]

use crate::sim::chunks::{Chunk, ChunkPos};
use std::collections::HashMap;
// use bevy::prelude::*;
use bevy::utils::FixedState;

pub struct Dimension {
	pub name: String,
	pub chunks: HashMap<ChunkPos, Chunk, FixedState>,
}
