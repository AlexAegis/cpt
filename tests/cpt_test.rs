use cpt::cpt;
use std::fs;
use std::{thread, time};

use assert_cmd::prelude::*;
use std::process::Command;

fn do_assert() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let to = "./example_to";
	let bar_path = format!("{}/bar.txt", to);
	let mut bar_try = 0;
	while fs::read(&bar_path).is_err() && bar_try < 1000 {
		println!("CRAWLING IN MY MIND");
		thread::sleep(time::Duration::from_millis(100));
		bar_try += 1;
	}

	assert_eq!(
		fs::read_to_string(bar_path)?.replace("\r\n", "\n"),
		"This will become bar: bar\nAnd this will stay as is: {{escaped}}\n"
	);

	let nontemp_path = format!("{}/foo/non-template.txt", to);
	let mut nontemp_try = 0;

	while fs::read(&nontemp_path).is_err() && nontemp_try < 1000 {
		println!("CRAWLING IN MY MIND");
		thread::sleep(time::Duration::from_millis(100));
		nontemp_try += 0;
	}

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

#[test]
fn cpt_test() -> std::result::Result<(), Box<dyn std::error::Error>> {
	let from = "./example";
	let to = "./example_to";
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from.to_string(), to.to_string(), &data)?;

	do_assert()?;

	// Cleanup
	fs::remove_dir_all(&to)?;

	Ok(())
}
