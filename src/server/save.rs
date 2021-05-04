use bevy::prelude::*;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveConfig {
	#[serde(skip)]
	save_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum SaveConfigError {
	#[error("IO error while {1}")]
	LoadError(#[source] std::io::Error, &'static str),
	#[error("non-valid save at path: {0:?}")]
	InvalidSave(PathBuf),
	#[error("ron format error")]
	RonError(#[from] ron::Error),
}

pub enum SaveLoadState {
	Created(SaveConfig),
	Existing(SaveConfig),
}

impl SaveConfig {
	pub fn load_path(path: impl AsRef<Path>) -> Result<SaveConfig, SaveConfigError> {
		let path = path.as_ref();
		let config_path = path.to_owned().join("config.ron");
		let config_string = std::fs::read_to_string(&config_path)
			.map_err(|e| SaveConfigError::LoadError(e, "reading config.ron file"))?;
		let mut save_config: SaveConfig = ron::from_str(&config_string)?;
		save_config.save_path = path.to_owned();
		trace!("Loaded a SaveConfig at: {:?}", path);
		Ok(save_config)
	}

	pub fn load_or_create_path(path: impl AsRef<Path>) -> Result<SaveLoadState, SaveConfigError> {
		let path = path.as_ref();
		if let Ok(config) = Self::load_path(path) {
			return Ok(SaveLoadState::Existing(config));
		}
		if !path.is_dir() {
			std::fs::create_dir_all(&path)
				.map_err(|e| SaveConfigError::LoadError(e, "creating save directory"))?;
		}
		let mut path: PathBuf = path.into();
		path.push("config.ron");
		if path.exists() {
			return Err(SaveConfigError::InvalidSave(path));
		}

		let empty_config = SaveConfig {
			save_path: path.to_owned(),
			..Default::default()
		};
		let config_string = ron::ser::to_string_pretty(
			&empty_config,
			PrettyConfig::new()
				.with_new_line("\n".to_owned())
				.with_indentor("\t".to_owned()),
		)? + "\n";

		info!("Writing a new SaveConfig to: {:?}", &path);
		std::fs::write(&path, config_string)
			.map_err(|e| SaveConfigError::LoadError(e, "writing empty configuration"))?;

		Ok(SaveLoadState::Created(empty_config))
	}
}
