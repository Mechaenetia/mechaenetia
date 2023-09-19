#![warn(clippy::pedantic)]

use semver::{Version, VersionReq};
use std::collections::HashMap;
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
	cmd_checks_licenses();
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

fn cmd_checks_licenses() {
	#[derive(serde::Deserialize)]
	struct Meta<'s> {
		#[serde(borrow)]
		packages: Vec<MetaPackage<'s>>,
	}
	#[derive(serde::Deserialize)]
	struct MetaPackage<'s> {
		#[serde(borrow)]
		name: &'s str,
		license: Option<&'s str>,
	}
	print!("> Licenses...");
	let expected = [
		"(MIT OR Apache-2.0) AND Unicode-DFS-2016",
		"0BSD OR MIT OR Apache-2.0",
		"Apache-2.0",
		"Apache-2.0 / MIT",
		"Apache-2.0 OR BSL-1.0",
		"Apache-2.0 OR MIT",
		"Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
		"Apache-2.0/MIT",
		"BSD-2-Clause",
		"BSD-2-Clause OR MIT OR Apache-2.0",
		"BSD-3-Clause",
		"BSD-3-Clause OR MIT OR Apache-2.0",
		"CC0-1.0",
		"CC0-1.0 OR Artistic-2.0",
		// Do NOT add GPL to the general list, it's only allowed for this project itself
		// "GPL-3.0-or-later",
		"ISC",
		"MIT",
		"MIT / Apache-2.0",
		"MIT OR Apache-2.0",
		"MIT OR Apache-2.0 OR Zlib",
		"MIT OR Zlib OR Apache-2.0",
		"MIT-0",
		"MIT/Apache-2.0",
		"Unlicense OR MIT",
		"Unlicense/MIT",
		"Zlib",
		"Zlib AND (MIT OR Apache-2.0)",
		"Zlib OR Apache-2.0 OR MIT",
	];
	// Only GPL we want to accept is within this project itself
	let special_cased = [
		("xtask", "GPL-3.0-or-later"),
		("mechaenetia_client", "GPL-3.0-or-later"),
		("mechaenetia_client_wgpu", "GPL-3.0-or-later"),
		("mechaenetia_engine", "GPL-3.0-or-later"),
		("mechaenetia_server", "GPL-3.0-or-later"),
		("mechaenetia_server_dedicated", "GPL-3.0-or-later"),
		("mechaenetia_utils", "GPL-3.0-or-later"),
	]
	.into_iter()
	.collect::<HashMap<&'static str, &'static str>>();

	let meta = run!("cargo metadata --format-version 1"; echo = false).unwrap();
	let meta: Meta = serde_json::from_str(&meta).unwrap();

	let mut errors = String::new();
	for MetaPackage { name, license } in meta.packages {
		if let Some(license) = license {
			let license = license.trim();
			if let Some(sc) = special_cased.get(name) {
				if *sc == license {
					continue;
				}
				errors.push_str(&format!("Special cased license for {name}: {license} != {sc}\n"));
			}
			if !expected.contains(&license) {
				errors.push_str(&format!("Unknown license for {name}: {license}\n"));
			}
		} else {
			errors.push_str(&format!("No license for {name}\n"));
		}
	}

	assert!(errors.is_empty(), "Errors:\n{errors}");
	println!("Passed");
}
