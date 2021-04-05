/*
use super::chunks::Chunk;
use amethyst::{
	prelude::*,
	winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};

pub struct GameState;
impl SimpleState for GameState {
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


	fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {


		Trans::None
	}


	fn handle_event(
		&mut self,
		_: StateData<'_, GameData<'_, '_>>,
		event: StateEvent,
	) -> SimpleTrans {
		if let StateEvent::Window(event) = &event {
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::KeyboardInput {
						input:
						KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::Escape),
							..
						},
						..
					}
					| WindowEvent::CloseRequested => Trans::Quit,
					_ => Trans::None,
				},
				_ => Trans::None,
			}
		} else {
			Trans::None
		}
	}
}
*/
