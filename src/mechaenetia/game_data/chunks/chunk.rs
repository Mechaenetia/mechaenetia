use std::fmt;
use std::marker::PhantomData;
use std::num::NonZeroU8;
use std::ops::Sub;

pub type ChunkCoordType = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord([ChunkCoordType; 3]);

pub struct ChunkCoordAABB {
	pub bottom: ChunkCoord,
	pub top: ChunkCoord,
}

impl ChunkCoord {
	pub fn new(x: ChunkCoordType, y: ChunkCoordType, z: ChunkCoordType) -> Self {
		ChunkCoord([x, y, z])
	}

	pub fn x(&self) -> ChunkCoordType {
		self.0[0]
	}

	pub fn y(&self) -> ChunkCoordType {
		self.0[1]
	}

	pub fn z(&self) -> ChunkCoordType {
		self.0[2]
	}

	pub fn area(&self) -> ChunkCoordArea {
		let x = self.x() as u16;
		let y = self.y() as u16;
		let z = self.z() as u16;
		ChunkCoordArea([x, y, z])
	}
}

pub type ChunkCoordAreaType = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkCoordArea([ChunkCoordAreaType; 3]);

impl ChunkCoordArea {
	pub fn width(&self) -> ChunkCoordAreaType {
		self.0[0]
	}

	pub fn length(&self) -> ChunkCoordAreaType {
		self.0[1]
	}

	pub fn height(&self) -> ChunkCoordAreaType {
		self.0[2]
	}

	pub fn area(&self) -> usize {
		self.0[0] as usize * self.0[1] as usize * self.0[2] as usize
	}
}

// Cubic Chunks
pub struct Chunk {
	chunk_edge_len: NonZeroU8,
	location: ChunkCoord,
	data: Vec<()>,
}

impl fmt::Debug for Chunk {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Chunk")
			.field("location", &self.location)
			.field("chunk_edge_len", &self.chunk_edge_len)
			.field("data", &"<snip>")
			.finish()
	}
}

impl Chunk {
	pub fn new(location: ChunkCoord, chunk_edge_len: NonZeroU8) -> Self {
		let len = chunk_edge_len.get() as usize;
		let area = len * len * len;
		Chunk {
			location,
			chunk_edge_len,
			data: vec![(); area],
		}
	}

	pub fn iter_tiles_mut(&mut self) -> IterChunkTilesMut {
		IterChunkTilesMut {
			_phantom: PhantomData::default(),
		}
	}
}

pub struct IterChunkTilesMut<'a> {
	_phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for IterChunkTilesMut<'a> {
	type Item = ((), &'a ());

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

impl Sub for ChunkCoord {
	type Output = ChunkCoordArea;

	fn sub(self, rhs: Self) -> Self::Output {
		let x = (self.x() as isize - rhs.x() as isize).abs() as ChunkCoordAreaType;
		let y = (self.y() as isize - rhs.y() as isize).abs() as ChunkCoordAreaType;
		let z = (self.z() as isize - rhs.z() as isize).abs() as ChunkCoordAreaType;
		ChunkCoordArea([x, y, z])
	}
}
