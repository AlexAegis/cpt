use cpt::cpt;
use std::fs;

#[test]
fn cpt_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = String::from("./example");
	let to = String::from("./example_to");
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from, to.clone(), &data)?;

	assert_eq!(
		fs::read_to_string(to.clone() + "/bar.txt")?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	assert_eq!(
		fs::read_to_string(to.clone() + "/foo/non-template.txt")?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);

	// Cleanup
	fs::remove_dir_all(&to)?;
	Ok(())
}
