mod args;

use crate::args::FromArgs;
use cpt::model::Cpt;

#[cfg_attr(tarpaulin, skip)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	Cpt::<String, String>::from_args(None)?.execute()
}
