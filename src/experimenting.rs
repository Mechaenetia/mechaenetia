//use mechaenetia::state::GameState;

use bevy::prelude::*;

pub fn start() -> anyhow::Result<()> {
	App::build()
		// DefaultPlugins sets up the standard renderer, winit, logger, some basic components, etc... depending on features enabled
		.add_plugins(DefaultPlugins)
		.run();
	Ok(())
}
