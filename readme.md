# cpt

[![Build Status](https://travis-ci.com/AlexAegis/cpt.svg?branch=master)](https://travis-ci.com/AlexAegis/cpt) [![Crates.io](https://img.shields.io/crates/v/cpt)](https://crates.io/crates/cpt) [![Docs.rs](https://docs.rs/mio/badge.svg)](https://docs.rs/cpt) [![Codacy Badge](https://api.codacy.com/project/badge/Grade/3091464ea5954b7b813b6a1152831a84)](https://www.codacy.com/manual/AlexAegis/cpt?utm_source=github.com&utm_medium=referral&utm_content=AlexAegis/cpt&utm_campaign=Badge_Grade) [![Coverage Status](https://coveralls.io/repos/github/AlexAegis/cpt/badge.svg?branch=master)](https://coveralls.io/github/AlexAegis/cpt?branch=master)

## Copy with Templates

Copies a folder structure and if templating data is supplied then all `.tpl` files will be converted using [Handlebars](https://github.com/wycats/handlebars.js/) and the `.tpl` file extension will then be stripped.

It does not write over existing files, unless the `-f` or `--force` flag is present.

Folder names also support Handlebars syntaxs, here every new line in the name means a different path calculated from there.

## Install

### As a library

```toml
[dependencies]
cpt = "0.3.0"
```

### As a command line tool

```bash
cargo install cpt
```

## Usage

### As a library

Using shorthands

```rust
use cpt::cpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let from = String::from("./templates/example");
	let to = String::from("./example_to");
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());

	cpt(from, to, data) // cp(from, to) to use without templating
}
```

Using the builder

```rust
use cpt::Cpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let from = String::from("./templates/example");
	let to = String::from("./example_to");
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());

	Cpt::new(from, to)
		.set_force(true)
		.set_dry(false)
		.set_data(data)
		.execute()
}
```

### As a command line tool

```bash
cpt ./example ./exampletest --json='{ \"foo\": \"bar\" }'
```

From the command line it can only accept jsons with a depth of 1.

In short, this is invalid: `--json='{ \"foo\": { \"inner\": \"bar\" } }'`

```bash
cpt --help

cpt 0.3.0
AlexAegis
Copies one folder structure to another place with files. Also formats templates!

USAGE:
    cpt.exe [FLAGS] [OPTIONS] <from> <to>

FLAGS:
    -d, --dry        If set, nothing will be written to the disk
    -f, --force      If set, files can be overwritten in the target folder
    -h, --help       Prints help information
    -q, --quiet      Tarpaulin
    -V, --version    Prints version information

OPTIONS:
    -j, --json <json>    JSON formatted templating data

ARGS:
    <from>    The folder that will be copied [default: .]
    <to>      The folder where the folder will be placed [default: ./target]
```

## Reason

I made this for an [Advent of Code](https://www.adventofcode.com) project scaffolder which you can find in my [AoC repo](https://github.com/AlexAegis/advent-of-code).

## Used libraries

-   [Handlebars](https://github.com/sunng87/handlebars-rust)
    > Templating engine
-   [Walkdir](https://github.com/BurntSushi/walkdir)
    > Recursive directory walker
-   [Clap](https://github.com/clap-rs/clap)
    > Command line arguments parser
-   [Serde](https://github.com/serde-rs/serde)
    > Serializer, deserializer. Here used for [JSON](http://www.json.org/) parsing
