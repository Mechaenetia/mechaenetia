#![warn(clippy::pedantic)]

use semver::{Version, VersionReq};
use std::io::IsTerminal;
use std::sync::OnceLock;
use xtask::not_bash::run;

const CARGODENY_REQUIRED_VERSION: &str = "^0.14.2";

static COLOR: OnceLock<&str> = OnceLock::new();

fn main() {
	COLOR
		.set(std::env::args().fold(
			if std::io::stdout().is_terminal() {
				"always"
			} else {
				"never"
			},
			|acc, arg| {
				if arg == "--color" {
					"always"
				} else if arg == "--no-color" {
					"never"
				} else {
					acc
				}
			},
		))
		.expect("COLOR must be set exactly once");
	let mut args = std::env::args();
	let _xtask_name = args.next().expect("first argument should always be the program name");
	let Some(cmd) = args.next() else { exit_with_help() };
	match cmd.as_str() {
		"--help" | "-h" => exit_with_help(),
		"checks" => cmd_checks(&mut args),
		cmd => {
			eprintln!("Unknown command: {cmd}\n");
			exit_with_help()
		}
	}
}

fn exit_with_help() -> ! {
	eprintln!(
		"\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <SUBCOMMAND>

SUBCOMMANDS:
    checks
"
	);
	std::process::exit(1)
}

fn cmd_checks(_args: &mut std::env::Args) {
	eprintln!("Running checks...");
	cmd_checks_cargo_deny();
}

fn cmd_checks_cargo_deny() {
	match run!("cargo-deny --version"; echo = false) {
		Ok(res) => {
			let Some(("", res)) = res.split_once("cargo-deny ") else {
				eprintln!("### cargo-deny not found, please install it via: `cargo install --locked cargo-deny`");
				std::process::exit(1);
			};
			let cargodeny_version = match Version::parse(res) {
				Ok(v) => v,
				Err(e) => {
					eprintln!("### cargo-deny version of `{res} could not be parsed: {e}");
					eprintln!("### Please reinstall cargo-deny via: `cargo install --locked cargo-deny`");
					std::process::exit(1);
				}
			};
			if !VersionReq::parse(CARGODENY_REQUIRED_VERSION)
				.expect("CARGODENY_REQUIRED_VERSION should always be a valid version requirement")
				.matches(&cargodeny_version)
			{
				eprintln!(
					"### cargo-deny version of `{res} is not compatible with the required version of `{CARGODENY_REQUIRED_VERSION}`",
				);
				eprintln!("### Please reinstall cargo-deny via: `cargo install --locked cargo-deny`");
				std::process::exit(1);
			}
		}
		Err(_e) => {
			eprintln!("### cargo-deny not found, please install it via: `cargo install --locked cargo-deny`");
			std::process::exit(1);
		}
	}
	match run!("cargo-deny --color {} check all", COLOR.get().expect("COLOR must be set exactly once"); echo = true) {
		Ok(_) => (),
		Err(_e) => {
			eprintln!("cargo-deny failed, please correct prior errors");
		}
	}
}
