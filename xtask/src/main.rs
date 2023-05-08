use xtask::not_bash::run;

fn main() {
	let mut args = std::env::args();
	let _xtask_name = args.next().expect("first argument should always be the program name");
	let Some(cmd) = args.next() else {
        exit_with_help()
    };
	//std::env::vars().for_each(|(k, v)| println!("env: {k} -> {v}"));
	match cmd.as_str() {
		"--help" | "-h" => exit_with_help(),
		"checks" => cmd_checks(args),
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

fn cmd_checks(_args: std::env::Args) {
	println!("Running checks...");
	cmd_checks_licenses();
}

fn cmd_checks_licenses() {
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
		"GPL-3.0-or-later",
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

	let meta = run!("cargo metadata --format-version 1"; echo = false).unwrap();
	let mut licenses = meta
		.split(|c| c == ',' || c == '{' || c == '}')
		.filter(|it| it.contains(r#""license""#))
		.map(|it| it.trim())
		.map(|it| it[r#""license":"#.len()..].trim_matches('"'))
		.collect::<Vec<_>>();
	licenses.sort();
	licenses.dedup();
	assert_eq!(licenses, expected);

	println!("Passed");
}
