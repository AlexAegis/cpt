# cpt

[![Build Status](https://travis-ci.com/AlexAegis/cpt.svg?branch=master)](https://travis-ci.com/AlexAegis/cpt) [![Crates.io](https://img.shields.io/crates/v/cpt)](https://crates.io/crates/cpt) [![Docs.rs](https://docs.rs/mio/badge.svg)](https://docs.rs/cpt) [![Codacy Badge](https://api.codacy.com/project/badge/Grade/3091464ea5954b7b813b6a1152831a84)](https://www.codacy.com/manual/AlexAegis/cpt?utm_source=github.com&utm_medium=referral&utm_content=AlexAegis/cpt&utm_campaign=Badge_Grade) [![Coverage Status](https://coveralls.io/repos/github/AlexAegis/cpt/badge.svg?branch=master)](https://coveralls.io/github/AlexAegis/cpt?branch=master)

## Copy with Templates

Copies a folder structure and if templating data is supplied then all `.tpl` files will be converted using [Handlebars](https://github.com/wycats/handlebars.js/) and the `.tpl` file extension will then be stripped.

It does not write over existing files unless the `-f` or `--force` flag is present.

It can be run `dry` which will skip any file writes, but still logs what would it do. Use the `-d` or `--dry` flags.

Folder and file names also support Handlebars syntax. (Although you can't use `\` and many others in folder names so you are limited). After applying the template into the file/folder names, `\n` characters (since they invalid anyway) will be handled specially. At every line break the created folder structure branches off. The content of each of them will be identical.

#### For example, with this data:

> The second line will be serialized as "file1.txt.tpl\nfile2.txt.tpl"

```json
{
	"dir": "dir1\ndir2",
	"file": ["file1.txt.tpl", "file2.txt.tpl"]
}
```

from this folder

```bash
./
./bar.txt.tpl
./{{dir}}/{{file}}
./{{dir}}/non-template.txt
```

these output files and folders will be produced

```bash
./
./bar.txt
./dir1
./dir2
./dir1/non-template.txt
./dir2/non-template.txt
./dir1/file1.txt
./dir2/file1.txt
./dir1/file2.txt
./dir2/file2.txt
```

You can try this out with this command after downloading this repository (Given that you have [Rust and Cargo](https://www.rust-lang.org/) installed):

```bash
cargo run ./templates/example_tpl_dir ./templates/example_to --json='{ \"file2\": \"bar\nbare\", \"dir\": \"dir1\ndir2\", \"file\": [\"file1.txt.tpl\", \"file2.txt.tpl\"] }'
```

## Install

### As a library

```toml
[dependencies]
cpt = "0.4.1"
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

Using help:

```bash
cpt --help

cpt 0.4.1
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

## Valid input

The serializer only supports strings and arrays. A valid TypeScript type of the input would look like this:

```ts
interface Input {
	[key: string]: string | string[];
}
```

## Motivation

I made this for my [Advent of Code](https://www.adventofcode.com) project scaffolder which you can find in my [AoC repo](https://github.com/AlexAegis/advent-of-code).

## What's next?

For this to be a little more than just a tiny toy project the next step would be to implement context-aware templating. If we think of a template as a tree where the leaves are the contents of a file, and their parents are the names of their files, then it would be nice to pass some context to these nodes about their parents and their positions.

This would allow automatic indexing for example.

## Used libraries

-   [Handlebars](https://github.com/sunng87/handlebars-rust)
    > Templating engine
-   [Walkdir](https://github.com/BurntSushi/walkdir)
    > Recursive directory walker
-   [Clap](https://github.com/clap-rs/clap)
    > Command-line arguments parser
-   [Serde](https://github.com/serde-rs/serde)
    > Serializer, deserializer. Here used for [JSON](http://www.json.org/) parsing
