use cpt::{args::FromArgs, cp, cpt, Cpt};
use std::fs;

use assert_cmd::prelude::*;
use std::process::Command;

fn do_assert(to: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
	let from_og = "./templates/example".to_string();
	let to_og = "./templates/example_to_args".to_string();

	let c = Cpt::<String, String>::from_args(Some(&Cpt::new(from_og.clone(), to_og.clone())))?;

	assert_eq!(c.from, from_og);
	assert_eq!(c.to, to_og);
	assert_eq!(c.data, None);
	Ok(())
}

#[test]
fn cpt_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./templates/example";
	let to = "./templates/example_to_cpt";
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from.to_string(), to.to_string(), data)?;

	do_assert(to)?;

	// Cleanup
	fs::remove_dir_all(&to)?;

	Ok(())
}

#[test]
fn cp_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./templates/example";
	let to = "./templates/example_to_cp";

	cp(from.to_string(), to.to_string())?;

	let bar_path = format!("{}/bar.txt.tpl", to);

	assert_eq!(
		fs::read_to_string(bar_path)?.replace("\r\n", "\n"),
		"This will become bar: {{foo}}\nAnd this will stay as is: \\{{escaped}}\n"
	);

	let nontemp_path = format!("{}/foo/non-template.txt", to);

	assert_eq!(
		fs::read_to_string(nontemp_path)?.replace("\r\n", "\n"),
		"unused: {{unused}}"
	);

	// Cleanup
	fs::remove_dir_all(&to)?;

	Ok(())
}

#[test]
fn bin_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./templates/example";
	let to = "./templates/example_to_args";

	Command::cargo_bin("cpt")
		.unwrap()
		.args(&[from, to, r#"--json={ "foo": "bar" }"#])
		.assert()
		.success();

	do_assert(to)?;
	// Cleanup
	fs::remove_dir_all(&to)?;

	Ok(())
}
