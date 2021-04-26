use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Default)]
pub(super) struct ExitPlugin;

impl Plugin for ExitPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<RequestExit>()
			.add_system_to_stage(CoreStage::Last, perform_exit.system())
			.add_system_to_stage(CoreStage::First, exit_requested.exclusive_system());
	}
}

/// If you wish to not shut down for another update cycle then set `delay` to true.  Can access this
/// via something like `mut exiting: Option<ResMut<Exiting>>` or so.  If `delay` is false then the
/// program will AppExit at the current CoreStage::Last.
///
/// If `force_exit_delay` is set (1 second by default) then once AppExit is sent then a new thread
/// is spawned that will wait that period of time and then force exit, for example when GPU drivers
/// don't let go properly then the app will still exit with code 0.
///
/// Don't delete this resource, after it's already been added then things are already running their
/// shutdown functions.
pub struct Exiting {
	delay: bool,
	force_exit_delay: Option<std::time::Duration>,
}

impl Exiting {
	pub fn delay(&mut self) {
		self.delay = true;
	}
}

fn perform_exit(mut exit: EventWriter<AppExit>, mut exiting: Option<ResMut<Exiting>>) {
	if let Some(exiting) = exiting.as_mut() {
		if !exiting.delay {
			info!("Exiting");
			exit.send(AppExit);
			if let Some(force_exit_delay) = exiting.force_exit_delay {
				std::thread::spawn(move || {
					// Hard exit after a second in case the GPU driver doesn't let go
					std::thread::sleep(force_exit_delay);
					trace!("Not shut down within 1 second, killing...");
					std::process::exit(0);
				});
			}
		}
		exiting.delay = false;
	}
}

/// Send this event to tell everything to shut down, app will exit at the end of the next frame
/// unless `Exiting` is modified to delay it.
///
/// Listen for the presence of `Exiting` as a resource to know when to run shutdown functionality.
#[derive(Default)]
pub struct RequestExit;

fn exit_requested(mut commands: Commands, mut requested: EventReader<RequestExit>) {
	if let Some(_) = requested.iter().next() {
		trace!("Requested Exit");
		commands.insert_resource(Exiting {
			delay: false,
			force_exit_delay: Some(std::time::Duration::from_millis(1000)),
		});
	}
}
