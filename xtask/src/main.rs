#![warn(clippy::pedantic)]

use std::collections::HashMap;
use xtask::not_bash::run;

fn main() {
	let mut args = std::env::args();
	let _xtask_name = args.next().expect("first argument should always be the program name");
	let Some(cmd) = args.next() else { exit_with_help() };
	//std::env::vars().for_each(|(k, v)| println!("env: {k} -> {v}"));
	match cmd.as_str() {
		"--help" | "-h" => exit_with_help(),
		"checks" => cmd_checks(&mut args),
		cmd => {
			println!("Unknown command: {cmd}\n");
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
	println!("Running checks...");
	cmd_checks_licenses();
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
