use cpt::args;

use cpt::cp;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("asd");
	let (from, to, data) = args()?;
	print!("Copying from {}, to {}", from, to);
	if let Some(d) = &data {
		println!(" with data: {:?}", &d);
	}
	println!();
	cp(from, to)?;
	Ok(())
}
