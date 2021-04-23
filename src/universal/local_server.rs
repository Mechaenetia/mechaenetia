use bevy::ecs::component::Component;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Default)]
pub(super) struct LocalServerPlugin;

impl Plugin for LocalServerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<CreateServer>();
	}
}

/// An even to request the LocalServer to create a new server
pub struct CreateServer {
	title: String,
}

impl CreateServer {
	pub fn new(title: String) -> Self {
		Self { title }
	}
}

/// An empty resource that is inserted when the local server is compiled in, and doesn't when its not.
pub struct LocalServerExists;

/// A convenient holder to get both a `LocalServerExists` and a an event writer at the same time and
/// only get the event if the server exists
#[derive(SystemParam)]
pub struct LocalServerEvent<'a, Event: Component> {
	exists: Option<Res<'a, LocalServerExists>>,
	event: EventWriter<'a, Event>,
}

impl<'a, Event: Component> LocalServerEvent<'a, Event> {
	pub fn exists(&self) -> bool {
		self.exists.is_some()
	}

	pub fn event(&mut self) -> Option<&mut EventWriter<'a, Event>> {
		if self.exists() {
			Some(&mut self.event)
		} else {
			None
		}
	}
}
