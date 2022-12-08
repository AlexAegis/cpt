mod args;

use crate::args::FromArgs;
use cpt::model::Cpt;

#[cfg_attr(tarpaulin, skip)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	Cpt::from_args(None)?.execute()
}
