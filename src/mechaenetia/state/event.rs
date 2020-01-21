use amethyst::{
	prelude::*,
	winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};

impl Events for GameState {
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
