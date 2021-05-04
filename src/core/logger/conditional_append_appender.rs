//! This is an appender that rolls a file on launch and then delegates to another appender

use crate::universal::conditional_map::ConditionalMap;
use bevy::utils::tracing::log::Record;
use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};
use serde_value::Value;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[serde(deny_unknown_fields)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, serde::Deserialize)]
pub struct ConditionallyAppendAppenderConfig {
	appender: Appender,
	id: String,
	default_enabled: Option<bool>,
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ConditionallyAppendAppenderDeserializer;

impl Deserialize for ConditionallyAppendAppenderDeserializer {
	type Trait = dyn Append;

	type Config = ConditionallyAppendAppenderConfig;

	fn deserialize(
		&self,
		config: ConditionallyAppendAppenderConfig,
		deserializers: &Deserializers,
	) -> anyhow::Result<Box<dyn Append>> {
		let appender = deserializers.deserialize(&config.appender.kind, config.appender.config)?;

		Ok(Box::new(ConditionallyAppendAppender {
			appender,
			id: config.id.clone(),
			state: ConditionalMap::get_or_create_by_id(
				config.id,
				config.default_enabled.unwrap_or(true),
			),
		}))
	}
}

#[derive(Debug)]
pub struct ConditionallyAppendAppender {
	appender: Box<dyn Append>,
	id: String,
	state: Arc<AtomicBool>,
}

impl Append for ConditionallyAppendAppender {
	fn append(&self, record: &Record) -> anyhow::Result<()> {
		if self.state.load(Ordering::Relaxed) {
			self.appender.append(record)
		} else {
			Ok(())
		}
	}

	fn flush(&self) {
		self.appender.flush();
	}
}
