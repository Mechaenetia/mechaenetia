use bevy::app::Events;
use bevy::asset::{AssetLoader, AssetServerError, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource, FluentValue};
use fluent_syntax::ast::Pattern;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

#[derive(Debug, TypeUuid)]
#[uuid = "4df317fb-0581-44f6-8b5f-7cbf12ddc460"]
pub struct I18NLanguageFile(FluentResource);

#[derive(Default)]
pub struct I18NLanguageFileAssetLoader;

impl AssetLoader for I18NLanguageFileAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, anyhow::Result<()>> {
		Box::pin(async move {
			let data = String::from_utf8(Vec::from(bytes))?;
			let res = match FluentResource::try_new(data) {
				Ok(res) => res,
				Err((res, errors)) => {
					for error in errors {
						error!(
							"`FluentResource` parse error from `{:?}`: {:?}",
							load_context.path(),
							error
						);
					}
					res
				}
			};
			load_context.set_default_asset(LoadedAsset::new(I18NLanguageFile(res)));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["lang"]
	}
}

type Bundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

pub struct I18N {
	root_path: PathBuf,
	bundles: Vec<(Vec<(bool, Handle<I18NLanguageFile>)>, Bundle)>,
}

pub struct I18NLanguageChangedEvent;
pub struct I18NChangeLanguageTo(pub Vec<LanguageIdentifier>);

pub struct I18NPlugin {
	root_path: PathBuf,
	languages: Vec<LanguageIdentifier>,
}

impl I18NPlugin {
	pub fn new(root_path: PathBuf, languages: Vec<LanguageIdentifier>) -> Self {
		Self {
			root_path,
			languages,
		}
	}
}

impl Plugin for I18NPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<I18NLanguageChangedEvent>()
			.add_event::<I18NChangeLanguageTo>()
			.add_asset::<I18NLanguageFile>()
			.init_asset_loader::<I18NLanguageFileAssetLoader>();

		let mut lang = I18N::new(self.root_path.clone());

		{
			let world = app.app.world.cell();
			let asset_server = world
				.get_resource::<AssetServer>()
				.expect("`AssetServer` must be registered as a resource before `I18N` is built");
			let assets = world.get_resource::<Assets<I18NLanguageFile>>().expect("just registered asset is apparently missing its assets container for `I18NLanguageFile`");
			let mut changed_events= world.get_resource_mut::<Events<I18NLanguageChangedEvent>>().expect("just registered event is apparently missing its resource for I18NLanguageChangedEvent");
			lang.change_language_to(
				&self.languages,
				&*asset_server,
				&*assets,
				&mut changed_events,
			)
			.expect("initial language selected does not exist");
		}

		app.insert_resource(lang)
			.add_system(language_asset_loaded.system())
			.add_system(change_language.system());
	}
}

fn language_asset_loaded(
	mut ev_asset: EventReader<AssetEvent<I18NLanguageFile>>,
	assets: Res<Assets<I18NLanguageFile>>,
	mut changed: EventWriter<I18NLanguageChangedEvent>,
	mut lang: ResMut<I18N>,
) {
	for ev in ev_asset.iter() {
		info!("language asset state changed: {:?}", &ev);
		match ev {
			AssetEvent::Created { handle } => {
				lang.update_bundle_for_matching_handle_asset(&handle, &*assets, &mut changed)
			}
			AssetEvent::Modified { handle } => {
				lang.update_bundle_for_matching_handle_asset(&handle, &*assets, &mut changed)
			}
			AssetEvent::Removed { handle } => {
				let _ = lang.remove_tracked_handle(&handle, &*assets, &mut changed);
			}
		}
	}
}

// #[derive(thiserror::Error, Debug)]
// pub enum I18NError {
// 	#[error("Resource parse error: {0:?}")]
// 	ResourceParseError(Vec<ParserError>),
// }

impl I18N {
	pub fn new(root_path: PathBuf) -> Self {
		Self {
			root_path,
			bundles: vec![],
		}
	}

	pub fn remaining_to_load(&self) -> usize {
		self.bundles
			.iter()
			.map(|(handles, _bundle)| handles)
			.flatten()
			.filter(|(loaded, _handle)| !*loaded)
			.count()
	}

	pub fn is_fully_loaded(&self) -> bool {
		self.bundles
			.iter()
			.map(|(handles, _bundle)| handles)
			.flatten()
			.all(|(loaded, _handle)| *loaded)
	}

	fn update_bundle_for_matching_handle_asset(
		&mut self,
		handle: &Handle<I18NLanguageFile>,
		assets: &Assets<I18NLanguageFile>,
		changed: &mut EventWriter<I18NLanguageChangedEvent>,
	) {
		if let Some(asset) = assets.get(handle) {
			for (handles, bundle) in self.bundles.iter_mut() {
				if let Some((loaded, _handle)) = handles.iter_mut().find(|t| t.1 == *handle) {
					// Yes this re-parse is bad but FluentResource doesn't implement `Clone`.
					// Ignoring errors since they were reported in the Asset itself earlier.
					let res = FluentResource::try_new(asset.0.source().to_owned())
						.unwrap_or_else(|(res, _errors)| res);
					if *loaded {
						bundle.add_resource_overriding(res);
					} else {
						if let Err(errors) = bundle.add_resource(res) {
							for error in errors {
								error!("duplicate message already exists in bundle: {:?}", error);
							}
						}
					}
					*loaded = true;
					break;
				}
			}
			if self.is_fully_loaded() {
				changed.send(I18NLanguageChangedEvent);
			}
		}
	}

	fn remove_tracked_handle(
		&mut self,
		handle: &Handle<I18NLanguageFile>,
		assets: &Assets<I18NLanguageFile>,
		changed: &mut EventWriter<I18NLanguageChangedEvent>,
	) -> Option<usize> {
		for (idx, (handles, _bundle)) in self.bundles.iter_mut().enumerate() {
			if let Some(i) = handles
				.iter()
				.map(|(_enabled, handle)| handle)
				.enumerate()
				.find_map(|(i, h)| if h == handle { Some(i) } else { None })
			{
				handles.swap_remove(i);
				self.reload_bundle_assets_at(assets, changed, idx);
				return Some(idx);
			}
		}
		None
	}

	fn reload_bundle_assets_at(
		&mut self,
		assets: &Assets<I18NLanguageFile>,
		changed: &mut EventWriter<I18NLanguageChangedEvent>,
		idx: usize,
	) {
		let (handles, bundle) = &mut self.bundles[idx];
		*bundle = Bundle::new_concurrent(bundle.locales.clone());
		bundle.set_use_isolating(false);
		for (_enabled, handle) in handles.iter() {
			if let Some(asset) = assets.get(handle) {
				// Yes this re-parse is bad but FluentResource doesn't implement `Clone`.
				// Ignoring errors since they were reported in the Asset itself earlier.
				let res = FluentResource::try_new(asset.0.source().to_owned())
					.unwrap_or_else(|(res, _errors)| res);
				bundle.add_resource_overriding(res);
			}
		}
		if self.is_fully_loaded() {
			changed.send(I18NLanguageChangedEvent);
		}
		todo!()
	}

	fn init_bundle_from_language(
		root_path: &Path,
		asset_server: &AssetServer,
		assets: &Assets<I18NLanguageFile>,
		language: &LanguageIdentifier,
	) -> Result<(Vec<(bool, Handle<I18NLanguageFile>)>, Bundle), AssetServerError> {
		let mut bundle = Bundle::new_concurrent(vec![language.clone()]);
		bundle.set_use_isolating(false);
		let mut path = root_path.to_owned();
		path.push(language.to_string());
		let handles = asset_server
			.load_folder(&path)?
			.iter()
			.map(|h| {
				let handle = h.clone().typed::<I18NLanguageFile>();
				// Pre-emptively try to load the asset if already available
				let loaded = if let Some(asset) = assets.get(&handle) {
					let res = FluentResource::try_new(asset.0.source().to_owned())
						.unwrap_or_else(|(res, _errors)| res);
					if let Err(errors) = bundle.add_resource(res) {
						for error in errors {
							error!("duplicate message already exists in bundle: {:?}", error);
						}
					}
					true
				} else {
					false
				};
				(loaded, handle)
			})
			.collect();
		Ok((handles, bundle))
	}

	pub fn change_language_to(
		&mut self,
		languages: &Vec<LanguageIdentifier>,
		asset_server: &AssetServer,
		assets: &Assets<I18NLanguageFile>,
		changed: &mut Events<I18NLanguageChangedEvent>,
	) -> Result<(), AssetServerError> {
		if self.bundles.len() != languages.len()
			|| self
				.bundles
				.iter()
				.map(|(_handles, bundle)| bundle.locales.first())
				.zip(languages.iter())
				.any(|(a, b)| a != Some(b))
		{
			info!("changing language to: {:?}", languages);
			self.bundles = languages
				.iter()
				.map(|l| Self::init_bundle_from_language(&self.root_path, asset_server, assets, l))
				.collect::<Result<_, _>>()?;
			if self.is_fully_loaded() {
				changed.send(I18NLanguageChangedEvent);
			}
			Ok(())
		} else {
			info!(
				"language change requested but it is already that language: {:?}",
				languages
			);
			Ok(())
		}
	}

	// pub fn add_language_function<F>(
	// 	&mut self,
	// 	language: LanguageIdentifier,
	// 	id: &str,
	// 	func: F,
	// ) -> Result<&mut Self, FluentError>
	// where
	// 	F: for<'a> Fn(&[FluentValue<'a>], &FluentArgs<'_>) -> FluentValue<'a>
	// 		+ Sync
	// 		+ Send
	// 		+ 'static,
	// {
	// 	for bundle in &mut self.bundles {
	// 		if bundle.locales.contains(&language) {
	// 			bundle.add_function(id, func)?;
	// 			break;
	// 		}
	// 	}
	// 	Ok(self)
	// }

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
		for (_handles, bundle) in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.value() {
					return Self::format_string(&bundle, id, value, None);
				}
			}
		}

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` not found for language `{}`",
				id, locale
			);
		}
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_with_args<'i, 's: 'i>(&'s self, id: &'i str, args: &'i FluentArgs) -> Cow<'i, str> {
		for (_handles, bundle) in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.value() {
					return Self::format_string(&bundle, id, value, Some(args));
				}
			}
		}

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` not found for language `{}`",
				id, locale
			);
		}
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_with_args_list<'i, 's: 'i, K, V, I>(&'s self, id: &'i str, args: I) -> Cow<'i, str>
	where
		K: Into<Cow<'i, str>>,
		V: Into<FluentValue<'i>>,
		I: IntoIterator<Item = (K, V)>,
	{
		for (_handles, bundle) in self.bundles.iter() {
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

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` not found for language `{}`",
				id, locale
			);
		}
		Cow::Owned(format!("##~{}~##", id))
	}

	pub fn get_attr<'i, 's: 'i>(&'s self, id: &'i str, attr: &'i str) -> Cow<'i, str> {
		for (_handles, bundle) in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.get_attribute(attr) {
					return Self::format_string(&bundle, id, value.value(), None);
				}
			}
		}

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` and attr `{}` not found for language `{}`",
				id, attr, locale
			);
		}
		Cow::Owned(format!("##~{}~@@~{}~##", id, attr))
	}

	pub fn get_attr_with_args<'i, 's: 'i>(
		&'s self,
		id: &'i str,
		attr: &'i str,
		args: &'i FluentArgs,
	) -> Cow<'i, str> {
		for (_handles, bundle) in self.bundles.iter() {
			if let Some(msg) = bundle.get_message(id) {
				if let Some(value) = msg.get_attribute(attr) {
					return Self::format_string(&bundle, id, value.value(), Some(args));
				}
			}
		}

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` and attr `{}` not found for language `{}`",
				id, attr, locale
			);
		}
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
		for (_handles, bundle) in self.bundles.iter() {
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

		if let Some(locale) = self
			.bundles
			.first()
			.map(|(_handles, bundle)| bundle.locales.first())
			.flatten()
		{
			error!(
				"I18N Message ID `{}` and attr `{}` not found for language `{}`",
				id, attr, locale
			);
		}
		Cow::Owned(format!("##~{}~@@~{}~##", id, attr))
	}

	pub fn get_current_language(&self) -> LanguageIdentifier {
		self.bundles
			.first()
			.map(|(_handles, b)| b.locales.first().cloned().unwrap_or_else(Default::default))
			.unwrap_or_else(Default::default)
	}
}

fn change_language(
	mut change: EventReader<I18NChangeLanguageTo>,
	mut lang: ResMut<I18N>,
	asset_server: Res<AssetServer>,
	assets: Res<Assets<I18NLanguageFile>>,
	mut changed_events: ResMut<Events<I18NLanguageChangedEvent>>,
) {
	if let Some(I18NChangeLanguageTo(languages)) = change.iter().last() {
		match lang.change_language_to(languages, &asset_server, &assets, &mut *changed_events) {
			Ok(()) => {}
			Err(e) => {
				error!("failed changing language with error: `{:?}", e);
			}
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
