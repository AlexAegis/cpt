use cpt::args;

use cpt::{cp, cpt};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	let (from, to, data) = args::<String, String>("", "")?;
	print!("Copying from {}, to {}", from, to);
	if let Some(d) = data {
		println!(" with data: {:?}", &d);
		cpt(from, to, &d)?;
	} else {
		cp(from, to)?;
	}
	println!();
	Ok(())
}
