use clap::{Arg, Command};
use std::error::Error;

use crate::Cpt;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub trait FromArgs: Default {
	fn from_args(defaults: Option<&Self>) -> Result<Self, Box<dyn Error>>;
}

impl FromArgs for Cpt {
	#[cfg_attr(tarpaulin, skip)]
	fn from_args(defaults: Option<&Cpt>) -> Result<Cpt, Box<dyn Error>> {
		let plain_def = Self::default();
		let def = defaults.unwrap_or(&plain_def);
		let m = Command::new("cpt")
			.version(VERSION.unwrap_or("unknown"))
			.about(
				"Copies one folder structure to another place with files. Also formats templates!",
			)
			.author("AlexAegis")
			.arg(
				Arg::new("from")
					.index(1)
					.help("The folder that will be copied"),
			)
			.arg(
				Arg::new("to")
					.index(2)
					.help("The folder where the folder will be placed"),
			)
			.arg(
				Arg::new("json")
					.short('j')
					.long("json")
					.value_parser(clap::value_parser!(String))
					.help("JSON formatted templating data"),
			)
			.arg(
				Arg::new("dry")
					.short('d')
					.long("dry")
					.action(clap::ArgAction::SetTrue)
					.help("If set, nothing will be written to the disk"),
			)
			.arg(
				Arg::new("force")
					.short('f')
					.long("force")
					.action(clap::ArgAction::SetTrue)
					.help("If set, files can be overwritten in the target folder"),
			)
			.arg(Arg::new("quiet").short('q').long("quiet").help("Tarpaulin"))
			.get_matches();

		let from = m.get_one("from").unwrap_or(&def.from);
		let to = m.get_one("to").unwrap_or(&def.to);
		let dry = m.get_flag("dry");
		let force = m.get_flag("force");

		// if !Path::new(&from).exists() {
		// 	panic!("from doesn't exist")
		// }

		let data_map = m
			.get_one::<String>("json")
			.map(|i| i.trim())
			.map(|data_str| {
				println!("data: {:?}", data_str);
				Box::new(
					serde_json::from_str::<serde_json::Value>(data_str)
						.expect("--json is not valid json"),
				)
			});

		Ok(Cpt::new(from.to_string(), to.to_string())
			.try_data(data_map)
			.set_dry(dry)
			.set_force(force))
	}
}
