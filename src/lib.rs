// #[cfg(feature = "client_tui")]
// mod client_tui;
// #[cfg(feature = "client_wgpu")]
// pub mod client_wgpu;
// pub mod core;
// #[cfg(feature = "server")]
// pub mod server;
// pub mod universal;
// 
// pub mod prelude {
// 	pub use crate::core::Engine;
// }

pub enum EngineState {
	Menu,
	Game,
	Exit,
}
