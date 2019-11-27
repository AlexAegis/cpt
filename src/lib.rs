use clap::{App, Arg};
use handlebars::Handlebars;
use serde::{de::DeserializeOwned, Serialize};
use std::{
	cmp,
	cmp::Eq,
	collections::{hash_map::RandomState, HashMap},
	error::Error,
	fs,
	fs::{DirBuilder, File},
	hash::Hash,
	io::Write,
	path::{Path, PathBuf, MAIN_SEPARATOR},
	vec::Vec,
};

use walkdir::WalkDir;

pub struct CptConfig {
	/// Does not write anything to the disk, just logs
	dry: bool,
	/// Allows overwriteing existing files
	force: bool,
}

impl Default for CptConfig {
	fn default() -> Self {
		CptConfig {
			dry: true,
			force: false,
		}
	}
}

impl CptConfig {
	pub fn new(dry: bool, force: bool) -> Self {
		CptConfig { dry, force }
	}
}

pub fn cp(from: String, to: String) -> Result<(), Box<dyn Error>> {
	cpt_inner::<String, String, RandomState>(from, to, None, CptConfig::default())
}

pub fn cpt<K, V, S: std::hash::BuildHasher + Default>(
	from: String,
	to: String,
	data: &HashMap<K, V, S>,
) -> Result<(), Box<dyn Error>>
where
	K: Hash + Eq + DeserializeOwned + Serialize,
	V: Hash + Eq + DeserializeOwned + Serialize,
{
	cpt_inner(from, to, Some(data), CptConfig::default())
}

/// Copy with templates
fn cpt_inner<K, V, S: std::hash::BuildHasher + Default>(
	from: String,
	to: String,
	data: Option<&HashMap<K, V, S>>,
	config: CptConfig,
) -> Result<(), Box<dyn Error>>
where
	K: Hash + Eq + DeserializeOwned + Serialize,
	V: Hash + Eq + DeserializeOwned + Serialize,
{
	let hb = Handlebars::new();
	for entry in WalkDir::new(from).into_iter().filter_map(|e| e.ok()) {
		let truncated_target = PathBuf::from(entry.path())
			.components()
			.skip_while(|c| c.as_os_str() == ".")
			.skip(1)
			.collect::<PathBuf>();
		let target = Path::new(&to).join(&truncated_target);

		let targets: Vec<PathBuf>;

		if let Some(map) = &data {
			let rs = target
				.components()
				.map(|c| c.as_os_str().to_string_lossy().into_owned())
				.map(|c| {
					println!("component before template {:?}", c);
					c
				})
				.map(|c| {
					hb.render_template(&c, &map)
						.unwrap_or_else(|_| c.to_string())
				})
				.map(|c| {
					println!("component after template {:?}", c);
					c
				})
				.map(|c| c.lines().map(|l| l.to_string()).collect::<Vec<String>>())
				.fold(vec![], |acc: Vec<Vec<String>>, n| {
					println!("acc {:?}, \n\t\tnext {:?}", &acc, &n);

					let acc_l = acc.len();
					let n_l = n.len();
					let mut b = std::iter::repeat(acc)
						.take(n.len())
						.flatten()
						.collect::<Vec<Vec<String>>>();

					let b_l = cmp::max(n_l, 1) * cmp::max(acc_l, 1);
					b.resize_with(b_l, || vec![]);

					std::iter::repeat(n)
						.flatten()
						.take(b.len())
						.enumerate()
						.for_each(|(i, ne)| {
							if let Some(e) = b.get_mut((i + n_l) % b_l) {
								e.push(ne);
							}
						});

					b
				})
				.into_iter()
				.map(|v| v.join(MAIN_SEPARATOR.to_string().as_str()))
				.collect::<Vec<String>>();

			println!("rs {:?}", &rs);

			targets = vec![target];
		} else {
			targets = vec![target];
		}

		println!("Creating {:?}", &targets);

		for mut trg in targets {
			if entry.path().is_dir() && !trg.exists() {
				if !config.dry {
					DirBuilder::new().recursive(true).create(&trg)?;
				}
			} else if entry.path().is_file() && (!trg.exists() || config.force) {
				// TODO: Test force mode
				let mut content = fs::read_to_string(entry.path())?;
				if let Some(map) = &data {
					if let Some(e) = trg.extension() {
						// Use only tpl files as templates
						if e.to_str().ok_or("Error")? == "tpl" {
							trg.set_extension(""); // And strip the extension
							content = hb.render_template(&content, &map)?;
						}
					}
				}
				if !config.dry {
					let mut file = File::create(trg)?;
					file.write_all(content.as_bytes())?;
					file.sync_all()?;
				}
			}
		}
	}
	Ok(())
}

pub type Args<K, V> = (String, String, Option<HashMap<K, V>>);

#[cfg_attr(tarpaulin, skip)]
pub fn args<K, V>(default_from: &str, default_to: &str) -> Result<Args<K, V>, Box<dyn Error>>
where
	K: Hash + Eq + DeserializeOwned,
	V: Hash + Eq + DeserializeOwned,
{
	let m = App::new("cpt")
		.version("1.0.0")
		.about("Copies one folder structure to another place with files. Also formats templates!")
		.author("AlexAegis")
		.arg(
			Arg::with_name("from")
				.short("f")
				.long("from")
				.required(true)
				.index(1)
				.default_value(default_from)
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
				.default_value(default_to)
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

	let mut data_map = None;
	if m.args.contains_key("json") {
		if let Some(d) = m.args["json"].vals.first() {
			let data_str = d.to_str().ok_or("Invalid string")?;
			data_map.replace(serde_json::from_str::<HashMap<K, V>>(&data_str)?);
		}
	}

	Ok((from.to_string(), to.to_string(), data_map))
}
