use anyhow;
use log::*;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod init;
pub mod experimenting;

#[derive(StructOpt, Debug)]
#[structopt(
	name = "Mechaenetia Client",
	about = "Client/Server implementation of Mechaenetia"
)]
pub struct CLIOpts {
	/// Logging verbosity, add more to be more verbose
	#[structopt(short, long, parse(from_occurrences))]
	verbose: u8,
	
	/// Path to a directory to store configuration files
	#[structopt(short, long, parse(from_os_str))]
	config_dir: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let opts = CLIOpts::from_args();
	init::init_logging(&opts.config_dir)?;
	info!("Hello, world!");
	debug!("Opts:  {:?}", &opts);
	
	experimenting::amethyststuff().unwrap();
	
	Ok(())
}
