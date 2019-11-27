use clap::{App, Arg};
use serde::{de::DeserializeOwned, Serialize};
use std::{cmp::Eq, collections::HashMap, error::Error, fmt::Debug, hash::Hash, path::Path};

use crate::Cpt;

pub trait FromArgs: Default {
	fn from_args(defaults: Option<&Self>) -> Result<Self, Box<dyn Error>>;
}

impl<K, V, S> FromArgs for Cpt<K, V, S>
where
	K: Hash + Eq + DeserializeOwned + Serialize + Debug,
	V: Hash + Eq + DeserializeOwned + Serialize + Debug,
	S: std::hash::BuildHasher + Default + Debug,
{
	#[cfg_attr(tarpaulin, skip)]
	fn from_args(defaults: Option<&Cpt<K, V, S>>) -> Result<Cpt<K, V, S>, Box<dyn Error>> {
		let plain_def = Self::default();
		let def = defaults.unwrap_or(&plain_def);
		let m = App::new("cpt")
			.version("1.0.0")
			.about(
				"Copies one folder structure to another place with files. Also formats templates!",
			)
			.author("AlexAegis")
			.arg(
				Arg::with_name("from")
					.short("f")
					.long("from")
					.required(true)
					.index(1)
					.default_value(&def.from)
					.validator(|s| {
						if Path::new(&s).exists() {
							Ok(())
						} else {
							Err("Source folder not exists".to_string())
						}
					})
					.help("The folder that will be copied"),
			)
			.arg(
				Arg::with_name("to")
					.short("t")
					.long("to")
					.required(true)
					.index(2)
					.default_value(&def.to)
					.help("The folder where the folder will be placed"),
			)
			.arg(
				Arg::with_name("json")
					.short("-j")
					.long("--json")
					.takes_value(true)
					.validator(|s| match serde_json::from_str::<HashMap<K, V>>(&s) {
						Ok(_) => Ok(()),
						Err(e) => Err(e.to_string()),
					})
					.help("JSON formatted templating data"),
			)
			.arg(
				Arg::with_name("dry")
					.short("-d")
					.long("--dry")
					.help("If set, nothing will be written to the disk"),
			)
			.arg(
				Arg::with_name("force")
					.short("-f")
					.long("--force")
					.help("If set, files can be overwritten in the target folder"),
			)
			.arg(
				Arg::with_name("quiet")
					.short("-q")
					.long("--quiet")
					.help("Tarpaulin"),
			)
			.get_matches();

		let from = m.args["from"]
			.vals
			.first()
			.ok_or("No from specified")?
			.to_str()
			.ok_or("Invalid string")?;
		let to = m.args["to"]
			.vals
			.first()
			.ok_or("No to specified")?
			.to_str()
			.ok_or("Invalid string")?;

		let dry = m.args.get("dry").is_some();
		let force = m.args.get("force").is_some();

		let mut data_map = None;
		if m.args.contains_key("json") {
			if let Some(d) = m.args["json"].vals.first() {
				let data_str = d.to_str().ok_or("Invalid string")?;
				data_map.replace(Box::from(serde_json::from_str::<HashMap<K, V, S>>(
					&data_str,
				)?));
			}
		}

		Ok(Cpt::new(from.to_string(), to.to_string())
			.try_data(data_map)
			.set_dry(dry)
			.set_force(force))
	}
}
