use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct ConditionalMap {
	map: RwLock<HashMap<String, Arc<AtomicBool>>>,
}

lazy_static::lazy_static! {
	static ref CONDITIONAL_MAP: ConditionalMap = ConditionalMap::default();
}

impl ConditionalMap {
	pub fn get_by_id(id: &str) -> Option<Arc<AtomicBool>> {
		CONDITIONAL_MAP
			.map
			.read()
			.expect("poisoned CONDITIONAL_MAP lock")
			.get(id)
			.map(|b| b.clone())
	}

	pub fn get_or_create_by_id(id: String, default: bool) -> Arc<AtomicBool> {
		CONDITIONAL_MAP
			.map
			.write()
			.expect("poisoned CONDITIONAL_MAP lock")
			.entry(id)
			.or_insert_with(|| Arc::new(AtomicBool::new(default)))
			.clone()
	}
}
