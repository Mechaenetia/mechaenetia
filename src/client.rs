use anyhow;
use log::*;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod init;

#[derive(StructOpt, Debug)]
#[structopt(
	name = "Mechaenetia Client",
	about = "Client/Server implementation of Mechaenetia"
)]
pub struct CLIOpts {
	#[structopt(short, long, parse(from_occurrences))]
	verbose: u8,

	#[structopt(short, long, parse(from_os_str))]
	config_dir: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let opts = CLIOpts::from_args();
	init::init_logging(&opts.config_dir)?;
	info!("Hello, world!");
	debug!("Opts:  {:?}", &opts);
	Ok(())
}
