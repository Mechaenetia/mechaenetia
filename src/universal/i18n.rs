use bevy::prelude::*;
use fluent::{bundle::FluentBundle, FluentArgs, FluentError, FluentResource, FluentValue};
use fluent_syntax::ast::Pattern;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

type Bundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;
pub struct I18N {
	bundles: Vec<Bundle>,
}

pub struct I18NLanguageChangedEvent;
pub struct I18NChangeLanguageTo(pub LanguageIdentifier);

// #[derive(thiserror::Error, Debug)]
// pub enum I18NError {
// 	#[error("Resource parse error: {0:?}")]
// 	ResourceParseError(Vec<ParserError>),
// }

impl I18N {
	pub fn with_language_from_path(
		language: LanguageIdentifier,
		path: impl Into<PathBuf>,
	) -> Result<Self, std::io::Error> {
		let mut this = Self {
			bundles: Vec::with_capacity(2),
		};
		this.add_language_from_path(language, path)?;
		Ok(this)
	}

	pub fn swap_to_language(
		&mut self,
		language: LanguageIdentifier,
		path: impl Into<PathBuf>,
	) -> Result<&mut Self, std::io::Error> {
		let old_language = self.get_current_language();
		info!("Changing language from {} to {}", old_language, language);
		if self.bundles.len() > 1 {
			self.bundles.swap_remove(0);
			self.bundles.resize_with(1, || unreachable!());
		}
		self.add_language_from_path(language, path)
	}

	pub fn add_language_from_path(
		&mut self,
		language: LanguageIdentifier,
		path: impl Into<PathBuf>,
	) -> Result<&mut Self, std::io::Error> {
		for lang in self.bundles.iter().map(|b| b.locales.iter()).flatten() {
			if language == *lang {
				error!("Adding a language more than once: {}", &lang);
			}
		}
		let mut bundle = FluentBundle::new_concurrent(vec![language.clone()]);
		bundle.set_use_isolating(false); // TODO: Remove this when the gui system used can print unicode text properly...
		Self::add_language_resources_from_path(&mut bundle, &language.to_string(), path, false)?;
		self.bundles.insert(0, bundle);
		Ok(self)
	}

	fn add_language_resource(
		bundle: &mut Bundle,
		id: &str,
		resource_string: String,
		overriding: bool,
	) {
		let res = match FluentResource::try_new(resource_string) {
			Ok(res) => res,
			Err((res, errors)) => {
				for error in errors {
					error!("Parse error from `{}`: {:?}", id, error);
				}
				res
			}
		};
		if overriding {
			bundle.add_resource_overriding(res);
		} else {
			match bundle.add_resource(res) {
				Ok(()) => (),
				Err(errors) => {
					for error in errors {
						error!("Parse error from `{}`: {:?}", id, error);
					}
				}
			}
		}
	}

	fn add_language_resources_from_path(
		bundle: &mut Bundle,
		id: &str,
		path: impl Into<PathBuf>,
		overriding: bool,
	) -> Result<(), std::io::Error> {
		let path = path.into();
		info!("Adding language resources from path: {:?}", &path);
		if path.is_dir() {
			for f in std::fs::read_dir(&path)? {
				if let Ok(f) = f {
					Self::add_language_resources_from_path(bundle, id, f.path(), overriding)?;
				}
			}
		} else if path.is_file() {
			if path.extension() == Some(OsStr::new("lang")) {
				match std::fs::read_to_string(&path) {
					Ok(resource_string) => {
						Self::add_language_resource(bundle, id, resource_string, overriding);
					}
					Err(e) => {
						error!(
							"Failed reading language file `{:?}` because: {:?}",
							&path, e
						)
					}
				}
			} else {
				warn!(
					"Ignoring unknown file in language resource directory: {:?}",
					&path
				);
			}
		} else {
			error!(
				"Attempted to process language path that doesn't exist: {:?}",
				&path
			);
		}
		Ok(())
	}

	pub fn add_language_function<F>(
		&mut self,
		language: LanguageIdentifier,
		id: &str,
		func: F,
	) -> Result<&mut Self, FluentError>
	where
		F: for<'a> Fn(&[FluentValue<'a>], &FluentArgs<'_>) -> FluentValue<'a>
			+ Sync
			+ Send
			+ 'static,
	{
		for bundle in &mut self.bundles {
			if bundle.locales.contains(&language) {
				bundle.add_function(id, func)?;
				break;
			}
		}
		Ok(self)
	}

	fn format_string<'s>(
		bundle: &'s Bundle,
		id: &'s str,
		pattern: &'s Pattern<&str>,
		args: Option<&'s FluentArgs>,
	) -> Cow<'s, str> {
		let mut errs = vec![];
		let str = bundle.format_pattern(pattern, args, &mut errs);
		if !errs.is_empty() {
			error!(
				"Message Format Errors of message ID `{}` with pattern `{:?}` and args `{:?}`: {:?}",
				id, pattern, args, errs
			);
		}
		str
	}

	pub fn get<'i, 's: 'i>(&'s self, id: &'i str) -> Cow<'i, str> {
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.value() {
					return Self::format_string(&bundle, id, value, None);
				}
			}
		}

		error!("I18N Message ID value not found: {}", id);
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_with_args<'i, 's: 'i>(&'s self, id: &'i str, args: &'i FluentArgs) -> Cow<'i, str> {
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.value() {
					return Self::format_string(&bundle, id, value, Some(args));
				}
			}
		}

		error!("I18N Message ID value not found: {}", id);
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_with_args_list<'i, 's: 'i, K, V, I>(&'s self, id: &'i str, args: I) -> Cow<'i, str>
	where
		K: Into<Cow<'i, str>>,
		V: Into<FluentValue<'i>>,
		I: IntoIterator<Item = (K, V)>,
	{
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.value() {
					let args: FluentArgs<'i> = args.into_iter().collect();
					// We can't point to things in this stackframe (since it's about to pop), so have
					// to make this owned.
					return Cow::Owned(
						Self::format_string(&bundle, id, value, Some(&args)).into_owned(),
					);
				}
			}
		}

		error!("I18N Message ID value not found: {}", id);
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_attr<'i, 's: 'i>(&'s self, id: &'i str, attr: &'i str) -> Cow<'i, str> {
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.get_attribute(attr) {
					return Self::format_string(&bundle, id, value.value(), None);
				}
			}
		}

		error!(
			"I18N Message ID and attr values not found: {:?} {:?}",
			id, attr
		);
		Cow::Owned(format!("##~{}~@@~{}~##", id, attr))
	}

	pub fn get_attr_with_args<'i, 's: 'i>(
		&'s self,
		id: &'i str,
		attr: &'i str,
		args: &'i FluentArgs,
	) -> Cow<'i, str> {
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.get_attribute(attr) {
					return Self::format_string(&bundle, id, value.value(), Some(args));
				}
			}
		}

		error!(
			"I18N Message ID and attr values not found: {:?} {:?}",
			id, attr
		);
		Cow::Owned(format!("##~{}~@@~{}~##", id, attr))
	}

	pub fn get_attr_with_args_list<'i, 's: 'i, K, V, I>(
		&'s self,
		id: &'i str,
		attr: &'i str,
		args: I,
	) -> Cow<'i, str>
	where
		K: Into<Cow<'i, str>>,
		V: Into<FluentValue<'i>>,
		I: IntoIterator<Item = (K, V)>,
	{
		for bundle in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.get_attribute(attr) {
					let args: FluentArgs<'i> = args.into_iter().collect();
					// We can't point to things in the stackframe (since it's about to pop), so have
					// to make this owned.
					return Cow::Owned(
						Self::format_string(&bundle, id, value.value(), Some(&args)).into_owned(),
					);
				}
			}
		}

		error!(
			"I18N Message ID and attr values not found: {:?} {:?}",
			id, attr
		);
		Cow::Owned(format!("##~{}~@@~{}~##", id, attr))
	}

	pub fn get_current_language(&self) -> &LanguageIdentifier {
		self.bundles
			.first()
			.expect("Should always have at least one language bundle, have none")
			.locales
			.first()
			.expect("Locale should always be defined for language bundles")
	}
}

pub struct I18NPlugin {
	language: LanguageIdentifier,
}

impl I18NPlugin {
	pub fn new(language: LanguageIdentifier) -> Self {
		Self { language }
	}
}

impl Plugin for I18NPlugin {
	fn build(&self, app: &mut AppBuilder) {
		let mut lang = I18N::with_language_from_path(
			"en-US"
				.parse()
				.expect("Failed to parse `en-US` as a language"),
			"./assets/lang/en-US",
		)
		.expect("Failed to construct I18N resource for language: ");

		let mut language_path: PathBuf = "./assets/lang/".into();
		language_path.push(self.language.to_string());
		lang.add_language_from_path(self.language.clone(), language_path)
			.expect("Failed to load I18N language");

		app.add_event::<I18NLanguageChangedEvent>()
			.add_event::<I18NChangeLanguageTo>()
			.insert_resource(lang)
			.add_system(change_language.system());
	}
}

fn change_language(
	mut change: EventReader<I18NChangeLanguageTo>,
	mut lang: ResMut<I18N>,
	mut notify: EventWriter<I18NLanguageChangedEvent>,
) {
	let old_language = lang.get_current_language();
	if let Some(I18NChangeLanguageTo(language)) = change
		.iter()
		.filter(|I18NChangeLanguageTo(language)| language != old_language)
		.last()
	{
		let mut path: PathBuf = "./assets/lang".into();
		path.push(language.to_string());
		if path.exists() {
			lang.swap_to_language(language.clone(), path)
				.expect("Failed to swap to new language");
			notify.send(I18NLanguageChangedEvent);
		} else {
			warn!(
				"Requested to change to a language that does not exist: {}",
				language
			);
		}
	}
}

pub fn scan_languages_on_fs() -> Result<Vec<LanguageIdentifier>, std::io::Error> {
	let mut ret = Vec::with_capacity(10);
	for path in std::fs::read_dir("./assets/lang")?.flatten() {
		if let Ok(file_type) = path.file_type() {
			if file_type.is_dir() {
				if let Ok(lang) = path
					.path()
					.iter()
					.last()
					.and_then(|l| l.to_str())
					.ok_or(LanguageIdentifierError::Unknown)
					.and_then(|l| l.parse::<LanguageIdentifier>())
				{
					ret.push(lang);
				}
			}
		}
	}
	Ok(ret)
}
