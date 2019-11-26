use cpt::cpt;
use std::fs;
use std::{thread, time};

use assert_cmd::prelude::*;
use std::process::Command;

fn do_assert() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let to = "./example_to";
	let bar_path = format!("{}/bar.txt", to);

	assert_eq!(
		fs::read_to_string(bar_path)?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	let nontemp_path = format!("{}/foo/non-template.txt", to);

	assert_eq!(
		fs::read_to_string(nontemp_path)?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);

	Ok(())
}

#[test]
fn args_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./example";
	let to = "./example_to";

	Command::cargo_bin("cpt")
		.unwrap()
		.args(&[from, to, r#"--json={ "foo": "bar" }"#])
		.assert()
		.success();

	do_assert()?;
	// Cleanup
	fs::remove_dir_all(&to)?;

	Ok(())
}
