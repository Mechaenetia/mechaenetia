use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub(super) struct LocalServerPlugin;

impl Plugin for LocalServerPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<LocalServerCommand>()
			.add_event::<LocalServerPublicState>();
	}
}

/// Event to send control commands to the LocalServer
#[derive(Debug)]
pub enum LocalServerCommand {
	CreateStartServer {
		path: PathBuf,
		config_only_if_not_existing: bool,
	},
	StopServer {
		force: bool,
	},
}

/// A resource that is inserted when the local server is compiled in, and doesn't when its not.
///
/// This should also always be sent as an event when it changes.
///
/// Do not add this resource in yourself unless you've implemented a local server for the client to
/// communicate with.
#[derive(Clone, PartialEq)]
pub enum LocalServerPublicState {
	/// A LocalServer is not running
	Off,
	/// A LocalServer is loading, the float is from 0.0 to 1.0 for percentage completion before the
	/// server can be joined
	Loading(f64),
	/// A LocalServer is running and ready for connection
	Running,
	/// A LocalServer is shutting down
	ShuttingDown,
}
