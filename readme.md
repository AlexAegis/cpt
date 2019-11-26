# cpt

## Copy with Templates

Copies a folder structure and if templating data is supplied then all `.tpl` files will be converted using [Handlebar]() and the `.tpl` extension will then be strippet.

It does not write over files.

## Usage

### As a library

```rust
use cpt::cpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut data = std::collections::HashMap::<String, String>::new();
	data.insert("foo".to_string(), "bar".to_string());
	cpt(from, to, Some(&data))?;
	Ok(())
}
```

### As a command line tool:

```bash
cpt ./example ./exampletest --json='{ \"foo\": \"bar\" }'
```

```bash
cpt --help

Copies one folder structure to another place with files. Also formats templates!

USAGE:
    cpt.exe [OPTIONS] <from> <to>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --json <json>    JSON formatted templating data

ARGS:
    <from>    The folder that will be copied
    <to>      The folder where the folder will be placed
```

## Reason

I made this for an [Advent of Code](https://www.adventofcode.com) project scaffolder which you can find in my [AoC repo](https://github.com/AlexAegis/advent-of-code).

## Used libraries

-   [Handlebars](https://github.com/sunng87/handlebars-rust)
    > Templating
-   [Walkdir](https://github.com/BurntSushi/walkdir)
    > Iterating through folders
-   [Clap](https://github.com/clap-rs/clap)
    > Command line arguments
-   [Serde](https://github.com/serde-rs/serde)
    > Json parsing
