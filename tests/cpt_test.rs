use cpt::cpt;
use std::fs;

#[test]
fn cpt_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = String::from("./example");
	let to = String::from("./example_to");
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from, to, &data)?;

	assert_eq!(
		fs::read_to_string("./example_to/bar.txt")?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	assert_eq!(
		fs::read_to_string("./example_to/foo/non-template.txt")?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);
	Ok(())
}
