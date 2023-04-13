pub mod chunks;
pub mod dimension;

use crate::states::SimState;
use bevy::prelude::*;
use std::path::PathBuf;

#[allow(clippy::module_name_repetitions)]
pub struct SimPlugin {
	pub default_save_path: PathBuf,
}

impl Plugin for SimPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(SimStorageLocation::new(self.default_save_path.clone()))
			.add_system(init_sim.in_schedule(OnEnter(SimState::Loaded)))
			.add_system(save_state_if_necessary.in_set(OnUpdate(SimState::Loaded)))
			.add_system(unload_sim.in_schedule(OnExit(SimState::Loaded)));
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Resource)]
pub struct SimStorageLocation {
	pub path: PathBuf,
	pub perform_saving: bool,
	pub last_saved_at: std::time::Instant,
}

impl Default for SimStorageLocation {
	fn default() -> Self {
		SimStorageLocation {
			path: PathBuf::from("save/default/"),
			perform_saving: true,
			last_saved_at: std::time::Instant::now(),
		}
	}
}

impl SimStorageLocation {
	#[must_use]
	pub fn new(path: PathBuf) -> Self {
		Self {
			path,
			..Default::default()
		}
	}
}

fn init_sim(mut sim_storage_location: ResMut<SimStorageLocation>, mut sim_state: ResMut<NextState<SimState>>) {
	if !sim_storage_location.path.is_dir() {
		if std::fs::create_dir_all(&sim_storage_location.path).is_err() {
			error!(
				"Sim Storage Location of `{:?}` could not be created",
				sim_storage_location.path
			);
			sim_state.set(SimState::NotLoaded);
		}
		if !sim_storage_location.path.is_dir() {
			todo!(
				"Sim Storage Location of `{:?}` does not exist, it must be initialized first, do that here",
				sim_storage_location.path
			);
		}
	}
	warn!(
		"Sim Storage Location of `{:?}` is not implemented yet",
		sim_storage_location.path
	);
	sim_storage_location.last_saved_at = std::time::Instant::now();
}

fn save_state_if_necessary(_sim_storage_location: Res<SimStorageLocation>) {
	// todo!("Save the sim?  {sim_storage_location:?}");
}

fn unload_sim(sim_storage_location: Res<SimStorageLocation>) {
	warn!("Unload the sim, should it be saved or not?  {sim_storage_location:?}");
}
