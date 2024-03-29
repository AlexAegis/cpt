use handlebars::Handlebars;
use std::{
	cmp,
	error::Error,
	fmt::Debug,
	fs,
	fs::{DirBuilder, File},
	io::Write,
	path::{Path, PathBuf, MAIN_SEPARATOR},
	vec::Vec,
};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Cpt {
	pub from: String,
	pub to: String,
	pub data: Option<Box<serde_json::Value>>,
	pub dry: bool,
	pub force: bool,
}

impl Cpt {
	pub fn new(from: String, to: String) -> Self {
		Cpt {
			from,
			to,
			..Cpt::default()
		}
	}

	pub fn dry(self) -> Self {
		self.set_dry(true)
	}

	pub fn force(self) -> Self {
		self.set_force(true)
	}

	pub fn set_dry(mut self, dry: bool) -> Self {
		self.dry = dry;
		self
	}

	pub fn set_force(mut self, force: bool) -> Self {
		self.force = force;
		self
	}

	pub fn set_data(mut self, data: serde_json::Value) -> Self {
		self.data = Some(Box::from(data));
		self
	}

	pub fn try_data(mut self, data: Option<Box<serde_json::Value>>) -> Self {
		self.data = data;
		self
	}

	pub fn execute(self) -> Result<(), Box<dyn Error>> {
		let hb = Handlebars::new();
		let from_buf = PathBuf::from(&self.from);
		for entry in WalkDir::new(&self.from).into_iter().filter_map(|e| e.ok()) {
			let truncated_target = PathBuf::from(entry.path())
				.components()
				.map(Some)
				.zip(
					from_buf
						.components()
						.map(Some)
						.chain(std::iter::repeat(None))
						.take(entry.path().components().count()),
				)
				.skip_while(|(e, f)| match (&e, &f) {
					(Some(a), Some(b)) => a.as_os_str() == b.as_os_str(),
					_ => false,
				})
				.filter_map(|(e, _)| e)
				.collect::<PathBuf>();

			let target = Path::new(&self.to).join(&truncated_target);
			let targets: Vec<PathBuf>;

			if let Some(map) = &self.data {
				targets = target
					.components()
					.map(|c| c.as_os_str().to_string_lossy().into_owned())
					.map(|c| {
						hb.render_template(&c, &map)
							.unwrap_or_else(|_| c.to_string())
					})
					.map(|c| c.lines().map(|l| l.to_string()).collect::<Vec<String>>()) // Newline separation
					.fold(vec![], |acc: Vec<Vec<String>>, n| {
						let acc_l = acc.len();
						let n_l = n.len();
						let mut b = std::iter::repeat(acc)
							.take(n.len())
							.flatten()
							.collect::<Vec<Vec<String>>>();

						let b_l = cmp::max(n_l, 1) * cmp::max(acc_l, 1);
						b.resize_with(b_l, Vec::new);

						std::iter::repeat(
							n.into_iter()
								.map(|ne| std::iter::repeat(ne).take(cmp::max(acc_l, 1))),
						)
						.flatten()
						.flatten()
						.take(b.len())
						.enumerate()
						.for_each(|(i, ne)| {
							if let Some(e) = b.get_mut((i) % b_l) {
								e.push(ne);
							}
						});

						b
					})
					.into_iter()
					.map(|v| v.join(MAIN_SEPARATOR.to_string().as_str()))
					.map(PathBuf::from)
					.collect::<Vec<PathBuf>>();
			} else {
				targets = vec![target];
			}

			for mut trg in targets {
				println!("Creating {:?}", trg);
				if entry.path().is_dir() && !trg.exists() {
					if !self.dry {
						DirBuilder::new().recursive(true).create(&trg)?;
					}
				} else if entry.path().is_file() {
					let c: Vec<u8> = if let Ok(mut content) = fs::read_to_string(entry.path()) {
						if let Some(map) = &self.data {
							if let Some(e) = trg.extension() {
								// Use only tpl files as templates
								if e.to_str().ok_or("Error")? == "tpl" {
									trg.set_extension(""); // And strip the extension
									content = hb.render_template(&content, &map)?;
								}
							}
						}
						content.bytes().collect()
					} else {
						fs::read(entry.path())?
					};

					if !self.dry && (!trg.exists() || self.force) {
						let mut file = File::create(trg)?;
						file.write_all(c.as_slice())?;
						file.sync_all()?;
					}
				}
			}
		}
		Ok(())
	}
}

impl Default for Cpt {
	fn default() -> Self {
		Cpt {
			from: ".".to_string(),
			to: "./target".to_string(),
			data: None,
			dry: false,
			force: false,
		}
	}
}
