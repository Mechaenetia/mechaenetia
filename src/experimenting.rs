//use mechaenetia::state::GameState;

use bevy::app::AppExit;
use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::renderer::TextureId;
use bevy::render::wireframe::WireframePlugin;
use bevy::utils::HashMap;
use mechaenetia::game_data::chunks::chunk::{Chunk, ChunkCoord};
use mechaenetia::game_data::worlds::world::world_generator::LayeredWorldGenerator;
use mechaenetia::game_data::worlds::Worlds;
use std::sync::{Arc, RwLock};

// Switch to #[bevy_main] if this is what's chosen as the main target
pub fn start() -> anyhow::Result<()> {
	// GameState::Startup is for system startup, not for loading textures or so, unless you want it all to happen before rendering, like progress bars
	// GameState::Update is for the standard update loop, there's also a Pre and  Post version
	let mut app_builder = App::build();

	app_builder
		// DefaultPlugins sets up the standard renderer, winit, logger, some basic components, etc... depending on features enabled
		.add_plugins_with(
			DefaultPlugins,
			|group| group.disable::<LogPlugin>(), // We have a more configurable logger, log4rs, so don't use EnvFilter
		)
		.add_startup_system(testing_blah.system())
		.add_system(testing_query.system())
		.add_state(GameState::Loading)
		.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_loading.system()))
		.add_system_set(
			SystemSet::on_update(GameState::Loading).with_system(update_loading.system()),
		)
		.add_system_set(SystemSet::on_pause(GameState::Loading).with_system(pause_loading.system()))
		.add_system_set(
			SystemSet::on_resume(GameState::Loading).with_system(resume_loading.system()),
		)
		.add_system_set(
			SystemSet::on_enter(GameState::MainMenu).with_system(load_mainmenu.system()),
		)
		.add_system_set(
			SystemSet::on_update(GameState::MainMenu).with_system(update_mainmenu.system()),
		)
		.add_system_set(
			SystemSet::on_exit(GameState::MainMenu).with_system(unload_mainmenu.system()),
		)
		.init_non_send_resource::<Worlds>()
		.add_startup_system(create_test_world.system());

	if false {
		app_builder.insert_resource(ReportExecutionOrderAmbiguities);
	}

	if true {
		app_builder.add_plugin(WireframePlugin::default());
	}

	app_builder.run();

	Ok(())
}

fn testing_blah(mut cmds: Commands) {
	cmds.spawn().insert(42usize);
	cmds.spawn().insert(21usize).insert(true);
	cmds.spawn().insert(18usize).insert(false).insert(());
	warn!("Made entities");
}

fn testing_query(
	mut query: Query<(&usize, Option<&bool>), (With<()>,)>,
	mut exit: EventWriter<AppExit>,
) {
	let mut do_exit = false;
	for (the_int, maybe_bool) in query.iter() {
		do_exit = true;
		warn!("Found things: {}, {:?}", the_int, maybe_bool);
	}
	if do_exit {
		exit.send(AppExit);
	}
}

// #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
// struct GameStage;
//
//#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
	Loading,
	MainMenu,
	InGame,
	PauseMenu,
}

fn load_loading(mut state: ResMut<State<GameState>>) {
	info!("load_loading");
	state.push(GameState::MainMenu).unwrap();
}

fn update_loading() {
	info!("update_loading");
}

fn pause_loading() {
	info!("pause_loading");
}

fn resume_loading(mut exit: EventWriter<AppExit>) {
	info!("resume_loading");
	exit.send(AppExit);
}

fn load_mainmenu() {
	info!("load_mainmenu");
}

fn update_mainmenu(mut state: ResMut<State<GameState>>) {
	info!("update_mainmenu");
	state.pop().unwrap();
}

fn unload_mainmenu() {
	info!("unload_mainmenu");
}

fn create_test_world(mut commands: Commands, mut worlds: NonSendMut<Worlds>) {
	let test_world_id =
		worlds.create_world("test".into(), Box::new(LayeredWorldGenerator::new(vec![])));
	let mut test_world = worlds.get_world_mut(test_world_id);
	let chunk = test_world.get_or_generate_chunk_now(ChunkCoord::new(0, 0, 0));

	commands
		.spawn()
		.insert_bundle((ChunkRenderable::new(chunk),));
}

struct ChunkRenderable {
	meshes: HashMap<TextureId, Mesh>,
}

impl ChunkRenderable {
	fn new(chunk: &Chunk) -> Self {
		let meshes = HashMap::default();
		// TODO:  When textures exist
		Self { meshes }
	}
}
