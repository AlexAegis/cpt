use serde::{de::DeserializeOwned, Serialize};
use std::{cmp::Eq, collections::HashMap, error::Error, hash::Hash};

pub mod args;
pub mod model;
pub use model::Cpt;

pub fn cp(from: String, to: String) -> Result<(), Box<dyn Error>> {
	Cpt::<String, String>::new(from, to).execute()
}

pub fn cpt<K, V, S>(from: String, to: String, data: HashMap<K, V, S>) -> Result<(), Box<dyn Error>>
where
	K: Hash + Eq + DeserializeOwned + Serialize,
	V: Hash + Eq + DeserializeOwned + Serialize,
	S: std::hash::BuildHasher + Default,
{
	Cpt::<K, V, S>::new(from, to).set_data(data).execute()
}
