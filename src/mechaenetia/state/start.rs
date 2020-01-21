use super::chunk::Chunk;
use amethyst::{
	prelude::*,
};

impl Start for GameState {
	fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		println!("Starting Game!");
		
		let test_chunk = Chunk::new();
		
		for x in 0..test_chunk.data.len() {
			for y in 0..test_chunk.data[x].len() {
				for z in 0..test_chunk.data[x][y].len() {
					let value = test_chunk.data[x][y][z];
					println!("{}; {}; {}; {}", x, y, z, value);
				}
			}
		}
	}
}
