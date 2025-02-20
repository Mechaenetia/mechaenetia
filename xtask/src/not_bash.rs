// From:  https://github.com/rust-lang/rust-analyzer/blob/3ffa915cbcf4d7a3988142cd94da0463acc87c8a/xtask/src/not_bash.rs

//! A bad shell -- small cross platform module for writing glue code

use std::{
	cell::RefCell,
	env,
	ffi::OsString,
	io::{self, Write},
	path::{Path, PathBuf},
	process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};

pub use fs_err as fs2;

#[macro_export]
macro_rules! run {
    ($($expr:expr),*) => {
        run!($($expr),*; echo = true)
    };
    ($($expr:expr),* ; echo = $echo:expr) => {
        $crate::not_bash::run_process(&format!($($expr),*), $echo, None)
    };
    ($($expr:expr),* ;  <$stdin:expr) => {
        $crate::not_bash::run_process(&format!($($expr),*), false, Some($stdin))
    };
}
pub use crate::run;

pub struct Pushd {
	_p: (),
}

pub fn pushd(path: impl Into<PathBuf>) -> Pushd {
	Env::with(|env| env.pushd(path.into()));
	Pushd { _p: () }
}

impl Drop for Pushd {
	fn drop(&mut self) {
		Env::with(Env::popd);
	}
}

pub struct PushEnv {
	_p: (),
}

#[must_use]
pub fn push_env(var: &str, value: &str) -> PushEnv {
	Env::with(|env| env.push_env(var.into(), value.into()));
	PushEnv { _p: () }
}

impl Drop for PushEnv {
	fn drop(&mut self) {
		Env::with(Env::pop_env);
	}
}

/// Remove a file or directory and all its contents recursively.
///
/// # Errors
///
/// Same as [`fs2::remove_file`] and [`fs2::remove_dir_all`].
pub fn rm_rf(path: impl AsRef<Path>) -> io::Result<()> {
	let path = path.as_ref();
	if !path.exists() {
		return Ok(());
	}
	if path.is_file() {
		fs2::remove_file(path)
	} else {
		fs2::remove_dir_all(path)
	}
}

#[doc(hidden)]
pub fn run_process(cmd: &str, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
	run_process_inner(cmd, echo, stdin).with_context(|| format!("process `{cmd}` failed"))
}

#[must_use]
pub fn date_iso() -> String {
	time::OffsetDateTime::now_utc().date().to_string()
}

fn run_process_inner(cmd: &str, echo: bool, stdin: Option<&[u8]>) -> Result<String> {
	let mut args = shell_expand(cmd);
	let binary = args.remove(0);
	let current_dir = Env::with(|it| it.cwd().to_path_buf());

	if echo {
		println!("> {cmd}");
	}

	let mut command = Command::new(binary);
	command.args(args).current_dir(current_dir).stderr(Stdio::inherit());
	let output = match stdin {
		None => command.stdin(Stdio::null()).output(),
		Some(stdin) => {
			command.stdin(Stdio::piped()).stdout(Stdio::piped());
			let mut process = command.spawn()?;
			process.stdin.take().unwrap().write_all(stdin)?;
			process.wait_with_output()
		}
	}?;
	let stdout = String::from_utf8(output.stdout)?;

	if echo {
		print!("{stdout}");
	}

	if !output.status.success() {
		bail!("{}", output.status)
	}

	Ok(stdout.trim().to_string())
}

// FIXME: some real shell lexing here
fn shell_expand(cmd: &str) -> Vec<String> {
	let mut res = Vec::new();
	for (string_piece, in_quotes) in cmd.split('\'').zip([false, true].iter().copied().cycle()) {
		if in_quotes {
			res.push(string_piece.to_string());
		} else if !string_piece.is_empty() {
			res.extend(string_piece.split_ascii_whitespace().map(String::from));
		}
	}
	res
}

struct Env {
	pushd_stack: Vec<PathBuf>,
	push_env_stack: Vec<(OsString, Option<OsString>)>,
}

impl Env {
	fn with<F: FnOnce(&mut Env) -> T, T>(f: F) -> T {
		thread_local! {
			static ENV: RefCell<Env> = RefCell::new(Env {
				pushd_stack: vec![env::current_dir().unwrap()],
				push_env_stack: vec![],
			});
		}
		ENV.with(|it| f(&mut it.borrow_mut()))
	}

	fn pushd(&mut self, dir: PathBuf) {
		let dir = self.cwd().join(dir);
		self.pushd_stack.push(dir);
		env::set_current_dir(self.cwd())
			.unwrap_or_else(|err| panic!("Failed to set cwd to {}: {}", self.cwd().display(), err));
	}
	fn popd(&mut self) {
		self.pushd_stack.pop().unwrap();
		env::set_current_dir(self.cwd()).unwrap();
	}
	fn push_env(&mut self, var: OsString, value: OsString) {
		self.push_env_stack.push((var.clone(), env::var_os(&var)));
		// TODO: Audit that the environment access only happens in single-threaded code.
		unsafe { env::set_var(var, value) };
	}
	fn pop_env(&mut self) {
		let (var, value) = self.push_env_stack.pop().unwrap();
		match value {
			// TODO: Audit that the environment access only happens in single-threaded code.
			None => unsafe { env::remove_var(var) },
			// TODO: Audit that the environment access only happens in single-threaded code.
			Some(value) => unsafe { env::set_var(var, value) },
		}
	}
	fn cwd(&self) -> &Path {
		self.pushd_stack.last().unwrap()
	}
}
