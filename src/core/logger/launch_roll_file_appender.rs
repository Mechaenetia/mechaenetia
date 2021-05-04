//! This is an appender that rolls a file on launch and then delegates to another appender

use log4rs::append::rolling_file::policy::compound::roll::Roll;
use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};
use serde_value::Value;
use std::collections::BTreeMap;
use std::path::Path;

#[serde(deny_unknown_fields)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, serde::Deserialize)]
pub struct LaunchRollFileAppenderConfig {
	appender: Appender,
	path: String,
	launch_roller: Roller,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Appender {
	kind: String,
	config: Value,
}

impl<'de> serde::Deserialize<'de> for Appender {
	fn deserialize<D>(d: D) -> Result<Appender, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

		let kind = match map.remove(&Value::String("kind".to_owned())) {
			Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
			None => return Err(serde::de::Error::missing_field("kind")),
		};

		Ok(Appender {
			kind,
			config: Value::Map(map),
		})
	}
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Roller {
	kind: String,
	config: Value,
}

impl<'de> serde::Deserialize<'de> for Roller {
	fn deserialize<D>(d: D) -> Result<Roller, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

		let kind = match map.remove(&Value::String("kind".to_owned())) {
			Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
			None => return Err(serde::de::Error::missing_field("kind")),
		};

		Ok(Roller {
			kind,
			config: Value::Map(map),
		})
	}
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Policy {
	kind: String,
	config: Value,
}

impl<'de> serde::Deserialize<'de> for Policy {
	fn deserialize<D>(d: D) -> Result<Policy, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

		let kind = match map.remove(&Value::String("kind".to_owned())) {
			Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
			None => "compound".to_owned(),
		};

		Ok(Policy {
			kind,
			config: Value::Map(map),
		})
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct RollFileOnLaunchAppenderDeserializer;

impl Deserialize for RollFileOnLaunchAppenderDeserializer {
	type Trait = dyn Append;

	type Config = LaunchRollFileAppenderConfig;

	fn deserialize(
		&self,
		config: LaunchRollFileAppenderConfig,
		deserializers: &Deserializers,
	) -> anyhow::Result<Box<dyn Append>> {
		let path = Path::new(&config.path);
		if path.exists() && path.is_file() {
			let launch_roller: Box<dyn Roll> = deserializers
				.deserialize(&config.launch_roller.kind, config.launch_roller.config)?;
			launch_roller.roll(path)?;
		}

		let appender = deserializers.deserialize(&config.appender.kind, config.appender.config)?;

		Ok(appender)
	}
}
