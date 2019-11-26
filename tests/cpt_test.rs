use cpt::cpt;
use std::fs;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn args_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./example";
	let to = "./example_to";

	Command::cargo_bin("cpt")
		.unwrap()
		.args(&[from, to, r#"--json={ "foo": "bar" }"#])
		.assert()
		.success();

	assert_eq!(
		fs::read_to_string(format!("{}/bar.txt", to))?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	assert_eq!(
		fs::read_to_string(format!("{}/foo/non-template.txt", to))?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);

	Ok(())
}

#[test]
fn cpt_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./example";
	let to = "./example_to";
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from.to_string(), to.to_string(), &data)?;

	assert_eq!(
		fs::read_to_string(format!("{}/bar.txt", to))?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	assert_eq!(
		fs::read_to_string(format!("{}/foo/non-template.txt", to))?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);

	// Cleanup
	// fs::remove_dir_all(&to)?;
	Ok(())
}
